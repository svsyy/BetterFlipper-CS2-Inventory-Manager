

use std::collections::HashSet;

use steam_vent_proto_csgo::base_gcmessages::{CSOEconItem, CSOEconItemAttribute};

use crate::inventory::item::{rarity_color, rarity_name, wear_bucket, Item, StickerInfo};
use crate::inventory::patterns;
use crate::inventory::{market_image, resolver};

const ATTR_PAINT_INDEX: u32 = 6;
const ATTR_PAINT_SEED: u32 = 7;
const ATTR_PAINT_WEAR: u32 = 8;
const ATTR_TRADABLE_AFTER_DATE: u32 = 75;
const ATTR_KILL_EATER_SCORE: u32 = 80;
const ATTR_SOUVENIR: u32 = 140;
const ATTR_CUSTOM_NAME: u32 = 111;
const ATTR_FREE_REWARD: u32 = 277;
const ATTR_MUSIC_INDEX: u32 = 166;
const ATTR_STORAGE_UNIT_COUNT: u32 = 270;

const ATTR_CASKET_ID_LOW: u32 = 272;

const ATTR_CASKET_ID_HIGH: u32 = 273;
const ATTR_KEYCHAIN_KIT_ID: u32 = 299;

const ATTR_STICKER_BASE: [u32; 6] = [113, 114, 115, 116, 117, 118];
const ATTR_STICKER_WEAR_BASE: [u32; 6] = [119, 120, 121, 122, 123, 124];

pub const CASKET_DEF_INDEX: u32 = 1201;

fn base_name_for_def(def_index: u32) -> (Option<String>, Option<String>) {
    let storage_unit_img = Some(
        "https://community.fastly.steamstatic.com/economy/image/i0CoZ81Ui0m-9KwlBY1L_18myuGuq1wfhWSaZgMttyVfPaERSR0Wqmu7LAocGJG51EejH_XV0MGkITXE5AB094KtuwG0Exv1yMfkqXcCtvT_MPw5JPTKV2bDk7Z3sudtHSjr2w0ptCMWPT2u/330x192?allow_animated=1".to_string()
    );
    match def_index {
        1200 => (Some("Name Tag".into()), None),
        1201 => (Some("Storage Unit".into()), storage_unit_img),
        1209 => (Some("Sticker".into()), None),
        1314 => (Some("Music Kit".into()), None),
        1324 => (Some("StatTrak\u{2122} Swap Tool".into()), None),
        1349 => (Some("Patch".into()), None),
        1355 => (Some("Charm".into()), None),
        4950 => (Some("Charm Detachments".into()), None),
        _ => (None, None),
    }
}

