

pub mod commands;
pub mod error;
pub mod events;
pub mod inventory;
pub mod persona;
pub mod secrets;
pub mod state;
pub mod steam;
pub mod storage;

pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info,cs2_im_lib=debug")),
        )
        .with_target(false)
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .manage(state::new_shared_state())
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = inventory::resolver::initialize().await {
                    tracing::warn!("item resolver initialize failed: {e}");
                }
                let _ = handle;
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::steam_session_status,
            commands::steam_login,
            commands::steam_login_with_token,
            commands::steam_logout,
            commands::list_saved_accounts,
            commands::inventory_get,
            commands::inventory_refresh,
            commands::casket_list,
            commands::casket_contents,
            commands::casket_add,
            commands::casket_remove,
            commands::casket_rename,
            commands::persona_get,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
