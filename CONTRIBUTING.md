# Contributing

Thanks for considering a contribution!

## Setting up

1. Install Rust 1.83+, Node 20+, and the platform-specific WebView deps listed
   in the [README](README.md).
2. `npm install`
3. `pwsh scripts/fetch-protos.ps1` (or the `.sh` variant on Linux/macOS).
4. `npm run tauri:dev`

## Code style

* **Rust**: `cargo fmt` and `cargo clippy --all-targets`. Keep modules small;
  every file should fit in one screen of `head -n 80`.
* **Svelte/TS**: 2-space indent, single quotes, semicolons, no trailing comma.
  Run `npm run check` before pushing.

## The "good first issues"

Anything tagged `TODO(steam-vent):` in the Rust source. These are concrete
wiring tasks — start with `src-tauri/src/steam/auth.rs::login_credentials`,
because everything else downstream depends on a working `SteamSession`.

## What we DON'T want

This is an intentionally focused project. The following are explicitly out of
scope and PRs adding them will be politely declined:

* Discord Rich Presence, Slack/Discord bots, telemetry, auto-update servers.
* Pricing data, market arbitrage, trading dashboards.
* Any feature that requires a hosted backend.

Build those things in a fork.