pub async fn convert(econ_item: &CSOEconItem) -> Option<Item> {
    let def_index = econ_item.def_index();

    
    if let Some(v) = read_u32(econ_item, ATTR_FREE_REWARD) {
        if v == 1 { return None; }
    }

    
    let asset_id = econ_item.id();
    if asset_id == 17_293_822_569_110_896_676 || asset_id == 17_293_822_569_102_708_641 {
        return None;
    }

    
    
    
    
    let paint_index = read_u32_or_float(econ_item, ATTR_PAINT_INDEX).unwrap_or(0);
    let paint_seed = read_u32_or_float(econ_item, ATTR_PAINT_SEED).unwrap_or(0);
    let paint_wear = read_f32(econ_item, ATTR_PAINT_WEAR);

    let stattrak = has_attribute(econ_item, ATTR_KILL_EATER_SCORE);
    let stattrak_count = read_u32(econ_item, ATTR_KILL_EATER_SCORE);
    
    
    
    let souvenir = has_attribute(econ_item, ATTR_SOUVENIR);

    
    
    
    
    let is_sticker_def = matches!(def_index, 1209 | 1349);
    let stickers = if paint_index > 0 || is_sticker_def {
        extract_stickers(econ_item).await
    } else {
        Vec::new()
    };

    
    
    
    
    
    
    
    
    
    let (base_name, base_image) = base_name_for_def(def_index);
    let meta = resolver::lookup(def_index, paint_index).await;
    let mut resolver_name = meta.as_ref().map(|m| m.name.clone()).or(base_name);
    let mut image_url = meta.as_ref().and_then(|m| m.image_url.clone()).or(base_image);
    let collection = meta.as_ref().and_then(|m| m.collection.clone());
    let resolver_rarity = meta.as_ref().map(|m| m.rarity).unwrap_or(0);

    
    
    
    if def_index == 1314 {
        if let Some(music_idx) = read_u32(econ_item, ATTR_MUSIC_INDEX) {
            if let Some(meta) = resolver::lookup(1314, music_idx).await {
                resolver_name = Some(meta.name);
                image_url = meta.image_url.or(image_url);
            } else {
                resolver_name = Some(format!("Music Kit | #{music_idx}"));
            }
        }
    }

    
    if def_index == 1355 {
        if let Some(kit_id) = read_u32(econ_item, ATTR_KEYCHAIN_KIT_ID) {
            if let Some(meta) = resolver::lookup_sticker(kit_id).await {
                let bare = meta.name
                    .strip_prefix("Charm | ").unwrap_or(&meta.name)
                    .strip_prefix("Sticker | ").unwrap_or(&meta.name)
                    .to_string();
                resolver_name = Some(format!("Charm | {bare}"));
                image_url = meta.image_url.or(image_url);
            }
        }
    }

    
    if is_sticker_def {
        if let Some(first) = stickers.first() {
            if first.slot == 0 && !first.name.is_empty() {
                
                
                resolver_name = Some(first.name.clone());
                image_url = first.image_url.clone().or(image_url);
            }
        }
    }

    let is_knife = (500..1100).contains(&def_index);
    let star = is_knife;
    let prefix = if stattrak { "StatTrak™ " } else if souvenir { "Souvenir " } else { "" };
    let base_name = resolver_name.unwrap_or_else(|| format!("Item #{def_index}"));

    
    let needs_prefix = !base_name.starts_with("StatTrak™") && !base_name.starts_with("Souvenir");
    let needs_star = star && !base_name.starts_with('★');
    let name = match (needs_star, needs_prefix && !prefix.is_empty()) {
        (true, true)  => format!("★ {prefix}{base_name}"),
        (true, false) => format!("★ {base_name}"),
        (false, true) => format!("{prefix}{base_name}"),
        (false, false) => base_name,
    };

    let rarity_value = econ_item.rarity();
    let rarity_u8: u8 = if rarity_value > 0 {
        rarity_value.try_into().unwrap_or(0)
    } else {
        resolver_rarity
    };

    let is_storage_unit = def_index == CASKET_DEF_INDEX;
    let storage_unit_item_count =
        if is_storage_unit { read_u32(econ_item, ATTR_STORAGE_UNIT_COUNT) } else { None };

    
    
    
    
    
    let casket_id = match (
        read_u32(econ_item, ATTR_CASKET_ID_LOW),
        read_u32(econ_item, ATTR_CASKET_ID_HIGH),
    ) {
        (Some(low), Some(high)) => Some(((high as u64) << 32 | low as u64).to_string()),
        (Some(low), None)       => Some((low as u64).to_string()),
        _ => None,
    };
    let casket_internal_id: Option<String> = None;
    let original_id = if econ_item.has_original_id() {
        Some(econ_item.original_id().to_string())
    } else {
        None
    };

    
    
    let custom_name = if !econ_item.custom_name().is_empty() {
        Some(econ_item.custom_name().to_string())
    } else {
        read_string(econ_item, ATTR_CUSTOM_NAME)
    }.map(|s| s.trim_matches(|c: char| c.is_control() || c.is_whitespace()).to_string())
     .filter(|s| !s.is_empty());

    let tradable_after_secs = read_u32(econ_item, ATTR_TRADABLE_AFTER_DATE);
    let tradable_after = tradable_after_secs
        .and_then(|s| chrono::DateTime::<chrono::Utc>::from_timestamp(s as i64, 0))
        .filter(|d| *d > chrono::Utc::now())
        .map(|d| d.to_rfc3339());

    let equipped_ct = econ_item.equipped_state.iter().any(|e| e.new_class() == 3);
    let equipped_t = econ_item.equipped_state.iter().any(|e| e.new_class() == 2);

    
    if is_storage_unit && image_url.is_none() {
        image_url = resolver::special_item_image(def_index).map(String::from);
    }

    
    let is_container = !is_storage_unit && is_container_by_name(&name);
    let untradeable = is_untradeable_by_name(&name);

    Some(Item {
        id: econ_item.id().to_string(),
        def_index,
        name,
        custom_name,
        image_url,
        paint_index: Some(paint_index).filter(|p| *p != 0),
        paint_seed: Some(paint_seed).filter(|s| *s != 0),
        paint_wear,
        wear_name: paint_wear.map(|w| wear_bucket(w).to_string()),
        rarity: rarity_u8,
        rarity_name: rarity_name(rarity_u8).to_string(),
        rarity_color: rarity_color(rarity_u8).to_string(),
        stattrak,
        stattrak_count,
        souvenir,
        stickers,
        moveable: tradable_after.is_none() && !untradeable,
        tradable_after,
        equipped_ct,
        equipped_t,
        position: (econ_item.inventory() & 0xFFFF) as i32,
        origin: econ_item.origin(),
        is_container,
        is_storage_unit,
        storage_unit_item_count,
        casket_id,
        casket_internal_id,
        original_id,
        collection,
        pattern: if paint_seed != 0 {
            patterns::get_pattern_info(def_index, paint_index, paint_seed)
        } else { None },
    })
}

