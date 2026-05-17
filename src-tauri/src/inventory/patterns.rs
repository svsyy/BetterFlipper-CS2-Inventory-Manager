

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PatternInfo {
    pub tier: String,
    pub tier_color: String,
    pub rarity: PatternRarity,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum PatternRarity {
    Common,
    Uncommon,
    Rare,
    Legendary,
}

mod paint {
    pub const CASE_HARDENED: u32 = 44;
    pub const FADE: u32 = 38;
    pub const MARBLE_FADE: u32 = 413;
    pub const AMBER_FADE: u32 = 568;
}

mod def {
    pub const AK47: u32 = 7;
    pub const FIVE_SEVEN: u32 = 3;
    pub const KARAMBIT: u32 = 507;
}

fn ak_blue_gem(seed: u32) -> Option<u8> {
    match seed {
        661 | 670 | 955 | 387 | 321 => Some(1),
        760 | 809 | 868 | 592 | 555 | 828 | 463 | 442 | 147 | 219 => Some(2),
        103 | 179 | 168 | 905 | 123 | 151 | 617 | 784 | 429 | 695 |
        57 | 490 | 479 | 823 | 562 => Some(3),
        _ => None,
    }
}

fn five_seven_blue_gem(seed: u32) -> Option<u8> {
    match seed {
        690 | 278 | 868 | 363 => Some(1),
        532 | 670 | 689 | 872 | 151 | 648 | 585 | 661 | 57 => Some(2),
        787 | 827 | 512 | 442 | 922 | 323 | 210 | 97 => Some(3),
        _ => None,
    }
}

fn karambit_blue_gem(seed: u32) -> Option<u8> {
    match seed {
        387 | 853 | 463 | 442 => Some(1),
        828 | 670 | 809 | 269 | 592 | 321 | 955 | 868 => Some(2),
        555 | 760 | 147 | 219 | 661 | 617 => Some(3),
        _ => None,
    }
}

fn knife_blue_gem(seed: u32) -> Option<u8> {
    match seed {
        387 | 853 | 463 | 442 | 670 => Some(1),
        828 | 809 | 321 | 955 | 868 | 592 | 269 => Some(2),
        555 | 760 | 147 | 219 | 661 | 617 | 151 => Some(3),
        _ => None,
    }
}

fn fade_percentage(seed: u32) -> u32 {
    let normalized = seed as f32 / 1001.0;
    let pct = 100.0 - (normalized * 20.0);
    pct.clamp(80.0, 100.0).round() as u32
}

fn marble_fade_pattern(seed: u32) -> (String, u8) {
    const TRUE_FIRE_ICE: &[u32] = &[
        2, 38, 55, 62, 81, 108, 116, 146, 152, 185, 199, 203, 248, 278, 282,
        293, 310, 327, 334, 341, 363, 392, 406, 412, 420, 428, 456, 470, 473,
        489, 502, 509, 548, 573, 586, 602, 615, 632, 653, 662, 668, 680, 698,
        714, 723, 741, 779, 786, 806, 823, 836, 854, 861, 889, 896, 912, 935,
        950, 962, 989,
    ];
    const FAKE_FIRE_ICE: &[u32] = &[
        5, 14, 26, 47, 72, 85, 97, 123, 134, 158, 167, 174, 192, 216, 225,
        237, 256, 268, 289, 301, 318, 345, 358, 375, 384, 399, 418, 434, 445,
        462, 478, 495, 512, 528, 543, 560, 575, 592, 608, 625, 642, 658, 675,
        692, 708, 725, 742, 758, 775, 792, 808, 825, 842, 858, 875, 892, 908,
        925, 942, 958, 975, 992,
    ];
    if TRUE_FIRE_ICE.contains(&seed) { ("True Fire & Ice".into(), 1) }
    else if FAKE_FIRE_ICE.contains(&seed) { ("Fake Fire & Ice".into(), 2) }
    else { ("Tri-Color".into(), 3) }
}

pub fn get_pattern_info(def_index: u32, paint_index: u32, paint_seed: u32) -> Option<PatternInfo> {
    use PatternRarity::*;

    if paint_index == paint::CASE_HARDENED {
        let tier = if def_index == def::AK47 { ak_blue_gem(paint_seed) }
            else if def_index == def::FIVE_SEVEN { five_seven_blue_gem(paint_seed) }
            else if def_index == def::KARAMBIT { karambit_blue_gem(paint_seed) }
            else if def_index >= 500 && def_index < 600 { knife_blue_gem(paint_seed) }
            else { None };

        if let Some(t) = tier {
            let (color, rarity) = match t {
                1 => ("#3b82f6", Legendary),
                2 => ("#60a5fa", Rare),
                _ => ("#93c5fd", Uncommon),
            };
            return Some(PatternInfo {
                tier: format!("Blue Gem T{t}"),
                tier_color: color.into(),
                rarity,
            });
        }
    }

    if paint_index == paint::FADE {
        let pct = fade_percentage(paint_seed);
        let (color, rarity) = if pct >= 98 { ("#fbbf24", Legendary) }
            else if pct >= 95 { ("#f59e0b", Rare) }
            else if pct >= 90 { ("#d97706", Uncommon) }
            else { ("#92400e", Common) };
        return Some(PatternInfo { tier: format!("{pct}% Fade"), tier_color: color.into(), rarity });
    }

    if paint_index == paint::AMBER_FADE {
        let pct = fade_percentage(paint_seed);
        let (color, rarity) = if pct >= 98 { ("#f97316", Legendary) }
            else if pct >= 95 { ("#ea580c", Rare) }
            else if pct >= 90 { ("#c2410c", Uncommon) }
            else { ("#9a3412", Common) };
        return Some(PatternInfo { tier: format!("{pct}% Fade"), tier_color: color.into(), rarity });
    }

    if paint_index == paint::MARBLE_FADE {
        let (name, tier) = marble_fade_pattern(paint_seed);
        let (color, rarity) = match tier {
            1 => ("#ef4444", Legendary),
            2 => ("#f97316", Rare),
            _ => ("#6b7280", Common),
        };
        return Some(PatternInfo { tier: name, tier_color: color.into(), rarity });
    }

    None
}
