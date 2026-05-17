// build.rs — Tauri only. CS2 Game Coordinator protobufs come from the
// `steam-vent-proto-csgo` crate (pulled in via the `csgo` feature on
// steam-vent), so no local prost-build step is needed.

fn main() {
    tauri_build::build();
}
