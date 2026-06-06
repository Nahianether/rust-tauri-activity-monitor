# Contributing to TimeAtlas

Thanks for your interest! TimeAtlas is intentionally small and opinionated, so
the contribution bar is a little different from a generic open-source project:
please read this before opening a large PR.

## Project philosophy

TimeAtlas is a **personal, local-only** time-awareness app. Three things are
non-negotiable design constraints, and PRs that violate them will be declined:

1. **No outbound network requests.** Data never leaves the user's machine.
2. **No team / multi-user features.** This is a single-user tool, not a
   workforce-monitoring product. No admin dashboards, no aggregations across
   users, no cloud sync.
3. **No surveillance primitives.** No screenshot capture, no keystroke logging,
   no clipboard recording, no application-content scraping.

Features that fit these constraints — categorization, weekly summaries, exports,
better visualizations, idle-detection accuracy on Wayland, etc. — are welcome.

## Local setup

Prerequisites:

- Rust ≥ 1.75 (`rustup` recommended)
- Node ≥ 18
- Tauri 2 platform prerequisites — see <https://tauri.app/start/prerequisites/>
  - **Windows**: WebView2 (usually pre-installed) + MSVC build tools
  - **macOS**: Xcode Command Line Tools
  - **Linux**: `libwebkit2gtk-4.1-dev libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev`

Once those are installed:

```bash
git clone https://github.com/Nahianether/rust-tauri-activity-monitor
cd rust-tauri-activity-monitor
npm install
npm run tauri dev
```

The dev server compiles in ~30–60 seconds on first run, then reloads instantly
on frontend changes via Vite HMR.

## Running checks before opening a PR

Please make sure all of these pass before submitting:

```bash
# Frontend (TypeScript + Vite)
npm run build

# Rust (formatting, lints, tests)
cd src-tauri
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --lib
```

If `cargo fmt` complains, run `cargo fmt --all` to autofix.

## Code style

- **Rust** — standard `rustfmt` defaults plus what's in `rustfmt.toml`. Keep
  modules tightly scoped; the tracker subsystem (`tracker/`) should not import
  from `commands.rs` or `lib.rs`.
- **TypeScript** — strict mode, no `any`, no implicit any. Components live in
  `src/components/` and are function components only.
- **Comments** — explain *why*, not *what*. The code already shows *what*.

## Commit messages

Short, declarative present tense. Group related changes. If you touch both Rust
and frontend, that's fine — but explain the cross-cutting reason in the body.

```
add weekly summary view

Adds a `get_weekly_summary` command that groups events by day-of-week,
and a `WeeklyView` component to render the result. Reuses the per-app
color hashing from `theme.ts`.
```

## What I'm unlikely to merge

- Adding any kind of telemetry, analytics, or "anonymous" usage stats.
- Adding a cloud-sync mode (even opt-in).
- Adding a team or admin view.
- Replacing the SQLite store with something heavier.
- Adding screenshots, keystroke capture, or content scraping of any kind.

## Reporting bugs

Please include:

- OS + version
- Output of `npm run tauri info`
- A clear repro — what you did, what you expected, what happened.

## Maintainer

[Nahianether](https://github.com/Nahianether) — feel free to reach out on
issues or via the contact info in my [GitHub profile](https://github.com/Nahianether).
