

use serde::Serialize;
use tauri::{AppHandle, State};

use crate::error::{AppError, AppResult};
use crate::events::{emit_login_status, LoginStage};
use crate::inventory::Item;
use crate::persona::{self, Persona};
use crate::secrets;
use crate::state::SharedState;
use crate::steam::auth::{self, CredentialsRequest, LoginResult};
use crate::steam::gc::start_cs2;
use crate::storage;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionStatus {
    pub logged_in: bool,
    pub username: Option<String>,
    pub steam_id: Option<String>,
    pub gc_ready: bool,
    pub inventory_count: usize,
    pub saved_accounts: Vec<String>,
}

#[tauri::command]
pub async fn steam_session_status(state: State<'_, SharedState>) -> AppResult<SessionStatus> {
    let session = state.session.read().await;
    let inv = state.inventory.read().await;
    let (logged_in, username, steam_id, gc_ready) = match session.as_ref() {
        Some(s) => (true, Some(s.username.clone()), Some(s.steam_id.to_string()), s.gc_ready().await),
        None => (false, None, None, false),
    };
    Ok(SessionStatus {
        logged_in,
        username,
        steam_id,
        gc_ready,
        inventory_count: inv.len(),
        saved_accounts: secrets::list_saved_accounts().unwrap_or_default(),
    })
}

#[tauri::command]
pub async fn steam_login(
    req: CredentialsRequest,
    state: State<'_, SharedState>,
    app: AppHandle,
) -> AppResult<LoginResult> {
    emit_login_status(&app, LoginStage::Connecting, Some("Discovering Steam servers".into()));

    let remember = req.remember;
    let (session, result) = auth::login_credentials(req).await?;

    if remember && !session.refresh_token.is_empty() {
        secrets::save_refresh_token(&session.username, &session.refresh_token)?;
    }

    
    *state.session.write().await = Some(session);

    
    
    let app_handle = app.clone();
    let state_for_gc = state.inner().clone();
    tauri::async_runtime::spawn(async move {
        let guard = state_for_gc.session.read().await;
        if let Some(session) = guard.as_ref() {
            match start_cs2(session, &state_for_gc, &app_handle).await {
                Ok(()) => emit_login_status(&app_handle, LoginStage::Ready, None),
                Err(e) => emit_login_status(
                    &app_handle,
                    LoginStage::Error,
                    Some(format!("GC start failed: {e}")),
                ),
            }
        }
    });

    Ok(result)
}

#[tauri::command]
pub async fn steam_login_with_token(
    username: String,
    state: State<'_, SharedState>,
    app: AppHandle,
) -> AppResult<LoginResult> {
    emit_login_status(&app, LoginStage::Connecting, None);

    let token = secrets::load_refresh_token(&username)?
        .ok_or_else(|| AppError::NoSavedToken(username.clone()))?;
    let (session, result) = auth::login_with_refresh_token(&username, &token).await?;

    
    if !session.refresh_token.is_empty() && session.refresh_token != token {
        secrets::save_refresh_token(&username, &session.refresh_token).ok();
    }

    *state.session.write().await = Some(session);

    let app_handle = app.clone();
    let state_for_gc = state.inner().clone();
    tauri::async_runtime::spawn(async move {
        let guard = state_for_gc.session.read().await;
        if let Some(session) = guard.as_ref() {
            match start_cs2(session, &state_for_gc, &app_handle).await {
                Ok(()) => emit_login_status(&app_handle, LoginStage::Ready, None),
                Err(e) => emit_login_status(
                    &app_handle,
                    LoginStage::Error,
                    Some(format!("GC start failed: {e}")),
                ),
            }
        }
    });

    Ok(result)
}

#[tauri::command]
pub async fn steam_logout(state: State<'_, SharedState>) -> AppResult<()> {
    let session = state.session.write().await.take();
    if let Some(s) = session {
        secrets::forget(&s.username).ok();
    }
    state.inventory.write().await.clear();
    Ok(())
}

#[tauri::command]
pub async fn list_saved_accounts() -> AppResult<Vec<String>> {
    secrets::list_saved_accounts()
}

#[tauri::command]
pub async fn inventory_get(state: State<'_, SharedState>) -> AppResult<Vec<Item>> {
    
    
    let inv = state.inventory.read().await;
    Ok(inv.iter().filter(|i| i.casket_id.is_none()).cloned().collect())
}

#[tauri::command]
pub async fn inventory_refresh(state: State<'_, SharedState>, app: AppHandle) -> AppResult<usize> {
    let session_guard = state.session.read().await;
    let session = session_guard.as_ref().ok_or(AppError::NotLoggedIn)?;
    start_cs2(session, &state, &app).await?;
    Ok(state.inventory.read().await.len())
}

#[tauri::command]
pub async fn casket_list(state: State<'_, SharedState>) -> AppResult<Vec<Item>> {
    storage::list_caskets(&state).await
}

#[tauri::command]
pub async fn casket_contents(state: State<'_, SharedState>, casket_id: String) -> AppResult<Vec<Item>> {
    storage::casket_contents(&state, &casket_id).await
}

#[tauri::command]
pub async fn casket_add(state: State<'_, SharedState>, casket_id: String, item_id: String) -> AppResult<()> {
    storage::add_to_casket(&state, &casket_id, &item_id).await
}

#[tauri::command]
pub async fn casket_remove(state: State<'_, SharedState>, casket_id: String, item_id: String) -> AppResult<()> {
    storage::remove_from_casket(&state, &casket_id, &item_id).await
}

#[tauri::command]
pub async fn casket_rename(state: State<'_, SharedState>, casket_id: String, new_name: String) -> AppResult<()> {
    storage::rename_casket(&state, &casket_id, &new_name).await
}

#[tauri::command]
pub async fn persona_get(state: State<'_, SharedState>) -> AppResult<Persona> {
    let guard = state.session.read().await;
    let session = guard.as_ref().ok_or(AppError::NotLoggedIn)?;
    persona::fetch(session.steam_id).await
}
