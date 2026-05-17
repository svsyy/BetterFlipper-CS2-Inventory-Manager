

use std::collections::HashMap;
use std::sync::OnceLock;

use serde::Deserialize;
use tokio::sync::RwLock;

use crate::error::AppResult;
use crate::inventory::cache::ensure_endpoint_caches;

pub type DefPaintKey = (u32, u32);

#[derive(Debug, Clone, Default)]
pub struct ItemMeta {
    pub name: String,
    pub image_url: Option<String>,
    pub rarity: u8,
    pub collection: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct StickerMeta {
    pub name: String,
    pub image_url: Option<String>,
}

#[derive(Deserialize)]
struct ByMykelEntry {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    market_hash_name: Option<String>,
    #[serde(default)]
    image: Option<String>,
    #[serde(default)]
    weapon: Option<ByMykelWeapon>,
    #[serde(default)]
    paint_index: Option<serde_json::Value>,
    #[serde(default)]
    def_index: Option<serde_json::Value>,
    #[serde(default)]
    rarity: Option<ByMykelRarity>,
    #[serde(default)]
    collections: Option<Vec<ByMykelCollection>>,
}

#[derive(Deserialize)]
struct ByMykelWeapon {
    
    
    
    #[serde(default)]
    weapon_id: Option<u32>,
}

#[derive(Deserialize)]
struct ByMykelRarity {
    #[serde(default)]
    id: Option<String>,
}

#[derive(Deserialize)]
struct ByMykelCollection {
    #[serde(default)]
    name: Option<String>,
}

#[derive(Default)]
pub struct ItemResolver {
    by_def_paint: HashMap<DefPaintKey, ItemMeta>,
    by_name: HashMap<String, ItemMeta>,
    by_sticker_kit: HashMap<u32, StickerMeta>,
}

impl ItemResolver {
    
    pub fn lookup(&self, def_index: u32, paint_index: u32, name_hint: Option<&str>) -> Option<ItemMeta> {
        if let Some(m) = self.by_def_paint.get(&(def_index, paint_index)) {
            return Some(m.clone());
        }
        if let Some(m) = self.by_def_paint.get(&(def_index, 0)) {
            return Some(m.clone());
        }
        if let Some(name) = name_hint {
            let key = normalize_name(name);
            if let Some(m) = self.by_name.get(&key) {
                return Some(m.clone());
            }
        }
        None
    }

    pub fn lookup_by_name(&self, name: &str) -> Option<ItemMeta> {
        self.by_name.get(&normalize_name(name)).cloned()
    }

    pub fn lookup_sticker(&self, sticker_kit_id: u32) -> Option<StickerMeta> {
        self.by_sticker_kit.get(&sticker_kit_id).cloned()
    }

    pub fn def_paint_count(&self) -> usize { self.by_def_paint.len() }
    pub fn name_count(&self) -> usize { self.by_name.len() }
    pub fn sticker_count(&self) -> usize { self.by_sticker_kit.len() }
}

static RESOLVER: OnceLock<RwLock<ItemResolver>> = OnceLock::new();

fn resolver_cell() -> &'static RwLock<ItemResolver> {
    RESOLVER.get_or_init(|| RwLock::new(ItemResolver::default()))
}

pub async fn lookup(def_index: u32, paint_index: u32) -> Option<ItemMeta> {
    let guard = resolver_cell().read().await;
    guard.lookup(def_index, paint_index, None)
}

pub async fn lookup_with_name(def_index: u32, paint_index: u32, name: &str) -> Option<ItemMeta> {
    let guard = resolver_cell().read().await;
    guard.lookup(def_index, paint_index, Some(name))
}

pub async fn lookup_by_name(name: &str) -> Option<ItemMeta> {
    let guard = resolver_cell().read().await;
    guard.lookup_by_name(name)
}

pub async fn lookup_sticker(sticker_kit_id: u32) -> Option<StickerMeta> {
    let guard = resolver_cell().read().await;
    guard.lookup_sticker(sticker_kit_id)
}

