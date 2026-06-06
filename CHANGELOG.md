# Changelog

All notable changes to TimeAtlas are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] — 2026-06-06

### Added

- **Real cross-platform idle detection** via the `user-idle` crate (Windows
  `GetLastInputInfo`, macOS `CGEventSourceSecondsSinceLastEventType`, Linux
  X11 `XScreenSaver`). Replaces the v0.1 stub that always returned false.
- **System tray icon** with menu items: *Show TimeAtlas*, *Pause tracking*,
  *Resume tracking*, *Quit*. Pause / Resume reflect live state.
- **Close-to-tray**: closing the window now hides it instead of exiting, so
  tracking keeps running while the window is dismissed.
- **Persisted settings** — idle threshold (default 180s), theme preference
  (Auto / Light / Dark), tracking enabled — stored as JSON in a single-row
  SQLite `settings` table.
- **Tabbed UI shell**: `Timeline` / `Summary` / `Settings` tabs.
- **Timeline tab** — SVG horizontal 24-hour bar; each 5-minute bucket is a
  column, sub-segmented per app when multiple apps were active within the
  bucket. Hour gridlines and labels every three hours. Hover tooltips show
  app name and exact duration.
- **Summary tab** — daily per-app totals with deterministic per-app color,
  percentage bar fill, and formatted duration.
- **Settings tab** — chip-style idle threshold picker (1 / 3 / 5 / 10 min),
  theme switch (Auto / Light / Dark), tracking toggle, dirty-state save
  button with explicit confirmation text.
- **Light & dark themes** with a CSS-variable token system. *Auto* follows
  the OS `prefers-color-scheme` and reacts to changes without a restart.
- **Deterministic per-app colors** — golden-ratio hash of the app name maps
  to an HSL hue. Same app gets the same color across every view.
- **Tracking-status header** with an animated pulse dot (green when tracking,
  grey when paused) and an inline pause/resume button.
- **Typed IPC layer** (`src/ipc.ts`) wrapping every `invoke()` call.
- **Storage unit tests** (6 cases) — increment accumulation, summary grouping
  across window titles, timeline bucket aggregation, settings default and
  round-trip, bucket alignment.

### Changed

- **Storage schema refactor**: the `activity` table is replaced by `events`,
  which adds a `bucket_minute` column so per-bucket aggregation is a single
  `GROUP BY` rather than recomputing on the fly. The legacy `activity`
  table is dropped on first run; v0.1 data is not migrated (pre-release
  versions don't need to preserve data).
- The tracker loop now reads idle threshold and tracking-enabled state from
  persisted settings on startup, instead of being hardcoded.
- Tauri commands now use a typed `tauri::State<Arc<Mutex<TrackerState>>>`
  alias for consistency.

### Removed

- The `last_seen` debug variable in the tracker loop (it was always
  overwritten before being read).

## [0.1.0] — 2026-06-06

Initial scaffold release. Compiles cleanly, includes:

- Tauri 2 + React + TypeScript shell
- 1 Hz active-window tracker loop in Rust
- Local SQLite storage in `~/.timeatlas/db.sqlite`
- Bare-bones React UI showing today's activity list
- MIT license, README, cross-platform icon placeholders

[0.2.0]: https://github.com/Nahianether/rust-tauri-activity-monitor/releases/tag/v0.2.0
[0.1.0]: https://github.com/Nahianether/rust-tauri-activity-monitor/releases/tag/v0.1.0
