use serde::{Deserialize, Serialize};

/// User-configurable settings. Persisted as a single JSON blob in the `settings` table
/// (one row, id=1). Loaded once on startup into the tracker state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// How many seconds of no input before we mark the user as idle and skip tracking.
    pub idle_threshold_seconds: u64,

    /// UI theme preference. Auto follows the OS.
    pub theme: Theme,

    /// Master switch. When false the tracker loop is a no-op.
    pub tracking_enabled: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Auto,
    Light,
    Dark,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            idle_threshold_seconds: 180,
            theme: Theme::Auto,
            tracking_enabled: true,
        }
    }
}
