

use std::sync::Arc;

use futures_util::StreamExt;
use steam_vent::ConnectionTrait;
use steam_vent_proto_common::protobuf::Message;
use steam_vent_proto_csgo::base_gcmessages::CSOEconItem;
use steam_vent_proto_csgo::gcsdk_gcmessages::{CMsgClientHello, CMsgSOCacheSubscribed};
use steam_vent_proto_csgo::GCHandshake;
use steam_vent::GameCoordinator;
use tauri::{AppHandle, Emitter};

use crate::error::{AppError, AppResult};
use crate::events::{emit_login_status, LoginStage, INVENTORY_UPDATED};
use crate::inventory::parser;
use crate::inventory::Item;
use crate::state::SharedState;
use crate::steam::gc_msg::{SoSingleCreate, SoSingleDestroy, SoSingleUpdate, SoMultipleUpdate};
use crate::steam::session::SteamSession;

const CLIENT_VERSION: u32 = 2_000_651;

pub async fn start_cs2(
    session: &SteamSession,
    state: &SharedState,
    app: &AppHandle,
) -> AppResult<()> {
    emit_login_status(app, LoginStage::WaitingGc, Some("Asking Steam to play CS2".into()));

    let handshake = GCHandshake {
        hello: CMsgClientHello {
            version: Some(CLIENT_VERSION),
            client_session_need: Some(0),
            client_launcher: Some(0),
            steam_launcher: Some(0),
            ..Default::default()
        },
    };

    let (gc, welcome) = GameCoordinator::with_handshake(&session.conn, &handshake)
        .await
        .map_err(|e| AppError::Protocol(format!("GC handshake failed: {e}")))?;

    tracing::info!("GC welcome received; {} caches subscribed",
        welcome.outofdate_subscribed_caches.len());

    emit_login_status(app, LoginStage::LoadingInventory, None);

    let items = parse_inventory_from_welcome(&welcome).await;
    tracing::info!("inventory parsed: {} items", items.len());
    debug_dump_inventory(&items);
    *state.inventory.write().await = items.clone();

    
    
    let snapshot: Vec<Item> = items.iter()
        .filter(|i| i.casket_id.is_none())
        .cloned()
        .collect();
    let _ = app.emit(INVENTORY_UPDATED, &snapshot);

    
    spawn_market_enrichment(items, state.clone(), app.clone());

    let gc_arc = Arc::new(gc);
    *session.gc.lock().await = Some(gc_arc.clone());

    
    
    
    
    
    spawn_so_listener(gc_arc, state.clone(), app.clone());

    
    
    
    
    emit_login_status(app, LoginStage::Ready, None);
    Ok(())
}

fn spawn_market_enrichment(items: Vec<Item>, state: SharedState, app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let app2 = app.clone();
        let state2 = state.clone();
        parser::enrich_missing_images_async(items, move |name, url| {
            
            let app3 = app2.clone();
            let state3 = state2.clone();
            tauri::async_runtime::spawn(async move {
                let snapshot: Vec<Item> = {
                    let mut inv = state3.inventory.write().await;
                    for item in inv.iter_mut() {
                        if item.image_url.is_none() && item.name == name {
                            item.image_url = Some(url.clone());
                        }
                    }
                    inv.iter()
                        .filter(|i| i.casket_id.is_none())
                        .cloned()
                        .collect()
                };
                let _ = app3.emit(INVENTORY_UPDATED, &snapshot);
            });
        }).await;
    });
}

fn log_casket_breakdown(items: &[Item]) {
    use std::collections::HashMap;
    let mut by_casket: HashMap<Option<String>, Vec<&Item>> = HashMap::new();
    for it in items {
        by_casket.entry(it.casket_id.clone()).or_default().push(it);
    }
    let main_count = by_casket.get(&None).map(|v| v.len()).unwrap_or(0);
    let casket_groups: Vec<(String, usize)> = by_casket.iter()
        .filter_map(|(k, v)| k.as_ref().map(|cid| (cid.clone(), v.len())))
        .collect();
    tracing::info!(
        "inventory breakdown: {main_count} main, {} casket groups: {:?}",
        casket_groups.len(),
        casket_groups
    );
    let casket_summary: Vec<String> = items.iter()
        .filter(|i| i.is_storage_unit)
        .map(|i| format!(
            "[asset={} oid={:?} pos={} count={}]",
            i.id,
            i.original_id,
            i.position,
            i.storage_unit_item_count.unwrap_or(0)
        ))
        .collect();
    tracing::info!("storage units: {:?}", casket_summary);
}

