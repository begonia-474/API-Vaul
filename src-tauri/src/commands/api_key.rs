use tauri::State;

use crate::crypto::aes::EncryptionKey;
use crate::db::connection::Database;
use crate::error::AppError;
use crate::models::api_key::{ApiKeyView, NewApiKey, UpdateApiKey};
use crate::services::key_service;
use crate::AuthState;

fn require_unlocked(auth_state: &State<'_, AuthState>) -> Result<(), AppError> {
    if !*auth_state.unlocked.lock().unwrap() {
        return Err(AppError::AuthFailed);
    }
    Ok(())
}

#[tauri::command]
pub fn get_all_keys(
    db: State<'_, Database>,
    auth_state: State<'_, AuthState>,
) -> Result<Vec<ApiKeyView>, AppError> {
    require_unlocked(&auth_state)?;
    key_service::get_all_keys(&db)
}

#[tauri::command]
pub fn search_keys(
    query: String,
    db: State<'_, Database>,
    auth_state: State<'_, AuthState>,
) -> Result<Vec<ApiKeyView>, AppError> {
    require_unlocked(&auth_state)?;
    key_service::search_keys(&db, &query)
}

#[tauri::command]
pub fn create_key(
    new_key: NewApiKey,
    db: State<'_, Database>,
    enc: State<'_, EncryptionKey>,
    auth_state: State<'_, AuthState>,
) -> Result<ApiKeyView, AppError> {
    require_unlocked(&auth_state)?;
    if new_key.name.trim().is_empty() {
        return Err(AppError::InvalidInput("Key name cannot be empty".to_string()));
    }
    if new_key.raw_key.trim().is_empty() {
        return Err(AppError::InvalidInput("API key cannot be empty".to_string()));
    }
    key_service::create_key(&db, &enc.key, &new_key)
}

#[tauri::command]
pub fn update_key(
    update: UpdateApiKey,
    db: State<'_, Database>,
    enc: State<'_, EncryptionKey>,
    auth_state: State<'_, AuthState>,
) -> Result<ApiKeyView, AppError> {
    require_unlocked(&auth_state)?;
    if update.name.trim().is_empty() {
        return Err(AppError::InvalidInput("Key name cannot be empty".to_string()));
    }
    key_service::update_key(&db, &enc.key, &update)
}

#[tauri::command]
pub fn delete_key(
    id: i64,
    db: State<'_, Database>,
    auth_state: State<'_, AuthState>,
) -> Result<(), AppError> {
    require_unlocked(&auth_state)?;
    key_service::delete_key(&db, id)
}

#[tauri::command]
pub fn get_child_keys(
    parent_id: i64,
    db: State<'_, Database>,
    auth_state: State<'_, AuthState>,
) -> Result<Vec<ApiKeyView>, AppError> {
    require_unlocked(&auth_state)?;
    key_service::get_child_keys(&db, parent_id)
}

#[tauri::command]
pub fn get_decrypted_key(
    id: i64,
    db: State<'_, Database>,
    enc: State<'_, EncryptionKey>,
    auth_state: State<'_, AuthState>,
) -> Result<String, AppError> {
    require_unlocked(&auth_state)?;
    key_service::get_decrypted_key(&db, &enc.key, id)
}
