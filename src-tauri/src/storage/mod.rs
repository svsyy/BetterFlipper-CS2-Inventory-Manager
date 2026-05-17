

use std::time::Duration;

use steam_vent::ConnectionTrait;
use steam_vent_proto_common::protobuf::Enum as ProtoEnum;
use steam_vent_proto_csgo::econ_gcmessages::{
    CMsgCasketItem, CMsgGCItemCustomizationNotification, EGCItemMsg,
    EGCItemCustomizationNotification,
};
use futures_util::StreamExt;

use crate::error::{AppError, AppResult};
use crate::inventory::Item;
use crate::state::SharedState;
use crate::steam::gc_msg;

const OPERATION_TIMEOUT: Duration = Duration::from_secs(10);

pub async fn list_caskets(state: &SharedState) -> AppResult<Vec<Item>> {
    let inv = state.inventory.read().await;
    Ok(inv.iter().filter(|i| i.is_storage_unit).cloned().collect())
}

pub async fn casket_contents(state: &SharedState, casket_id: &str) -> AppResult<Vec<Item>> {
    
    
    
    
    let target_oid: String = casket_id.to_string();

    
    {
        let inv = state.inventory.read().await;
        let cached: Vec<Item> = inv.iter()
            .filter(|i| i.casket_id.as_deref() == Some(target_oid.as_str()))
            .cloned()
            .collect();
        if !cached.is_empty() {
            return Ok(cached);
        }
    }

    
    
    
    
    
    
    
    
    
    
    let session_guard = state.session.read().await;
    let session = session_guard.as_ref().ok_or(AppError::NotLoggedIn)?;
    let gc = session.gc_clone().await.ok_or(AppError::GcNotReady)?;
    drop(session_guard);
    let casket_u64: u64 = casket_id.parse().map_err(|_| AppError::other("invalid casket id"))?;

    let mut body = CMsgCasketItem::new();
    body.set_casket_item_id(casket_u64);
    body.set_item_item_id(0);
    gc_msg::send_proto(&gc, EGCItemMsg::k_EMsgGCCasketItemLoadContents, body)
        .await
        .map_err(|e| AppError::Protocol(format!("LoadContents failed: {e}")))?;
    tracing::info!("sent LoadContents for casket {casket_id}, waiting for SO updates…");

    
    
    let deadline = tokio::time::Instant::now() + OPERATION_TIMEOUT;
    loop {
        tokio::time::sleep(std::time::Duration::from_millis(250)).await;

        let inv = state.inventory.read().await;
        let matching: Vec<Item> = inv.iter()
            .filter(|i| i.casket_id.as_deref() == Some(target_oid.as_str()))
            .cloned()
            .collect();
        if !matching.is_empty() {
            tracing::info!("casket {casket_id} (oid={target_oid}): {} items loaded", matching.len());
            return Ok(matching);
        }
        if tokio::time::Instant::now() >= deadline {
            tracing::info!("casket {casket_id} (oid={target_oid}): empty or timed out");
            return Ok(Vec::new());
        }
    }
}

pub async fn add_to_casket(
    state: &SharedState,
    casket_id: &str,
    item_id: &str,
) -> AppResult<()> {
    let session_guard = state.session.read().await;
    let session = session_guard.as_ref().ok_or(AppError::NotLoggedIn)?;
    let _guard = session.acquire_op_lock().await;
    let gc = session.gc_clone().await.ok_or(AppError::GcNotReady)?;

    let casket_u64: u64 = casket_id.parse().map_err(|_| AppError::other("invalid casket id"))?;
    let item_u64: u64 = item_id.parse().map_err(|_| AppError::other("invalid item id"))?;
    let target_oid: String = casket_id.to_string();

    
    
    let stream = gc.on::<CMsgGCItemCustomizationNotification>();

    let mut body = CMsgCasketItem::new();
    body.set_casket_item_id(casket_u64);
    body.set_item_item_id(item_u64);
    gc_msg::send_proto(&gc, EGCItemMsg::k_EMsgGCCasketItemAdd, body)
        .await
        .map_err(|e| AppError::Protocol(format!("CasketItemAdd failed: {e}")))?;

    wait_for_notification(
        stream,
        casket_u64,
        &[
            EGCItemCustomizationNotification::k_EGCItemCustomizationNotification_CasketAdded,
            EGCItemCustomizationNotification::k_EGCItemCustomizationNotification_CasketInvFull,
            EGCItemCustomizationNotification::k_EGCItemCustomizationNotification_CasketTooFull,
        ],
    )
    .await?;

    update_item_casket_id(state, item_id, Some(target_oid.clone())).await;
    Ok(())
}