fn debug_dump_inventory(items: &[Item]) {
    log_casket_breakdown(items);
    
    
    let Some(dirs) = directories::ProjectDirs::from("com", "cs2-inventory-manager", "cs2-im") else { return };
    let dir = dirs.data_dir().to_path_buf();
    if let Err(e) = std::fs::create_dir_all(&dir) {
        tracing::warn!("debug dump: mkdir failed: {e}");
        return;
    }
    let path = dir.join("inventory_debug.json");
    match serde_json::to_vec_pretty(items) {
        Ok(bytes) => {
            if let Err(e) = std::fs::write(&path, bytes) {
                tracing::warn!("debug dump write failed: {e}");
            } else {
                tracing::info!("debug dump → {}", path.display());
            }
        }
        Err(e) => tracing::warn!("debug dump serialize failed: {e}"),
    }

    
    
    let mut unresolved: std::collections::HashMap<u32, usize> = Default::default();
    for it in items {
        if it.name.starts_with("Item #") || it.name.starts_with("★ Item #") {
            *unresolved.entry(it.def_index).or_default() += 1;
        }
    }
    if unresolved.is_empty() {
        tracing::info!("name resolution: 0 unresolved items");
    } else {
        let mut entries: Vec<_> = unresolved.into_iter().collect();
        entries.sort_by(|a, b| b.1.cmp(&a.1));
        let summary: Vec<String> = entries.iter().take(10)
            .map(|(d, c)| format!("def={d}×{c}"))
            .collect();
        tracing::warn!("unresolved items ({} def_indexes): {}", entries.len(), summary.join(", "));
    }
}

fn spawn_so_listener(gc: Arc<GameCoordinator>, state: SharedState, app: AppHandle) {
    
    
    {
        let gc = gc.clone();
        let state = state.clone();
        let app = app.clone();
        tauri::async_runtime::spawn(async move {
            let stream = gc.on::<CMsgSOCacheSubscribed>();
            let mut stream = Box::pin(stream);
            while let Some(next) = stream.next().await {
                let Ok(msg) = next else { continue };
                let mut new_items = Vec::new();
                for kind in &msg.objects {
                    if kind.type_id() != 1 { continue; }
                    for data in &kind.object_data {
                        if let Ok(econ) = CSOEconItem::parse_from_bytes(data) {
                            new_items.push(econ);
                        }
                    }
                }
                apply_so_items(&state, &app, new_items, "CacheSubscribed").await;
            }
        });
    }

    
    
    
    spawn_single_listener::<SoSingleCreate>(gc.clone(), state.clone(), app.clone(), "Create",
        |w| (w.0.type_id(), w.0.object_data.clone().unwrap_or_default()));

    spawn_single_listener::<SoSingleUpdate>(gc.clone(), state.clone(), app.clone(), "Update",
        |w| (w.0.type_id(), w.0.object_data.clone().unwrap_or_default()));

    
    
    {
        let gc = gc.clone();
        let state = state.clone();
        let app = app.clone();
        tauri::async_runtime::spawn(async move {
            let stream = gc.on::<SoSingleDestroy>();
            let mut stream = Box::pin(stream);
            while let Some(next) = stream.next().await {
                let Ok(msg) = next else { continue };
                if msg.0.type_id() != 1 { continue; }
                let Some(data) = msg.0.object_data.as_ref() else { continue };
                let Ok(econ) = CSOEconItem::parse_from_bytes(data) else { continue };
                let id = econ.id().to_string();
                {
                    let mut inv = state.inventory.write().await;
                    inv.retain(|i| i.id != id);
                }
                let snapshot: Vec<Item> = state.inventory.read().await.iter()
                    .filter(|i| i.casket_id.is_none())
                    .cloned()
                    .collect();
                let _ = app.emit(INVENTORY_UPDATED, &snapshot);
                tracing::info!("SO Destroy: item {id} removed");
            }
        });
    }

    
    
    {
        let gc = gc.clone();
        let state = state.clone();
        let app = app.clone();
        tauri::async_runtime::spawn(async move {
            let stream = gc.on::<SoMultipleUpdate>();
            let mut stream = Box::pin(stream);
            while let Some(next) = stream.next().await {
                let Ok(msg) = next else { continue };
                let mut new_items = Vec::new();
                for obj in &msg.0.objects_modified {
                    if obj.type_id() != 1 { continue; }
                    if let Some(data) = obj.object_data.as_ref() {
                        if let Ok(econ) = CSOEconItem::parse_from_bytes(data) {
                            new_items.push(econ);
                        }
                    }
                }
                apply_so_items(&state, &app, new_items, "UpdateMultiple").await;
            }
        });
    }
}

