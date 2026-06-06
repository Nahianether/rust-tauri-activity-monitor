use anyhow::{Context, Result};
use chrono::{Local, NaiveDate};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::path::PathBuf;

use super::ActivityEntry;

#[derive(Clone)]
pub struct Storage {
    pool: SqlitePool,
}

impl Storage {
    pub async fn new() -> Result<Self> {
        let path = data_dir()?;
        std::fs::create_dir_all(&path).context("creating data dir")?;
        let db_file = path.join("db.sqlite");
        let url = format!("sqlite://{}?mode=rwc", db_file.display());

        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(&url)
            .await
            .context("connecting sqlite")?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS activity (
                day          TEXT    NOT NULL,
                app_name     TEXT    NOT NULL,
                window_title TEXT    NOT NULL,
                duration_sec INTEGER NOT NULL DEFAULT 0,
                PRIMARY KEY (day, app_name, window_title)
            )
            "#,
        )
        .execute(&pool)
        .await?;

        Ok(Self { pool })
    }

    pub async fn increment(&self, app: &str, title: &str, seconds: i64) -> Result<()> {
        let day = today_str();
        sqlx::query(
            r#"
            INSERT INTO activity (day, app_name, window_title, duration_sec)
            VALUES (?1, ?2, ?3, ?4)
            ON CONFLICT (day, app_name, window_title)
            DO UPDATE SET duration_sec = duration_sec + excluded.duration_sec
            "#,
        )
        .bind(day)
        .bind(app)
        .bind(title)
        .bind(seconds)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn today_summary(&self) -> Result<Vec<ActivityEntry>> {
        let day = today_str();
        let rows: Vec<(String, String, i64)> = sqlx::query_as(
            r#"
            SELECT app_name, window_title, duration_sec
            FROM activity
            WHERE day = ?1
            ORDER BY duration_sec DESC
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
}

fn today_str() -> String {
    let d: NaiveDate = Local::now().date_naive();
    d.format("%Y-%m-%d").to_string()
}

fn data_dir() -> Result<PathBuf> {
    let base = dirs::home_dir().context("home dir not found")?;
    Ok(base.join(".timeatlas"))
}
