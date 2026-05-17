

use std::sync::Arc;
use tokio::sync::RwLock;

use crate::inventory::item::Item;
use crate::steam::session::SteamSession;

#[derive(Default)]
pub struct AppState {
    
    pub session: RwLock<Option<SteamSession>>,
    
    pub inventory: RwLock<Vec<Item>>,
}

pub type SharedState = Arc<AppState>;

pub fn new_shared_state() -> SharedState {
    Arc::new(AppState::default())
}
