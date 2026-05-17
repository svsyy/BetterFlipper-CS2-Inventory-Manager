

use std::sync::Arc;
use steam_vent::{Connection, GameCoordinator};
use tokio::sync::{Mutex, OwnedMutexGuard};

pub const CS2_APPID: u32 = 730;

pub struct SteamSession {
    pub username: String,
    pub steam_id: u64,
    pub refresh_token: String,
    
    
    pub conn: Arc<Connection>,
    
    pub gc: Mutex<Option<Arc<GameCoordinator>>>,
    
    
    
    
    pub gc_op_lock: Arc<Mutex<()>>,
}

impl SteamSession {
    pub fn new(conn: Arc<Connection>, username: String, refresh_token: String) -> Self {
        let steam_id = u64::from(conn.steam_id());
        Self {
            username,
            steam_id,
            refresh_token,
            conn,
            gc: Mutex::new(None),
            gc_op_lock: Arc::new(Mutex::new(())),
        }
    }

    pub async fn gc_ready(&self) -> bool {
        self.gc.lock().await.is_some()
    }

    pub async fn gc_clone(&self) -> Option<Arc<GameCoordinator>> {
        self.gc.lock().await.as_ref().map(Arc::clone)
    }

    pub async fn set_gc(&self, gc: GameCoordinator) {
        *self.gc.lock().await = Some(Arc::new(gc));
    }

    pub async fn acquire_op_lock(&self) -> OwnedMutexGuard<()> {
        Arc::clone(&self.gc_op_lock).lock_owned().await
    }
}
