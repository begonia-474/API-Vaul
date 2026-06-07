pub mod commands;
pub mod crypto;
pub mod db;
pub mod error;
pub mod models;
pub mod services;

use crypto::aes::EncryptionKey;
use db::connection::Database;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle().clone();
            let db = Database::new(&app_handle).expect("Failed to initialize database");
            let encryption_key = EncryptionKey::load_or_create(&app_handle);

            app.manage(db);
            app.manage(encryption_key);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::auth::setup_password,
            commands::auth::verify_password,
            commands::auth::is_first_run,
            commands::auth::change_password,
            commands::api_key::get_all_keys,
            commands::api_key::create_key,
            commands::api_key::update_key,
            commands::api_key::delete_key,
            commands::api_key::get_decrypted_key,
            commands::api_key::search_keys,
            commands::provider::get_all_providers,
            commands::settings::get_settings,
            commands::settings::update_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}