pub async fn convert_inventory(items: &[CSOEconItem]) -> Vec<Item> {
    let mut out = Vec::with_capacity(items.len());
    for econ in items {
        if let Some(item) = convert(econ).await {
            out.push(item);
        }
    }
    out.sort_by_key(|i| i.position);
    out
}

pub async fn enrich_missing_images_async<F>(items: Vec<Item>, on_resolved: F)
where
    F: Fn(String, String) + Send + 'static,
{
    let mut distinct_names: HashSet<String> = HashSet::new();
    for item in &items {
        if item.image_url.is_none() && !item.name.is_empty() {
            distinct_names.insert(item.name.clone());
        }
    }
    if distinct_names.is_empty() { return; }
    let names_vec: Vec<String> = distinct_names.into_iter().collect();
    tracing::info!("resolving {} missing images from Steam Market in background…", names_vec.len());

    for name in names_vec {
        if let Some(url) = market_image::resolve(&name).await {
            if !url.is_empty() {
                on_resolved(name, url);
            }
        }
        
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    }
}

fn read_attr<'a>(item: &'a CSOEconItem, def: u32) -> Option<&'a CSOEconItemAttribute> {
    item.attribute.iter().find(|a| a.def_index() == def)
}

fn has_attribute(item: &CSOEconItem, def: u32) -> bool {
    read_attr(item, def).is_some()
}

fn read_u32(item: &CSOEconItem, def: u32) -> Option<u32> {
    let attr = read_attr(item, def)?;
    if let Some(v) = attr.value { return Some(v); }
    let bytes = attr.value_bytes();
    if bytes.len() >= 4 {
        Some(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    } else { None }
}

fn read_u32_or_float(item: &CSOEconItem, def: u32) -> Option<u32> {
    let attr = read_attr(item, def)?;
    let bytes = attr.value_bytes();
    if bytes.len() >= 4 {
        let f = f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        if f.is_finite() && f >= 0.0 && f < (u32::MAX as f32) {
            return Some(f as u32);
        }
    }
    attr.value
}

fn read_u64(item: &CSOEconItem, def: u32) -> Option<u64> {
    let attr = read_attr(item, def)?;
    let bytes = attr.value_bytes();
    if bytes.len() >= 8 {
        Some(u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    } else if bytes.len() >= 4 {
        Some(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as u64)
    } else { None }
}

fn read_f32(item: &CSOEconItem, def: u32) -> Option<f32> {
    let attr = read_attr(item, def)?;
    let bytes = attr.value_bytes();
    if bytes.len() >= 4 {
        Some(f32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    } else if let Some(v) = attr.value {
        Some(f32::from_bits(v))
    } else { None }
}

fn read_string(item: &CSOEconItem, def: u32) -> Option<String> {
    let attr = read_attr(item, def)?;
    let bytes = attr.value_bytes();
    if bytes.is_empty() { return None; }
    String::from_utf8(bytes.to_vec()).ok()
}

async fn extract_stickers(item: &CSOEconItem) -> Vec<StickerInfo> {
    let mut out = Vec::new();
    for slot in 0..6 {
        let Some(sticker_id) = read_u32(item, ATTR_STICKER_BASE[slot]) else { continue };
        if sticker_id == 0 { continue; }
        let wear = read_f32(item, ATTR_STICKER_WEAR_BASE[slot]).unwrap_or(0.0);

        let (name, image_url) = match resolver::lookup_sticker(sticker_id).await {
            Some(meta) => (meta.name, meta.image_url),
            None => (format!("Sticker #{sticker_id}"), None),
        };

        out.push(StickerInfo {
            slot: slot as u8,
            name,
            image_url,
            wear,
        });
    }
    out
}

fn is_container_by_name(name: &str) -> bool {
    let lower = name.to_lowercase();
    lower.contains("case")
        || lower.contains("capsule")
        || lower.contains("package")
        || lower.contains("sticker capsule")
        || lower.contains("souvenir package")
}

fn is_untradeable_by_name(name: &str) -> bool {
    let lower = name.to_lowercase();
    lower.contains("veteran coin")
        || lower.contains("service medal")
        || lower.contains("loyalty badge")
        || lower.contains("medallion")
}
