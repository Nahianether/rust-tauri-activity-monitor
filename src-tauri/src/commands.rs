use std::sync::Arc;
use tokio::sync::Mutex;

use crate::settings::Settings;
use crate::tracker::{ActivityEntry, TimelineSegment, TrackerState};

type State<'a> = tauri::State<'a, Arc<Mutex<TrackerState>>>;

/// Today's per-app totals, sorted descending by time spent. Drives the
/// "Summary" tab of the UI.
#[tauri::command]
pub async fn get_today_summary(state: State<'_>) -> Result<Vec<ActivityEntry>, String> {
    let guard = state.lock().await;
    let Some(storage) = guard.storage() else {
        return Ok(vec![]);
    };
    storage.today_summary().await.map_err(|e| e.to_string())
}

/// Today's timeline (5-minute buckets × app). Drives the "Timeline" tab.
#[tauri::command]
pub async fn get_today_timeline(state: State<'_>) -> Result<Vec<TimelineSegment>, String> {
    let guard = state.lock().await;
    let Some(storage) = guard.storage() else {
        return Ok(vec![]);
    };
    storage.today_timeline().await.map_err(|e| e.to_string())
}

/// Read current persisted settings.
#[tauri::command]
pub async fn get_settings(state: State<'_>) -> Result<Settings, String> {
    let guard = state.lock().await;
    Ok(guard.settings.clone())
}

/// Persist a full settings replacement and update the live tracker state in
/// one shot, so the next tick of the tracker loop sees the new values.
#[tauri::command]
pub async fn update_settings(new_settings: Settings, state: State<'_>) -> Result<Settings, String> {
    let mut guard = state.lock().await;
    if let Some(storage) = guard.storage() {
        storage
            .save_settings(&new_settings)
            .await
            .map_err(|e| e.to_string())?;
    }
    guard.settings = new_settings.clone();
    Ok(new_settings)
}

/// Shortcut for the most common settings change: toggle tracking on/off.
/// Used by both the UI pause button and the system-tray menu.
#[tauri::command]
pub async fn set_tracking(enabled: bool, state: State<'_>) -> Result<(), String> {
    let mut guard = state.lock().await;
    guard.settings.tracking_enabled = enabled;
    if let Some(storage) = guard.storage() {
        let snapshot = guard.settings.clone();
        storage
            .save_settings(&snapshot)
            .await
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}
