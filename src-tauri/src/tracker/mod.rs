mod active_window;
mod idle;
mod storage;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::interval;

use storage::Storage;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityEntry {
    pub app_name: String,
    pub window_title: String,
    pub duration_seconds: i64,
}

pub struct TrackerState {
    pub enabled: bool,
    storage: Option<Storage>,
}

impl TrackerState {
    pub fn new() -> Self {
        Self { enabled: true, storage: None }
    }

    pub async fn today_summary(&self) -> Result<Vec<ActivityEntry>> {
        match &self.storage {
            Some(s) => s.today_summary().await,
            None => Ok(vec![]),
        }
    }
}

/// Main tracker loop — polls active window once per second and persists
/// elapsed time per (app, window_title) bucket.
pub async fn run_loop(state: Arc<Mutex<TrackerState>>) -> Result<()> {
    let storage = Storage::new().await?;
    {
        let mut s = state.lock().await;
        s.storage = Some(storage.clone());
    }

    let mut ticker = interval(Duration::from_secs(1));

    loop {
        ticker.tick().await;

        let enabled = state.lock().await.enabled;
        if !enabled || idle::is_idle() {
            continue;
        }

        let Some(win) = active_window::current() else {
            continue;
        };

        if let Err(e) = storage.increment(&win.app_name, &win.window_title, 1).await {
            tracing::warn!("storage write failed: {e}");
        }
    }
}
