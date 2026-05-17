

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use steam_vent::auth::{
    AuthConfirmationHandler, DeviceConfirmationHandler, FileGuardDataStore,
    SharedSecretAuthConfirmationHandler, UserProvidedAuthConfirmationHandler,
};
use steam_vent::{Connection, ServerList};
use tokio::io::AsyncWriteExt;

use crate::error::{AppError, AppResult};
use crate::steam::session::SteamSession;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CredentialsRequest {
    pub username: String,
    pub password: String,
    pub steam_guard_code: Option<String>,
    pub shared_secret: Option<String>,
    pub remember: bool,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoginResult {
    pub steam_id: String,
    pub username: String,
    pub persona_name: Option<String>,
    pub remembered: bool,
}

async fn handler_with_code(code: &str) -> impl AuthConfirmationHandler {
    let (mut tx, rx) = tokio::io::duplex(256);
    let line = format!("{}\n", code.trim());
    let _ = tx.write_all(line.as_bytes()).await;
    let _ = tx.shutdown().await;
    drop(tx);
    UserProvidedAuthConfirmationHandler::new(rx, tokio::io::sink())
}

pub async fn login_credentials(req: CredentialsRequest) -> AppResult<(SteamSession, LoginResult)> {
    let server_list = ServerList::discover()
        .await
        .map_err(|e| AppError::Protocol(format!("server discovery failed: {e}")))?;

    let guard_store = FileGuardDataStore::user_cache();

    let conn = match (req.shared_secret.as_deref(), req.steam_guard_code.as_deref()) {
        (Some(secret), _) if !secret.is_empty() => {
            Connection::login(
                &server_list,
                &req.username,
                &req.password,
                guard_store,
                SharedSecretAuthConfirmationHandler::new(secret)
                    .or(DeviceConfirmationHandler),
            )
            .await
        }
        (_, Some(code)) if !code.is_empty() => {
            let handler = handler_with_code(code).await;
            Connection::login(
                &server_list,
                &req.username,
                &req.password,
                guard_store,
                handler.or(DeviceConfirmationHandler),
            )
            .await
        }
        _ => {
            
            Connection::login(
                &server_list,
                &req.username,
                &req.password,
                guard_store,
                DeviceConfirmationHandler,
            )
            .await
        }
    }
    .map_err(map_login_err)?;

    let refresh_token = conn.access_token().unwrap_or_default().to_string();
    let conn = Arc::new(conn);
    let session = SteamSession::new(conn, req.username.clone(), refresh_token);

    let result = LoginResult {
        steam_id: session.steam_id.to_string(),
        username: req.username,
        persona_name: None,
        remembered: req.remember,
    };
    Ok((session, result))
}

pub async fn login_with_refresh_token(
    username: &str,
    token: &str,
) -> AppResult<(SteamSession, LoginResult)> {
    let server_list = ServerList::discover()
        .await
        .map_err(|e| AppError::Protocol(format!("server discovery failed: {e}")))?;

    let conn = Connection::access(&server_list, username, token)
        .await
        .map_err(map_login_err)?;

    let refresh_token = conn.access_token().unwrap_or(token).to_string();
    let conn = Arc::new(conn);
    let session = SteamSession::new(conn, username.to_string(), refresh_token);

    let result = LoginResult {
        steam_id: session.steam_id.to_string(),
        username: username.to_string(),
        persona_name: None,
        remembered: true,
    };
    Ok((session, result))
}

fn map_login_err(e: impl std::fmt::Display) -> AppError {
    let msg = e.to_string();
    let lc = msg.to_lowercase();
    if lc.contains("invalid password") || lc.contains("invalid credentials") {
        AppError::InvalidCredentials
    } else if lc.contains("guard") || lc.contains("two-factor") || lc.contains("two_factor") {
        AppError::SteamGuardRequired
    } else {
        AppError::Protocol(msg)
    }
}
