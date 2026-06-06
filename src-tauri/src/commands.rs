use std::sync::Arc;
use tokio::sync::Mutex;
use crate::tracker::{TrackerState, ActivityEntry};

#[tauri::command]
pub async fn get_today_summary(
    state: tauri::State<'_, Arc<Mutex<TrackerState>>>,
) -> Result<Vec<ActivityEntry>, String> {
    let state = state.lock().await;
    state.today_summary().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_tracking(
    enabled: bool,
    state: tauri::State<'_, Arc<Mutex<TrackerState>>>,
) -> Result<(), String> {
    let mut state = state.lock().await;
    state.enabled = enabled;
    Ok(())
}
