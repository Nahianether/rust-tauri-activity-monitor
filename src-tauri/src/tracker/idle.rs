use user_idle::UserIdle;

/// Seconds since the last keyboard or mouse input, cross-platform.
/// Returns `None` if the platform API is unavailable (e.g. some Wayland sessions).
pub fn seconds_since_input() -> Option<u64> {
    UserIdle::get_time().ok().map(|idle| idle.as_seconds())
}

/// True if no input has been observed for at least `threshold_seconds`.
pub fn is_idle(threshold_seconds: u64) -> bool {
    seconds_since_input().is_some_and(|s| s >= threshold_seconds)
}