pub async fn initialize() -> AppResult<()> {
    let endpoints = ensure_endpoint_caches().await?;
    if endpoints.is_empty() {
        tracing::warn!("no ByMykel endpoint cached and network unavailable — running with bare def_index names");
        return Ok(());
    }

    let mut by_def_paint: HashMap<DefPaintKey, ItemMeta> = HashMap::with_capacity(20_000);
    let mut by_name: HashMap<String, ItemMeta> = HashMap::with_capacity(20_000);
    let mut by_sticker_kit: HashMap<u32, StickerMeta> = HashMap::with_capacity(8_000);

    for (endpoint, path) in &endpoints {
        let Ok(bytes) = tokio::fs::read(path).await else { continue };

        
        
        
        
        
        let raw: serde_json::Value = match serde_json::from_slice(&bytes) {
            Ok(v) => v,
            Err(e) => {
                tracing::warn!("{endpoint} parse failed: {e}");
                continue;
            }
        };
        let raw_entries: Vec<serde_json::Value> = match raw {
            serde_json::Value::Array(arr) => arr,
            other => {
                tracing::warn!("{endpoint} unexpected shape (not array): {}", other.is_object());
                continue;
            }
        };

        let mut endpoint_count = 0usize;
        for raw_e in raw_entries {
            let Ok(e) = serde_json::from_value::<ByMykelEntry>(raw_e) else { continue };
            endpoint_count += 1;
            let Some(image) = e.image.clone() else { continue };

            let rarity = parse_rarity(e.rarity.as_ref().and_then(|r| r.id.as_deref()));
            let collection = e.collections.as_ref()
                .and_then(|cs| cs.first().and_then(|c| c.name.clone()));

            
            if let Some(name) = &e.name {
                let key = normalize_name(name);
                by_name.entry(key).or_insert_with(|| ItemMeta {
                    name: name.clone(),
                    image_url: Some(image.clone()),
                    rarity,
                    collection: collection.clone(),
                });
            }
            if let Some(mhn) = &e.market_hash_name {
                let key = normalize_name(mhn);
                by_name.entry(key).or_insert_with(|| ItemMeta {
                    name: mhn.clone(),
                    image_url: Some(image.clone()),
                    rarity,
                    collection: collection.clone(),
                });
            }

            
            if *endpoint == "skins.json" {
                if let (Some(weapon_id), Some(paint_index_raw)) =
                    (e.weapon.as_ref().and_then(|w| w.weapon_id), e.paint_index.as_ref())
                {
                    if let Some(paint_index) = value_to_u32(paint_index_raw) {
                        let final_name = e.name.clone()
                            .or(e.market_hash_name.clone())
                            .unwrap_or_default();
                        by_def_paint.insert(
                            (weapon_id, paint_index),
                            ItemMeta {
                                name: final_name,
                                image_url: Some(image.clone()),
                                rarity,
                                collection: collection.clone(),
                            },
                        );
                    }
                }
            }

            
            
            
            
            
            
            
            let is_item_def_endpoint = matches!(
                *endpoint,
                "agents.json" | "crates.json" | "keys.json"
                | "collectibles.json" | "tools.json" | "graffiti.json"
            );
            if is_item_def_endpoint {
                if let Some(def_idx_raw) = e.def_index.as_ref() {
                    if let Some(def_idx) = value_to_u32(def_idx_raw) {
                        let final_name = e.name.clone()
                            .or(e.market_hash_name.clone())
                            .unwrap_or_default();
                        by_def_paint.entry((def_idx, 0)).or_insert(ItemMeta {
                            name: final_name,
                            image_url: Some(image.clone()),
                            rarity,
                            collection: collection.clone(),
                        });
                    }
                }
            }

            
            
            
            
            if *endpoint == "music_kits.json" {
                if let Some(def_idx_raw) = e.def_index.as_ref() {
                    if let Some(music_idx) = value_to_u32(def_idx_raw) {
                        let final_name = e.name.clone()
                            .or(e.market_hash_name.clone())
                            .unwrap_or_default();
                        by_def_paint.insert((1314, music_idx), ItemMeta {
                            name: final_name,
                            image_url: Some(image.clone()),
                            rarity,
                            collection: collection.clone(),
                        });
                    }
                }
            }

            
            if matches!(*endpoint, "stickers.json" | "patches.json" | "keychains.json") {
                if let Some(def_idx_raw) = e.def_index.as_ref() {
                    if let Some(def_idx) = value_to_u32(def_idx_raw) {
                        let name = e.name.clone().unwrap_or_default();
                        by_sticker_kit.insert(def_idx, StickerMeta {
                            name,
                            image_url: Some(image.clone()),
                        });
                    }
                }
            }
        }
        tracing::info!("{endpoint}: {endpoint_count} entries parsed");
    }

    let dp = by_def_paint.len();
    let nm = by_name.len();
    let sk = by_sticker_kit.len();

    *resolver_cell().write().await = ItemResolver {
        by_def_paint,
        by_name,
        by_sticker_kit,
    };
    tracing::info!("item resolver loaded: {dp} def+paint, {nm} names, {sk} sticker kits");
    Ok(())
}

fn value_to_u32(v: &serde_json::Value) -> Option<u32> {
    match v {
        serde_json::Value::Number(n) => n.as_u64().map(|x| x as u32),
        serde_json::Value::String(s) => s.parse().ok(),
        _ => None,
    }
}

fn parse_rarity(id: Option<&str>) -> u8 {
    match id.unwrap_or("") {
        "rarity_common_weapon" | "rarity_common"       => 1,
        "rarity_uncommon_weapon" | "rarity_uncommon"   => 2,
        "rarity_rare_weapon" | "rarity_rare"           => 3,
        "rarity_mythical_weapon" | "rarity_mythical"   => 4,
        "rarity_legendary_weapon" | "rarity_legendary" => 5,
        "rarity_ancient_weapon" | "rarity_ancient"     => 6,
        "rarity_contraband"                            => 6,
        _ => 0,
    }
}

pub fn normalize_name(name: &str) -> String {
    let mut s = name.to_string();
    for prefix in ["StatTrak™ ", "StatTrak™\u{00a0}", "Souvenir ", "★ "] {
        if let Some(rest) = s.strip_prefix(prefix) {
            s = rest.to_string();
        }
    }
    
    if let Some(open) = s.rfind('(') {
        if s.ends_with(')') {
            let suffix = &s[open..];
            if matches!(
                suffix,
                "(Factory New)" | "(Minimal Wear)" | "(Field-Tested)" | "(Well-Worn)" | "(Battle-Scarred)"
            ) {
                s = s[..open].trim().to_string();
            }
        }
    }
    s.trim().to_lowercase()
}

pub fn special_item_image(def_index: u32) -> Option<&'static str> {
    match def_index {
        1201 => Some("https://community.fastly.steamstatic.com/economy/image/i0CoZ81Ui0m-9KwlBY1L_18myuGuq1wfhWSaZgMttyVfPaERSR0Wqmu7LAocGJG51EejH_XV0MGkITXE5AB094KtuwG0Exv1yMfkqXcCtvT_MPw5JPTKV2bDk7Z3sudtHSjr2w0ptCMWPT2u/330x192?allow_animated=1"),
        _ => None,
    }
}
