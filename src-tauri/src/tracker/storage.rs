use anyhow::{Context, Result};
use chrono::{Local, NaiveDate, Timelike};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::path::PathBuf;

use crate::settings::Settings;

/// One row of today's per-app summary, used by the daily summary view.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityEntry {
    pub app_name: String,
    pub window_title: String,
    pub duration_seconds: i64,
}

/// One cell of the timeline grid: a 5-minute bucket × app, with how many
/// seconds were spent in that app during that bucket.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineSegment {
    pub bucket_minute: i64,
    pub app_name: String,
    pub duration_seconds: i64,
}

/// Width of one timeline bucket. 5 minutes is the sweet spot:
/// 288 buckets/day fit comfortably across a desktop window.
pub const BUCKET_MINUTES: i64 = 5;

#[derive(Clone)]
pub struct Storage {
    pool: SqlitePool,
}

impl Storage {
    /// Open (or create) the on-disk database in `~/.timeatlas/db.sqlite`.
    pub async fn new() -> Result<Self> {
        let path = data_dir()?;
        std::fs::create_dir_all(&path).context("creating data dir")?;
        let db_file = path.join("db.sqlite");
        let url = format!("sqlite://{}?mode=rwc", db_file.display());
        Self::open(&url).await
    }

    /// Lower-level constructor used by both `new()` and unit tests.
    pub async fn open(url: &str) -> Result<Self> {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(url)
            .await
            .context("connecting sqlite")?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS events (
                day            TEXT    NOT NULL,
                bucket_minute  INTEGER NOT NULL,
                app_name       TEXT    NOT NULL,
                window_title   TEXT    NOT NULL DEFAULT '',
                duration_sec   INTEGER NOT NULL DEFAULT 0,
                PRIMARY KEY (day, bucket_minute, app_name, window_title)
            )
            "#,
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS settings (
                id   INTEGER PRIMARY KEY CHECK (id = 1),
                data TEXT    NOT NULL
            )
            "#,
        )
        .execute(&pool)
        .await?;

        // Drop the legacy v0.1 `activity` table if it exists. Pre-release software
        // doesn't need to preserve old data.
        sqlx::query("DROP TABLE IF EXISTS activity")
            .execute(&pool)
            .await?;

