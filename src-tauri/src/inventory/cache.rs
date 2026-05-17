

use std::path::PathBuf;
use std::time::{Duration, SystemTime};

use directories::ProjectDirs;

use crate::error::{AppError, AppResult};

const API_NAME: &str = "cs2-im";
const API_QUALIFIER: &str = "com";
const API_ORG: &str = "cs2-inventory-manager";

const BYMYKEL_BASE: &str =
    "https://raw.githubusercontent.com/ByMykel/CSGO-API/main/public/api/en";

const STALE_AFTER: Duration = Duration::from_secs(24 * 60 * 60); 

pub const ENDPOINTS: &[&str] = &[
    "skins.json",
    "stickers.json",
    "agents.json",
    "crates.json",
    "keys.json",
    "collectibles.json",
    "music_kits.json",
    "patches.json",
    "graffiti.json",
    "keychains.json",
    "tools.json",
];

fn cache_dir() -> AppResult<PathBuf> {
    ProjectDirs::from(API_QUALIFIER, API_ORG, API_NAME)
        .map(|d| d.data_dir().to_path_buf())
        .ok_or_else(|| AppError::other("could not resolve user data dir"))
}

fn endpoint_path(endpoint: &str) -> AppResult<PathBuf> {
    let dir = cache_dir()?;
    std::fs::create_dir_all(&dir)?;
    Ok(dir.join(endpoint))
}

fn is_fresh(path: &PathBuf) -> bool {
    let Ok(meta) = std::fs::metadata(path) else { return false };
    let Ok(modified) = meta.modified() else { return false };
    let Ok(age) = SystemTime::now().duration_since(modified) else { return true };
    age < STALE_AFTER
}

pub async fn ensure_endpoint_caches() -> AppResult<Vec<(&'static str, PathBuf)>> {
    let mut out = Vec::with_capacity(ENDPOINTS.len());

    for endpoint in ENDPOINTS {
        let path = endpoint_path(endpoint)?;

        if is_fresh(&path) {
            out.push((*endpoint, path));
            continue;
        }

        match download_endpoint(endpoint, &path).await {
            Ok(()) => out.push((*endpoint, path)),
            Err(e) => {
                tracing::warn!("endpoint {endpoint} refresh failed: {e}");
                if path.exists() {
                    out.push((*endpoint, path));
                }
            }
        }
    }

    Ok(out)
}

async fn download_endpoint(endpoint: &str, dest: &PathBuf) -> AppResult<()> {
    let url = format!("{BYMYKEL_BASE}/{endpoint}");
    tracing::info!("fetching {endpoint}…");
    let tmp = dest.with_extension("json.tmp");
    let bytes = reqwest::get(&url).await?.bytes().await?;
    tokio::fs::write(&tmp, &bytes).await?;
    tokio::fs::rename(&tmp, dest).await?;
    tracing::info!("{endpoint} cached ({} bytes)", bytes.len());
    Ok(())
}

pub fn market_image_cache_path() -> AppResult<PathBuf> {
    let dir = cache_dir()?;
    std::fs::create_dir_all(&dir)?;
    Ok(dir.join("market_images.json"))
}
