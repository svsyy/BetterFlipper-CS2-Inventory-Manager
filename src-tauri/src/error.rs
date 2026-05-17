

use serde::{Serialize, Serializer};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("not logged in to Steam")]
    NotLoggedIn,

    #[error("Game Coordinator is not ready yet")]
    GcNotReady,

    #[error("Steam Guard code required")]
    SteamGuardRequired,

    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("operation timed out after {0:?}")]
    Timeout(std::time::Duration),

    #[error("storage unit is full")]
    StorageUnitFull,

    #[error("no saved refresh token for {0}")]
    NoSavedToken(String),

    #[error("keyring: {0}")]
    Keyring(String),

    #[error("io: {0}")]
    Io(String),

    #[error("http: {0}")]
    Http(String),

    #[error("steam protocol: {0}")]
    Protocol(String),

    #[error("{0}")]
    Other(String),
}

impl AppError {
    pub fn other(msg: impl Into<String>) -> Self {
        Self::Other(msg.into())
    }
}

impl From<keyring::Error> for AppError {
    fn from(e: keyring::Error) -> Self { Self::Keyring(e.to_string()) }
}
impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self { Self::Io(e.to_string()) }
}
impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self { Self::Http(e.to_string()) }
}
impl From<anyhow::Error> for AppError {
    fn from(e: anyhow::Error) -> Self { Self::Other(e.to_string()) }
}
impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self { Self::Other(format!("json: {e}")) }
}

impl Serialize for AppError {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;
