mod active_window;
mod idle;
mod storage;

use anyhow::Result;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::interval;

pub use storage::{ActivityEntry, Storage, TimelineSegment};

use crate::settings::Settings;

pub struct TrackerState {
    pub settings: Settings,
    storage: Option<Storage>,
}

impl TrackerState {
    pub fn new() -> Self {
        Self {
            settings: Settings::default(),
            storage: None,
        }
    }

    pub fn storage(&self) -> Option<&Storage> {
        self.storage.as_ref()
    }
}

/// Main tracker loop. Polls the active window once per second and persists
/// elapsed time into the 5-minute bucket that contains the current moment.
///
/// Skips writes when:
///   - `settings.tracking_enabled == false` (user-paused)
///   - the user is idle (no input for `settings.idle_threshold_seconds`)
///   - no foreground window can be identified
pub async fn run_loop(state: Arc<Mutex<TrackerState>>) -> Result<()> {
    let storage = Storage::new().await?;
    {
        // Stash the storage handle and replace defaults with persisted settings.
        let mut s = state.lock().await;
        s.storage = Some(storage.clone());
        if let Ok(loaded) = storage.load_settings().await {
            s.settings = loaded;
        }
    }

    let mut ticker = interval(Duration::from_secs(1));

    loop {
        ticker.tick().await;

        let (enabled, idle_threshold) = {
            let s = state.lock().await;
            (s.settings.tracking_enabled, s.settings.idle_threshold_seconds)
        };

        if !enabled || idle::is_idle(idle_threshold) {
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
