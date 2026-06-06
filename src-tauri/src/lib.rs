mod tracker;
mod commands;

use std::sync::Arc;
use tokio::sync::Mutex;
use tracker::TrackerState;

pub fn run() {
    tracing_subscriber::fmt().init();

    let state = Arc::new(Mutex::new(TrackerState::new()));
    let tracker_handle = state.clone();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(state)
        .setup(move |_app| {
            let handle = tracker_handle.clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = tracker::run_loop(handle).await {
                    tracing::error!("tracker loop crashed: {e}");
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_today_summary,
            commands::set_tracking,
        ])
        .run(tauri::generate_context!())
        .expect("error while running TimeAtlas");
}
