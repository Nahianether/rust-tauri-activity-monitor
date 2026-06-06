use active_win_pos_rs::get_active_window;

#[derive(Debug, Clone)]
pub struct WindowSnapshot {
    pub app_name: String,
    pub window_title: String,
}

pub fn current() -> Option<WindowSnapshot> {
    match get_active_window() {
        Ok(w) => Some(WindowSnapshot {
            app_name: w.app_name,
            window_title: w.title,
        }),
        Err(_) => None,
    }
}
