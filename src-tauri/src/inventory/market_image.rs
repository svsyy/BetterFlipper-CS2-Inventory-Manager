

use std::collections::HashMap;
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::error::AppResult;
use crate::inventory::cache::market_image_cache_path;

const MAX_ENTRIES: usize = 5000;

#[derive(Serialize, Deserialize, Default)]
struct DiskCache {
    entries: HashMap<String, String>,
}

struct MarketImageState {
    
    entries: Vec<(String, String)>,
    loaded_from_disk: bool,
}

impl MarketImageState {
    fn new() -> Self {
        Self { entries: Vec::with_capacity(MAX_ENTRIES), loaded_from_disk: false }
    }

    fn get(&mut self, key: &str) -> Option<String> {
        let pos = self.entries.iter().position(|(k, _)| k == key)?;
        let entry = self.entries.remove(pos);
        let url = entry.1.clone();
        self.entries.push(entry);
        Some(url)
    }

    fn insert(&mut self, key: String, url: String) {
        if let Some(pos) = self.entries.iter().position(|(k, _)| *k == key) {
            self.entries.remove(pos);
        }
        self.entries.push((key, url));
        if self.entries.len() > MAX_ENTRIES {
            self.entries.remove(0);
        }
    }

    fn snapshot_to_disk(&self) -> DiskCache {
        let mut entries = HashMap::with_capacity(self.entries.len());
        for (k, v) in &self.entries {
            if !v.is_empty() {
                entries.insert(k.clone(), v.clone());
            }
        }
        DiskCache { entries }
    }
}

fn state() -> &'static Mutex<MarketImageState> {
    static STATE: OnceLock<Mutex<MarketImageState>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(MarketImageState::new()))
}

async fn ensure_disk_loaded() {
    let mut s = state().lock().await;
    if s.loaded_from_disk { return; }
    s.loaded_from_disk = true;
    let Ok(path) = market_image_cache_path() else { return };
    let Ok(bytes) = tokio::fs::read(&path).await else { return };
    let Ok(cache): Result<DiskCache, _> = serde_json::from_slice(&bytes) else { return };
    for (k, v) in cache.entries {
        s.insert(k, v);
    }
}

async fn persist_to_disk() -> AppResult<()> {
    let snapshot = { state().lock().await.snapshot_to_disk() };
    let path = market_image_cache_path()?;
    let bytes = serde_json::to_vec(&snapshot)?;
    tokio::fs::write(&path, bytes).await?;
    Ok(())
}

pub async fn resolve(item_name: &str) -> Option<String> {
    if item_name.is_empty() { return None; }
    ensure_disk_loaded().await;

    {
        let mut s = state().lock().await;
        if let Some(cached) = s.get(item_name) {
            if !cached.is_empty() {
                return Some(cached);
            }
            return None;
        }
    }

    let url = format!(
        "https://steamcommunity.com/market/listings/730/{}?l=english",
        urlencoding_encode(item_name)
    );

    let html = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .ok()?
        .get(&url)
        .header("Accept-Language", "en-US,en;q=0.9")
        .send()
        .await
    {
        Ok(r) => match r.text().await { Ok(t) => t, Err(_) => return None },
        Err(_) => return None,
    };

    let image_url = extract_image_url(&html);
    {
        let mut s = state().lock().await;
        s.insert(item_name.to_string(), image_url.clone().unwrap_or_default());
    }
    if image_url.is_some() {
        let _ = persist_to_disk().await;
    }
    image_url
}

pub async fn resolve_many(names: &[String]) -> HashMap<String, String> {
    let futures = names.iter().map(|n| {
        let name = n.clone();
        async move {
            let url = resolve(&name).await;
            (name, url)
        }
    });
    let results = futures_util::future::join_all(futures).await;
    let mut out = HashMap::with_capacity(results.len());
    for (name, url) in results {
        if let Some(u) = url {
            out.insert(name, u);
        }
    }
    out
}

fn extract_image_url(html: &str) -> Option<String> {
    
    
    const NEEDLE: &str = "://community.";
    let bytes = html.as_bytes();
    let mut idx = 0;
    while idx + NEEDLE.len() < bytes.len() {
        if let Some(pos) = html[idx..].find(NEEDLE) {
            let abs = idx + pos;
            
            if abs >= 5 && &html[abs - 5..abs] == "https" {
                
                let start = abs - 5;
                let end_offset = html[start..]
                    .find(|c: char| c == '"' || c == '\'' || c == ' ' || c == '\n' || c == '\r' || c == '\\')
                    .unwrap_or(html.len() - start);
                let candidate = &html[start..start + end_offset];
                if candidate.contains("/economy/image/") {
                    let upgraded = upgrade_size(candidate);
                    return Some(upgraded);
                }
            }
            idx = abs + NEEDLE.len();
        } else { return None; }
    }
    None
}

fn upgrade_size(url: &str) -> String {
    
    if let Some(slash) = url.rfind('/') {
        let (head, tail) = url.split_at(slash + 1);
        if tail.chars().all(|c| c.is_ascii_digit() || c == 'x') && tail.contains('x') {
            return format!("{head}330x192");
        }
    }
    url.to_string()
}

fn urlencoding_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len() * 3);
    for byte in s.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => out.push(byte as char),
            _ => out.push_str(&format!("%{:02X}", byte)),
        }
    }
    out
}
