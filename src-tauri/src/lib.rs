mod commands;
mod settings;
mod tracker;

use std::sync::Arc;
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::TrayIconBuilder;
use tauri::{Manager, WindowEvent};
use tokio::sync::Mutex;
use tracker::TrackerState;

const TRAY_SHOW: &str = "show";
const TRAY_PAUSE: &str = "pause";
const TRAY_RESUME: &str = "resume";
const TRAY_QUIT: &str = "quit";

pub fn run() {
    tracing_subscriber::fmt().init();

    let state = Arc::new(Mutex::new(TrackerState::new()));
    let tracker_handle = state.clone();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(state)
        .setup(move |app| {
            // Build the tray menu. We keep both "Pause" and "Resume" items in the
            // menu and toggle their `enabled` state instead of mutating the menu
            // contents — same UX on every platform with less code.
            let show_item = MenuItem::with_id(app, TRAY_SHOW, "Show TimeAtlas", true, None::<&str>)?;
            let pause_item =
                MenuItem::with_id(app, TRAY_PAUSE, "Pause tracking", true, None::<&str>)?;
            let resume_item =
                MenuItem::with_id(app, TRAY_RESUME, "Resume tracking", false, None::<&str>)?;
            let sep = PredefinedMenuItem::separator(app)?;
            let quit_item = MenuItem::with_id(app, TRAY_QUIT, "Quit", true, None::<&str>)?;

            let menu = Menu::with_items(
                app,
                &[&show_item, &pause_item, &resume_item, &sep, &quit_item],
            )?;

            // Clones captured by the event closure so we can flip the items'
            // enabled state when tracking is paused/resumed from the tray.
            let pause_handle = pause_item.clone();
            let resume_handle = resume_item.clone();

            let _tray = TrayIconBuilder::with_id("timeatlas-tray")
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("TimeAtlas")
                .menu(&menu)
                .on_menu_event(move |app, event| match event.id.as_ref() {
                    TRAY_SHOW => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    TRAY_PAUSE => {
                        apply_tracking_change(app, false);
                        let _ = pause_handle.set_enabled(false);
                        let _ = resume_handle.set_enabled(true);
                    }
                    TRAY_RESUME => {
                        apply_tracking_change(app, true);
                        let _ = pause_handle.set_enabled(true);
                        let _ = resume_handle.set_enabled(false);
                    }
                    TRAY_QUIT => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .build(app)?;

            // Spawn the tracker loop.
            let tracker = tracker_handle.clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = tracker::run_loop(tracker).await {
                    tracing::error!("tracker loop crashed: {e}");
                }
            });

            // Close-to-tray: when the user clicks the window's X, hide the
            // window instead of exiting. They can quit explicitly via the tray.
            if let Some(window) = app.get_webview_window("main") {
                let window_handle = window.clone();
                window.on_window_event(move |event| {
                    if let WindowEvent::CloseRequested { api, .. } = event {
                        let _ = window_handle.hide();
                        api.prevent_close();
                    }
                });
            }

            // Sync the tray's Pause/Resume enabled state with persisted settings
            // once those have been loaded by the tracker.
            let app_handle = app.handle().clone();
            let init_pause = pause_item.clone();
            let init_resume = resume_item.clone();
            tauri::async_runtime::spawn(async move {
                // Small delay to let the tracker load settings from disk.
                tokio::time::sleep(std::time::Duration::from_millis(150)).await;
                if let Some(state) = app_handle.try_state::<Arc<Mutex<TrackerState>>>() {
                    let enabled = state.lock().await.settings.tracking_enabled;
                    let _ = init_pause.set_enabled(enabled);
                    let _ = init_resume.set_enabled(!enabled);
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_today_summary,
            commands::get_today_timeline,
            commands::get_settings,
            commands::update_settings,
            commands::set_tracking,
        ])
        .run(tauri::generate_context!())
        .expect("error while running TimeAtlas");
}

/// Flip tracking on/off from the tray menu and persist the new value.
fn apply_tracking_change(app: &tauri::AppHandle, enabled: bool) {
    let app = app.clone();
    tauri::async_runtime::spawn(async move {
        if let Some(state) = app.try_state::<Arc<Mutex<TrackerState>>>() {
            let storage_and_settings = {
                let mut s = state.lock().await;
                s.settings.tracking_enabled = enabled;
                s.storage().cloned().map(|st| (st, s.settings.clone()))
            };
            if let Some((storage, settings)) = storage_and_settings {
                if let Err(e) = storage.save_settings(&settings).await {
                    tracing::warn!("save settings from tray failed: {e}");
                }
            }
        }
    });
}
