#!/usr/bin/env bash
# scripts/fetch-protos.sh
# Downloads the CS2 Game Coordinator protobuf definitions we need.
# Run once after cloning the repo (and re-run when CS2 updates break the schema).

set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PROTO_DIR="$ROOT/src-tauri/proto"
BASE="https://raw.githubusercontent.com/SteamDatabase/Protobufs/master/csgo"

FILES=(
  base_gcmessages.proto
  econ_gcmessages.proto
  cstrike15_gcmessages.proto
  gcsystemmsgs.proto
  gcsdk_gcmessages.proto
  steammessages.proto
  steammessages_unified_base.steamclient.proto
)

mkdir -p "$PROTO_DIR"

for f in "${FILES[@]}"; do
  echo "Fetching $f..."
  curl -fsSL "$BASE/$f" -o "$PROTO_DIR/$f" || echo "WARN: failed to fetch $f"
done

echo
echo "Done. Now run: npm run tauri:dev"
