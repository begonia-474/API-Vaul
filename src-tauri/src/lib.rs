pub mod commands;
pub mod crypto;
pub mod db;
pub mod error;
pub mod models;
pub mod services;

use std::sync::Mutex;

use crypto::aes::EncryptionKey;
use db::connection::Database;
use tauri::Manager;

pub struct AuthState {
    pub unlocked: Mutex<bool>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle().clone();
            let db = Database::new(&app_handle).expect("Failed to initialize database");
            let encryption_key = EncryptionKey::load_or_create(&app_handle);

            app.manage(db);
            app.manage(encryption_key);
            app.manage(AuthState { unlocked: Mutex::new(false) });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::auth::setup_password,
            commands::auth::verify_password,
            commands::auth::is_first_run,
            commands::auth::change_password,
            commands::auth::lock_app,
            commands::api_key::get_all_keys,
            commands::api_key::create_key,
            commands::api_key::update_key,
            commands::api_key::delete_key,
            commands::api_key::get_decrypted_key,
            commands::api_key::search_keys,
            commands::provider::create_provider,
            commands::provider::get_all_providers,
            commands::provider::get_providers_with_keys,
            commands::provider::get_provider_keys,
            commands::provider::update_provider_metadata,
            commands::settings::get_settings,
            commands::settings::update_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}