

use serde::{Deserialize, Serialize};

use crate::inventory::patterns::PatternInfo;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: String,             
    pub def_index: u32,
    pub name: String,
    pub custom_name: Option<String>,
    pub image_url: Option<String>,

    
    pub paint_index: Option<u32>,
    pub paint_seed: Option<u32>,
    pub paint_wear: Option<f32>,
    pub wear_name: Option<String>,  

    
    pub rarity: u8,             
    pub rarity_name: String,
    pub rarity_color: String,   

    
    pub stattrak: bool,
    pub stattrak_count: Option<u32>,
    pub souvenir: bool,
    pub stickers: Vec<StickerInfo>,

    
    pub moveable: bool,
    pub tradable_after: Option<String>,    
    pub equipped_ct: bool,
    pub equipped_t: bool,
    pub position: i32,
    pub origin: u32,

    
    pub is_container: bool,
    pub is_storage_unit: bool,
    pub storage_unit_item_count: Option<u32>,
    
    
    pub casket_id: Option<String>,
    
    
    pub casket_internal_id: Option<String>,
    
    
    pub original_id: Option<String>,

    
    pub collection: Option<String>,
    
    pub pattern: Option<PatternInfo>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StickerInfo {
    pub slot: u8,
    pub name: String,
    pub image_url: Option<String>,
    pub wear: f32,              
}

pub fn rarity_color(rarity: u8) -> &'static str {
    match rarity {
        1 => "#b0c3d9",
        2 => "#5e98d9",
        3 => "#4b69ff",
        4 => "#8847ff",
        5 => "#d32ce6",
        6 => "#eb4b4b",
        _ => "#9ca3af",
    }
}

pub fn rarity_name(rarity: u8) -> &'static str {
    match rarity {
        1 => "Consumer Grade",
        2 => "Industrial Grade",
        3 => "Mil-Spec",
        4 => "Restricted",
        5 => "Classified",
        6 => "Covert",
        _ => "Unknown",
    }
}

pub fn wear_bucket(wear: f32) -> &'static str {
    match wear {
        w if w < 0.07 => "Factory New",
        w if w < 0.15 => "Minimal Wear",
        w if w < 0.38 => "Field-Tested",
        w if w < 0.45 => "Well-Worn",
        _ => "Battle-Scarred",
    }
}
