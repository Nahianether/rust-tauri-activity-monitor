/// Idle detection — returns true if no input activity for longer than the
/// configured threshold. Currently a stub; v0.1 will wire up platform-specific
/// `GetLastInputInfo` (Windows), `CGEventSourceSecondsSinceLastEventType`
/// (macOS), and X11/Wayland equivalents.
pub fn is_idle() -> bool {
    false
}