        Ok(Self { pool })
    }

    /// Record `seconds` of activity for the given app + window title at the
    /// current 5-minute bucket of the current day.
    pub async fn increment(&self, app: &str, title: &str, seconds: i64) -> Result<()> {
        let now = Local::now();
        let day = now.date_naive().format("%Y-%m-%d").to_string();
        let bucket = (now.hour() as i64) * 60 + (now.minute() as i64);
        let bucket = (bucket / BUCKET_MINUTES) * BUCKET_MINUTES;

        sqlx::query(
            r#"
            INSERT INTO events (day, bucket_minute, app_name, window_title, duration_sec)
            VALUES (?1, ?2, ?3, ?4, ?5)
            ON CONFLICT (day, bucket_minute, app_name, window_title)
            DO UPDATE SET duration_sec = duration_sec + excluded.duration_sec
            "#,
        )
        .bind(day)
        .bind(bucket)
        .bind(app)
        .bind(title)
        .bind(seconds)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Today's per-app totals, ordered by time spent.
    pub async fn today_summary(&self) -> Result<Vec<ActivityEntry>> {
        let day = today_str();
        let rows: Vec<(String, String, i64)> = sqlx::query_as(
            r#"
            SELECT app_name,
                   COALESCE(MAX(window_title), '') AS window_title,
                   SUM(duration_sec) AS total
            FROM events
            WHERE day = ?1
            GROUP BY app_name
            ORDER BY total DESC
            LIMIT 50
            "#,
        )
        .bind(day)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|(app_name, window_title, duration_seconds)| ActivityEntry {
                app_name,
                window_title,
                duration_seconds,
            })
            .collect())
    }

    /// Today's timeline: one row per (5-minute bucket, app). Frontend renders
    /// this as a horizontal stacked bar across the day.
    pub async fn today_timeline(&self) -> Result<Vec<TimelineSegment>> {
        let day = today_str();
        let rows: Vec<(i64, String, i64)> = sqlx::query_as(
            r#"
            SELECT bucket_minute,
                   app_name,
                   SUM(duration_sec) AS total
            FROM events
            WHERE day = ?1
            GROUP BY bucket_minute, app_name
            ORDER BY bucket_minute ASC, total DESC
            "#,
        )
        .bind(day)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(
                |(bucket_minute, app_name, duration_seconds)| TimelineSegment {
                    bucket_minute,
                    app_name,
                    duration_seconds,
                },
            )
            .collect())
    }

    /// Load persisted settings, or `Settings::default()` if none have been saved.
    pub async fn load_settings(&self) -> Result<Settings> {
        let row: Option<(String,)> = sqlx::query_as("SELECT data FROM settings WHERE id = 1")
            .fetch_optional(&self.pool)
            .await?;
        match row {
            Some((data,)) => Ok(serde_json::from_str(&data).unwrap_or_default()),
            None => Ok(Settings::default()),
        }
    }

    /// Persist settings (upsert into the single-row settings table).
    pub async fn save_settings(&self, settings: &Settings) -> Result<()> {
        let data = serde_json::to_string(settings)?;
        sqlx::query(
            r#"
            INSERT INTO settings (id, data) VALUES (1, ?1)
            ON CONFLICT (id) DO UPDATE SET data = excluded.data
            "#,
        )
        .bind(data)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

fn today_str() -> String {
    let d: NaiveDate = Local::now().date_naive();
    d.format("%Y-%m-%d").to_string()
}

fn data_dir() -> Result<PathBuf> {
    let base = dirs::home_dir().context("home dir not found")?;
    Ok(base.join(".timeatlas"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::Theme;

    async fn fresh_storage() -> Storage {
        Storage::open("sqlite::memory:")
            .await
            .expect("open in-memory")
    }

    #[tokio::test]
    async fn increment_creates_and_accumulates() {
        let s = fresh_storage().await;
        s.increment("Code", "main.rs", 5).await.unwrap();
        s.increment("Code", "main.rs", 3).await.unwrap();
        let summary = s.today_summary().await.unwrap();
        assert_eq!(summary.len(), 1);
        assert_eq!(summary[0].app_name, "Code");
        assert_eq!(summary[0].duration_seconds, 8);
    }

    #[tokio::test]
    async fn summary_groups_by_app_across_window_titles() {
        let s = fresh_storage().await;
        s.increment("Chrome", "github.com", 10).await.unwrap();
        s.increment("Chrome", "stackoverflow.com", 7).await.unwrap();
        s.increment("Code", "main.rs", 4).await.unwrap();
        let summary = s.today_summary().await.unwrap();
        assert_eq!(summary.len(), 2);
        assert_eq!(summary[0].app_name, "Chrome");
        assert_eq!(summary[0].duration_seconds, 17);
        assert_eq!(summary[1].app_name, "Code");
        assert_eq!(summary[1].duration_seconds, 4);
    }

    #[tokio::test]
    async fn timeline_aggregates_by_bucket_and_app() {
        let s = fresh_storage().await;
        s.increment("Code", "main.rs", 30).await.unwrap();
        s.increment("Chrome", "github.com", 20).await.unwrap();
        let timeline = s.today_timeline().await.unwrap();
        assert!(!timeline.is_empty());
        // All events from this single test run land in one bucket. Sum-per-app should match.
        let total_for_code: i64 = timeline
            .iter()
            .filter(|t| t.app_name == "Code")
            .map(|t| t.duration_seconds)
            .sum();
        assert_eq!(total_for_code, 30);
    }

    #[tokio::test]
    async fn settings_default_when_unset() {
        let s = fresh_storage().await;
        let settings = s.load_settings().await.unwrap();
        assert_eq!(settings.idle_threshold_seconds, 180);
        assert!(settings.tracking_enabled);
        assert_eq!(settings.theme, Theme::Auto);
    }

    #[tokio::test]
    async fn settings_round_trip() {
        let s = fresh_storage().await;
        let settings = Settings {
            idle_threshold_seconds: 600,
            theme: Theme::Dark,
            tracking_enabled: false,
        };
        s.save_settings(&settings).await.unwrap();

        let loaded = s.load_settings().await.unwrap();
        assert_eq!(loaded.idle_threshold_seconds, 600);
        assert_eq!(loaded.theme, Theme::Dark);
        assert!(!loaded.tracking_enabled);
    }

    #[tokio::test]
    async fn bucket_alignment_snaps_to_5_minutes() {
        // Sanity-check the bucket math the increment() function uses.
        // Wrap in a fn to keep clippy from const-folding individual cases away.
        fn snap(m: i64) -> i64 {
            (m / BUCKET_MINUTES) * BUCKET_MINUTES
        }
        assert_eq!(snap(0), 0);
        assert_eq!(snap(4), 0);
        assert_eq!(snap(5), 5);
        assert_eq!(snap(9), 5);
        assert_eq!(snap(63), 60);
        assert_eq!(snap(1435), 1435);
    }
}
