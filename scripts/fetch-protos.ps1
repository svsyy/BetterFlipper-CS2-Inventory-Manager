# scripts/fetch-protos.ps1
# Downloads the CS2 Game Coordinator protobuf definitions we need.
# Run once after cloning the repo (and re-run when CS2 updates break the schema).
#
# Usage:  pwsh scripts/fetch-protos.ps1

$ErrorActionPreference = 'Stop'

$root      = Split-Path -Parent $PSScriptRoot
$protoDir  = Join-Path $root 'src-tauri\proto'
$baseUrl   = 'https://raw.githubusercontent.com/SteamDatabase/Protobufs/master/csgo'

$files = @(
    'base_gcmessages.proto'
    'econ_gcmessages.proto'
    'cstrike15_gcmessages.proto'
    'gcsystemmsgs.proto'
    'gcsdk_gcmessages.proto'
    'steammessages.proto'
    'steammessages_unified_base.steamclient.proto'
)

New-Item -ItemType Directory -Force -Path $protoDir | Out-Null

foreach ($name in $files) {
    $dest = Join-Path $protoDir $name
    $url  = "$baseUrl/$name"
    Write-Host "Fetching $name..." -ForegroundColor Cyan
    try {
        Invoke-WebRequest -Uri $url -OutFile $dest -UseBasicParsing
    } catch {
        Write-Warning "Failed to fetch $url ($_). Skipping."
    }
}

Write-Host ""
Write-Host "Done. Now run: npm run tauri:dev" -ForegroundColor Green
