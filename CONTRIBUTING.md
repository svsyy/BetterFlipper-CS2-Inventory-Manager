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

Build those things in a fork.