pub async fn remove_from_casket(
    state: &SharedState,
    casket_id: &str,
    item_id: &str,
) -> AppResult<()> {
    let session_guard = state.session.read().await;
    let session = session_guard.as_ref().ok_or(AppError::NotLoggedIn)?;
    let _guard = session.acquire_op_lock().await;
    let gc = session.gc_clone().await.ok_or(AppError::GcNotReady)?;

    let casket_u64: u64 = casket_id.parse().map_err(|_| AppError::other("invalid casket id"))?;
    let item_u64: u64 = item_id.parse().map_err(|_| AppError::other("invalid item id"))?;

    let stream = gc.on::<CMsgGCItemCustomizationNotification>();

    let mut body = CMsgCasketItem::new();
    body.set_casket_item_id(casket_u64);
    body.set_item_item_id(item_u64);
    gc_msg::send_proto(&gc, EGCItemMsg::k_EMsgGCCasketItemExtract, body)
        .await
        .map_err(|e| AppError::Protocol(format!("CasketItemExtract failed: {e}")))?;

    wait_for_notification(
        stream,
        casket_u64,
        &[EGCItemCustomizationNotification::k_EGCItemCustomizationNotification_CasketRemoved],
    )
    .await?;

    update_item_casket_id(state, item_id, None).await;
    Ok(())
}

pub async fn rename_casket(
    state: &SharedState,
    casket_id: &str,
    new_name: &str,
) -> AppResult<()> {
    if new_name.is_empty() || new_name.len() > 40 {
        return Err(AppError::other("name must be 1–40 characters"));
    }
    let session_guard = state.session.read().await;
    let session = session_guard.as_ref().ok_or(AppError::NotLoggedIn)?;
    let _guard = session.acquire_op_lock().await;
    let gc = session.gc_clone().await.ok_or(AppError::GcNotReady)?;

    let casket_u64: u64 = casket_id.parse().map_err(|_| AppError::other("invalid casket id"))?;

    let stream = gc.on::<CMsgGCItemCustomizationNotification>();

    
    
    
    
    
    
    let name_bytes = new_name.as_bytes();
    let mut body = Vec::with_capacity(8 + 8 + 1 + name_bytes.len() + 1);
    body.extend_from_slice(&0u64.to_le_bytes());        
    body.extend_from_slice(&casket_u64.to_le_bytes());  
    body.push(0x00);
    body.extend_from_slice(name_bytes);
    body.push(0x00);
    gc_msg::send_raw(&gc, EGCItemMsg::k_EMsgGCNameItem, body)
        .await
        .map_err(|e| AppError::Protocol(format!("NameItem failed: {e}")))?;

    wait_for_notification(
        stream,
        casket_u64,
        &[EGCItemCustomizationNotification::k_EGCItemCustomizationNotification_NameItem],
    )
    .await?;

    update_item_custom_name(state, casket_id, new_name.to_string()).await;
    Ok(())
}

async fn wait_for_notification<S>(
    stream: S,
    target_item_id: u64,
    accepted: &[EGCItemCustomizationNotification],
) -> AppResult<()>
where
    S: futures_util::Stream<Item = Result<CMsgGCItemCustomizationNotification, steam_vent::NetworkError>>,
{
    let mut stream = Box::pin(stream);
    let deadline = tokio::time::sleep(OPERATION_TIMEOUT);
    tokio::pin!(deadline);

    loop {
        tokio::select! {
            _ = &mut deadline => {
                return Err(AppError::Protocol("GC notification timeout".into()));
            }
            next = stream.next() => {
                let Some(msg) = next else {
                    return Err(AppError::Protocol("GC stream ended".into()));
                };
                let Ok(notif) = msg else { continue };
                let request_kind = match <EGCItemCustomizationNotification as ProtoEnum>::from_i32(notif.request() as i32) {
                    Some(k) => k,
                    None => continue,
                };
                let mentions_target = notif.item_id.is_empty()
                    || notif.item_id.iter().any(|id| *id == target_item_id);
                if !mentions_target { continue; }

                use EGCItemCustomizationNotification as N;
                match request_kind {
                    N::k_EGCItemCustomizationNotification_CasketInvFull
                    | N::k_EGCItemCustomizationNotification_CasketTooFull
                        if accepted.contains(&request_kind) => {
                        return Err(AppError::other("Storage unit is full"));
                    }
                    k if accepted.contains(&k) => {
                        return Ok(());
                    }
                    _ => continue,
                }
            }
        }
    }
}

async fn update_item_casket_id(state: &SharedState, item_id: &str, new_casket: Option<String>) {
    let mut inv = state.inventory.write().await;
    if let Some(it) = inv.iter_mut().find(|i| i.id == item_id) {
        it.casket_id = new_casket;
    }
}

async fn update_item_custom_name(state: &SharedState, casket_id: &str, new_name: String) {
    let mut inv = state.inventory.write().await;
    if let Some(it) = inv.iter_mut().find(|i| i.id == casket_id) {
        it.custom_name = Some(new_name);
    }
}