fn spawn_single_listener<W>(gc: Arc<GameCoordinator>, state: SharedState, app: AppHandle, label: &'static str,
    extract: impl Fn(&W) -> (i32, Vec<u8>) + Send + Sync + 'static)
where
    W: steam_vent::message::NetMessage + 'static,
{
    tauri::async_runtime::spawn(async move {
        let stream = gc.on::<W>();
        let mut stream = Box::pin(stream);
        while let Some(next) = stream.next().await {
            let Ok(msg) = next else { continue };
            let (type_id, data) = extract(&msg);
            if type_id != 1 { continue; }
            let Ok(econ) = CSOEconItem::parse_from_bytes(&data) else { continue };
            apply_so_items(&state, &app, vec![econ], label).await;
        }
    });
}

async fn apply_so_items(state: &SharedState, app: &AppHandle, econ_items: Vec<CSOEconItem>, label: &str) {
    if econ_items.is_empty() { return; }
    let parsed = parser::convert_inventory(&econ_items).await;
    if parsed.is_empty() { return; }
    let in_casket = parsed.iter().filter(|i| i.casket_id.is_some()).count();
    tracing::info!("SO {label}: {} items ({} in caskets)", parsed.len(), in_casket);
    {
        let mut inv = state.inventory.write().await;
        for item in parsed {
            if let Some(existing) = inv.iter_mut().find(|i| i.id == item.id) {
                *existing = item;
            } else {
                inv.push(item);
            }
        }
    }
    let snapshot: Vec<Item> = state.inventory.read().await.iter()
        .filter(|i| i.casket_id.is_none())
        .cloned()
        .collect();
    let _ = app.emit(INVENTORY_UPDATED, &snapshot);
}

async fn parse_inventory_from_welcome(
    welcome: &steam_vent_proto_csgo::gcsdk_gcmessages::CMsgClientWelcome,
) -> Vec<Item> {
    let mut econ_items = Vec::new();
    for cache in &welcome.outofdate_subscribed_caches {
        for kind in &cache.objects {
            if kind.type_id() != 1 { continue; } 
            for data in &kind.object_data {
                match CSOEconItem::parse_from_bytes(data) {
                    Ok(econ) => econ_items.push(econ),
                    Err(e) => tracing::warn!("CSOEconItem decode error: {e}"),
                }
            }
        }
    }
    
    
    for econ in econ_items.iter() {
        if econ.def_index() == 1201 {
            tracing::info!(
                "STORAGE UNIT proto dump: id={} original_id={} inventory={} attrs={:?}",
                econ.id(),
                econ.original_id(),
                econ.inventory(),
                econ.attribute.iter()
                    .map(|a| format!(
                        "{}=[{:?}]/{:?}",
                        a.def_index(),
                        a.value_bytes().iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(""),
                        a.value
                    ))
                    .collect::<Vec<_>>()
            );
            break;
        }
    }
    for econ in econ_items.iter() {
        if econ.attribute.iter().any(|a| a.def_index() == 272) {
            tracing::info!(
                "CASKET ITEM proto dump: id={} original_id={} inventory={} def={} attrs={:?}",
                econ.id(),
                econ.original_id(),
                econ.inventory(),
                econ.def_index(),
                econ.attribute.iter()
                    .map(|a| format!(
                        "{}=[{:?}]/{:?}",
                        a.def_index(),
                        a.value_bytes().iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(""),
                        a.value
                    ))
                    .collect::<Vec<_>>()
            );
            break;
        }
    }
    parser::convert_inventory(&econ_items).await
}

pub async fn ensure_gc(session: &SteamSession) -> AppResult<Arc<GameCoordinator>> {
    session.gc_clone().await.ok_or(AppError::GcNotReady)
}
