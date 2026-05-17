# CS2 Game Coordinator protobufs

**You don't need to do anything here.**

The Rust crate `steam-vent-proto-csgo` (pulled in transitively via the
`csgo` feature on `steam-vent`) ships pre-generated bindings for every CS2
GC message we use. When CS2 updates the wire format, just bump the
`steam-vent-proto-csgo` version — no local `prost-build` step required.

This directory is kept only so the build script `scripts/fetch-protos.*`
has a stable destination if you ever want to vendor a custom subset.
