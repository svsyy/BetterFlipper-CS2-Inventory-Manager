

use serde::Serialize;
use tauri::{AppHandle, Emitter};

pub const LOGIN_STATUS: &str = "steam://login-status";
pub const INVENTORY_UPDATED: &str = "inventory://updated";
pub const INVENTORY_ITEM_DELTA: &str = "inventory://item-delta";
pub const CASKET_TRANSFER_PROGRESS: &str = "casket://transfer-progress";
pub const CASKET_CONTENTS: &str = "casket://contents";
pub const ERROR_TOAST: &str = "app://error";

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum LoginStage {
    Idle,
    Connecting,
    Authenticating,
    AwaitingGuard,
    LoggingIn,
    WaitingGc,
    LoadingInventory,
    Ready,
    Error,
}

#[derive(Serialize, Clone, Debug)]
pub struct LoginStatusPayload {
    pub stage: LoginStage,
    pub message: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "kebab-case", tag = "kind")]
pub enum ItemDelta {
    Acquired { item_id: String },
    Removed { item_id: String },
    Changed { item_id: String },
}

#[derive(Serialize, Clone, Debug)]
pub struct TransferProgress {
    pub done: usize,
    pub total: usize,
    pub current_item: Option<String>,
    pub failed: usize,
}

pub fn emit_login_status(app: &AppHandle, stage: LoginStage, message: Option<String>) {
    let _ = app.emit(LOGIN_STATUS, LoginStatusPayload { stage, message });
}
