# TimeAtlas

> A **personal**, **local-only** time-awareness desktop app for developers and knowledge workers.
> Built with Tauri 2 + Rust + React.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE)
[![Rust](https://img.shields.io/badge/rust-2021-orange.svg?logo=rust)](https://www.rust-lang.org/)
[![Tauri](https://img.shields.io/badge/tauri-2.x-FFC131.svg?logo=tauri&logoColor=black)](https://tauri.app/)
[![Version](https://img.shields.io/badge/version-0.2.0-success.svg)](./CHANGELOG.md)

TimeAtlas quietly tracks which apps and windows you spend time in throughout the day, then shows you a clean timeline and daily summary so you can understand where your hours actually go.

It is intentionally **personal**, **not** a team monitoring tool:

- 🚫 No screenshot capture
- 🚫 No keystroke logging
- 🚫 No cloud sync — all data lives on your machine
- 🚫 No manager / admin dashboards
- 🚫 No productivity scoring or judgment

Inspired by [ActivityWatch](https://activitywatch.net/), reimagined with a modern Tauri 2 + React stack and a focus on a single user's awareness — not workplace surveillance.

---

## Why I built this

I wanted to know where my own time goes during a coding day without handing telemetry to a SaaS, and without the heavy footprint of Electron-based tools. Tauri 2 + Rust gives a single ~10 MB binary that quietly observes the active window, writes to a local SQLite file, and never phones home.

---

## What you'll see

TimeAtlas has three tabs. Here's what each one looks like in terminal-art form — the real UI is a polished React app with light/dark themes:

### `Timeline` — your day as a 24-hour horizontal bar

```
┌─────────────────────────────────────────────────────────────────────┐
│  ⏱  TimeAtlas                          ●Tracking   [Pause]          │
├─────────────────────────────────────────────────────────────────────┤
│  ▸Timeline   Summary   Settings                                     │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│                  ▄▄         ▄▄▄▄▄▄▄          ▄▄▄▄                   │
│  ▄▄▄        ▄▄▄▄▄██▄▄  ▄▄▄  ███████▄▄  ▄▄▄▄  █████  ▄▄▄    ▄▄▄      │
│  ████  ▄▄▄  █████████  ███  ████████▄  ████  █████  ███▄  ▄███▄     │
│  ██████████████████████████████████████████████████████████████     │
│                                                                     │
│  00:00     03:00     06:00     09:00     12:00     15:00     18:00  │
└─────────────────────────────────────────────────────────────────────┘
```

Each 5-minute bucket is one vertical column. When multiple apps were active in the same bucket, the column is sub-segmented in proportion to time spent. Hovering any segment shows the exact `app — duration` tooltip.

### `Summary` — today's per-app totals

```
┌─────────────────────────────────────────────────────────────────────┐
│  Today, 5 apps                                              4h 32m  │
├─────────────────────────────────────────────────────────────────────┤
│  ● Code            ████████████████████████░░░░░░░  2h 14m   49%   │
│  ● Chrome          ███████████░░░░░░░░░░░░░░░░░░░░    58m    21%   │
│  ● Slack           █████░░░░░░░░░░░░░░░░░░░░░░░░░░    32m    12%   │
│  ● Spotify         ████░░░░░░░░░░░░░░░░░░░░░░░░░░░    24m     9%   │
│  ● Terminal        ███░░░░░░░░░░░░░░░░░░░░░░░░░░░░    24m     9%   │
└─────────────────────────────────────────────────────────────────────┘
```

Each app gets a deterministic color (golden-ratio hue hash of the app name), so the same app is always the same color across both the timeline and the summary.

### `Settings` — three knobs, no more

```
┌─────────────────────────────────────────────────────────────────────┐
│  Idle threshold   ( 1 min )  ( 3 min ✓ ) ( 5 min ) ( 10 min )       │
│                                                                     │
│  Theme            ( Auto ✓ ) ( Light )   ( Dark )                   │
│                                                                     │
│  Tracking         [✓] Enabled                                       │
│                                                                     │
│  [ Save changes ]                                                   │
└─────────────────────────────────────────────────────────────────────┘
```

Settings are stored in `~/.timeatlas/db.sqlite` alongside your activity data. TimeAtlas makes zero outbound network requests.

---

## Features

**v0.2** delivers a complete personal time-tracker:

- ✅ **Active-window tracking** — 1 Hz polling, cross-platform via [`active-win-pos-rs`](https://crates.io/crates/active-win-pos-rs)
- ✅ **Real idle detection** — Windows `GetLastInputInfo`, macOS `CGEventSourceSecondsSinceLastEventType`, Linux X11 `XScreenSaver`, all via [`user-idle`](https://crates.io/crates/user-idle)
- ✅ **5-minute bucket aggregation** — events stored compactly; queries are a single `GROUP BY`
- ✅ **SVG timeline view** — 24-hour horizontal bar with sub-segmented buckets and hover tooltips
- ✅ **Daily summary view** — totals per app with percentage bars and per-app colors
- ✅ **System tray** — Show / Pause / Resume / Quit menu items, live state reflected in menu
- ✅ **Close-to-tray** — closing the window hides it; tracking keeps running
- ✅ **Light & dark themes** — Auto follows OS `prefers-color-scheme`, switches without restart
- ✅ **Persisted settings** — idle threshold, theme, tracking-enabled toggle
- ✅ **6 storage unit tests** — increment aggregation, summary grouping, timeline bucketing, settings round-trip
- ✅ **Clippy-clean, rustfmt-checked** — see [`CONTRIBUTING.md`](./CONTRIBUTING.md) for the pre-PR check list

---

## Quick start

### Prerequisites

- [Rust](https://rustup.rs/) ≥ 1.75
- [Node.js](https://nodejs.org/) ≥ 18
- Tauri 2 platform prerequisites — see [tauri.app/start/prerequisites](https://tauri.app/start/prerequisites/)
  - **Windows**: WebView2 (usually pre-installed) + MSVC build tools
  - **macOS**: Xcode Command Line Tools
  - **Linux**: `libwebkit2gtk-4.1-dev libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev`

### Run in development

```bash
git clone https://github.com/Nahianether/rust-tauri-activity-monitor
cd rust-tauri-activity-monitor
npm install
npm run tauri dev
```

The dev server takes ~30–60 seconds on first compile, then reloads instantly via Vite HMR on every frontend change.

### Build a release binary

```bash
npm run tauri build
```

Outputs platform-native installers (and a single binary) under `src-tauri/target/release/`.

### Run tests

```bash
cd src-tauri
cargo test --lib
```

---

## Architecture

TimeAtlas is split into a Rust backend (tracker loop, persistence, system tray) and a React frontend (three tabbed views).

```
┌─────────────────────────────────────────────────────────────┐
│                    React + TypeScript                       │
│                                                             │
│   ┌──────────┐  ┌──────────┐  ┌──────────┐                  │
│   │ Timeline │  │ Summary  │  │ Settings │                  │
│   │ (SVG)    │  │ (list)   │  │ (form)   │                  │
│   └──────────┘  └──────────┘  └──────────┘                  │
│                                                             │
│   theme.ts (resolved + per-app colors)                      │
│   ipc.ts   (typed wrappers around invoke())                 │
└────────────────────────┬────────────────────────────────────┘
                         │   Tauri IPC commands
                         │   get_today_summary
                         │   get_today_timeline
                         │   get_settings
                         │   update_settings
                         │   set_tracking
┌────────────────────────▼────────────────────────────────────┐
│                       Rust backend                          │
│                                                             │
│   ┌─────────────────┐         ┌────────────────────────┐    │
│   │  tracker loop   │ ──────▶ │  storage.rs (SQLite)   │    │
│   │  (1 Hz polling) │         │   events (bucketed)    │    │
│   │  active_window  │         │   settings (JSON blob) │    │
│   │  idle detection │         └────────────────────────┘    │
│   └─────────────────┘                                       │
│                                                             │
│   ┌─────────────────┐         ┌────────────────────────┐    │
│   │  system tray    │         │  settings.rs           │    │
│   │  Show / Pause   │         │  (Settings, Theme)     │    │
│   │  Resume / Quit  │         └────────────────────────┘    │
│   └─────────────────┘                                       │
└─────────────────────────────────────────────────────────────┘
                         │
                  ┌──────▼──────────────┐
                  │ ~/.timeatlas/       │
                  │   db.sqlite         │
                  └─────────────────────┘
```

### Data model

Activity is stored as a flat events table with one row per `(day, 5-min bucket, app, window title)` combination:

```sql
CREATE TABLE events (
    day            TEXT    NOT NULL,
    bucket_minute  INTEGER NOT NULL,  -- 0, 5, 10, …, 1435
    app_name       TEXT    NOT NULL,
    window_title   TEXT    NOT NULL DEFAULT '',
    duration_sec   INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (day, bucket_minute, app_name, window_title)
);
```

Every tick of the tracker loop does a single `INSERT … ON CONFLICT DO UPDATE` to increment the count for the current bucket. Both the summary and timeline views are pure `GROUP BY` queries over this same table — no separate denormalized tables to keep in sync.

Settings ride in a single-row table:

```sql
CREATE TABLE settings (
    id   INTEGER PRIMARY KEY CHECK (id = 1),
    data TEXT    NOT NULL  -- JSON blob
);
```

---

## Tech stack

| Layer | Choice | Why |
|---|---|---|
| Desktop runtime | Tauri 2.x | Single ~10 MB binary, system webview, native menu/tray APIs |
| Backend language | Rust 2021 | Memory safety + speed for a long-running tracker loop |
| Async runtime | Tokio | Standard for Rust async; sqlx + Tauri both build on it |
| Storage | SQLite via [`sqlx`](https://crates.io/crates/sqlx) | Embedded, zero-config, fast `GROUP BY` for aggregations |
| Active window | [`active-win-pos-rs`](https://crates.io/crates/active-win-pos-rs) | Cross-platform foreground-window detection |
| Idle detection | [`user-idle`](https://crates.io/crates/user-idle) | Wraps platform APIs for input-idle time |
| Frontend | React 18 + TypeScript (strict) | Familiar, typed, plays nicely with Vite |
| Build | Vite 5 | Fast HMR; bundles to a static dist tauri serves |
| Styling | Vanilla CSS with custom-property tokens | No CSS framework needed; light/dark via `data-theme` |

---

## Privacy guarantees

These are non-negotiable design constraints — see [`CONTRIBUTING.md`](./CONTRIBUTING.md):

1. **No outbound network requests.** TimeAtlas has no HTTP client compiled in. The only files it touches are inside `~/.timeatlas/`.
2. **No team / multi-user features.** Single-user tool. No admin dashboards, no aggregation across users.
3. **No surveillance primitives.** No screenshot capture, no keystroke logging, no clipboard recording, no application-content scraping.

If you want to verify: clone the repo, search for `reqwest`, `hyper`, `http` — you'll only find them as transitive dependencies of Tauri itself for its IPC layer, never used for outbound traffic.

---

## Roadmap

Planned v0.3:

- [ ] Categorization — tag apps as Work / Communication / Browse / Other
- [ ] Weekly view — same SVG bar pattern, 7 stacked rows
- [ ] CSV export from the Summary tab
- [ ] Auto-start at login (platform-specific)
- [ ] Wayland idle detection (currently best-effort)

v0.4+:

- [ ] Optional encrypted local backup (JSON or sqlite dump)
- [ ] Customizable bucket size (currently fixed at 5 min)
- [ ] Tagging rules — "anything matching regex X gets tagged as Y"

What I will **not** add — even if you ask:

- ❌ Cloud sync (even opt-in)
- ❌ Telemetry / analytics
- ❌ Team / admin views
- ❌ Screenshot or keystroke capture of any kind

---

## Contributing

PRs welcome — see [`CONTRIBUTING.md`](./CONTRIBUTING.md) for setup, code style, the checks CI runs, and the project's design constraints.

The pre-PR checklist is the same set of commands CI runs:

```bash
npm run build
cd src-tauri
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --lib
```

---

## License

[MIT](./LICENSE) — see file for details.

---

## Acknowledgments

- [ActivityWatch](https://activitywatch.net/) for proving that local-only personal time tracking is the right design.
- [Tauri](https://tauri.app/) team for an excellent v2 release with first-class tray and IPC support.
- [`active-win-pos-rs`](https://github.com/dimusic/active-win-pos-rs) and [`user-idle`](https://github.com/olback/user-idle) for the cross-platform building blocks.
