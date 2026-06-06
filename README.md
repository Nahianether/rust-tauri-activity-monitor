# TimeAtlas

> A **personal**, **local-only** time-awareness desktop app for developers and knowledge workers.
> Built with Tauri 2 + Rust + React.

TimeAtlas quietly tracks which apps and windows you spend time in throughout the day, then shows you a clean timeline and daily summary so you can understand where your hours actually go.

It is intentionally **personal**, **not** a team monitoring tool:
- No screenshot capture
- No keystroke logging
- No cloud sync — all data lives on your machine
- No manager/admin dashboards
- No productivity scoring or judgment

Inspired by [ActivityWatch](https://activitywatch.net/), reimagined with a modern Tauri 2 + React stack and a focus on a single user's awareness — not workplace surveillance.

## Why I built this

I wanted to know where my own time goes during a coding day without handing telemetry to a SaaS, and without the heavy footprint of Electron-based tools. Tauri 2 + Rust gives a single ~10MB binary that quietly observes the active window, writes to a local SQLite file, and never phones home.

## Features (v0.1)

- ✅ Detect active window title and application name (cross-platform: macOS, Windows, Linux)
- ✅ Idle detection — no mouse/keyboard activity for N minutes is recorded as idle
- ✅ Local SQLite storage of `(timestamp, app_name, window_title, duration)`
- ✅ Timeline view — today's activity as a horizontal bar
- ✅ Daily summary — time per app, top 10 windows
- ✅ Pause / resume tracking from system tray
- ❌ **Out of scope by design:** cloud sync, team features, screenshots, keylogging, AI categorization

## Tech stack

| Layer | Choice |
|---|---|
| Desktop runtime | Tauri 2.x |
| Backend | Rust (tokio async) |
| Storage | SQLite via `sqlx` |
| Active-window detection | `active-win-pos-rs` |
| Frontend | React 18 + TypeScript + Vite |
| Styling | Tailwind CSS |

## Architecture

```
┌─────────────────────────────────────────┐
│            React frontend               │
│  (Timeline, DailySummary, Settings)     │
└──────────────┬──────────────────────────┘
               │ Tauri IPC commands
┌──────────────▼──────────────────────────┐
│           Rust backend                  │
│  ┌──────────────┐   ┌────────────────┐  │
│  │ tracker loop │──▶│ sqlite writer  │  │
│  │ (1s polling) │   └────────────────┘  │
│  └──────────────┘                       │
│  ┌──────────────┐                       │
│  │ idle detector│                       │
│  └──────────────┘                       │
└─────────────────────────────────────────┘
               │
        ┌──────▼──────┐
        │ ~/.timeatlas│
        │   /db.sqlite│
        └─────────────┘
```

## Install (development)

Prerequisites:
- Rust 1.75+
- Node 18+
- Tauri prerequisites: https://tauri.app/start/prerequisites/

```bash
git clone https://github.com/Nahianether/rust-tauri-activity-monitor
cd rust-tauri-activity-monitor
npm install
npm run tauri dev
```

## Build a release binary

```bash
npm run tauri build
```

Outputs a single binary to `src-tauri/target/release/`.

## Roadmap

- [ ] v0.1 — active window tracking, idle detection, daily summary
- [ ] v0.2 — tagging (mark windows as "work", "deep focus", "communication")
- [ ] v0.3 — weekly / monthly reports
- [ ] v0.4 — optional encrypted local export (CSV / JSON)

## Privacy

TimeAtlas stores all data in `~/.timeatlas/db.sqlite` on your machine. There is no network code in this app. The only outbound network request is the auto-update check (which can be disabled in settings).

## License

MIT — see [LICENSE](./LICENSE).
