use tauri::State;

use crate::db::connection::Database;
use crate::error::AppError;
use crate::models::api_key::ApiKeyView;
use crate::models::provider::Provider;
use crate::services::key_service;
use crate::services::provider_service;
use crate::AuthState;

fn require_unlocked(auth_state: &State<'_, AuthState>) -> Result<(), AppError> {
    if !*auth_state.unlocked.lock().unwrap() {
        return Err(AppError::AuthFailed);
    }
    Ok(())
}

#[tauri::command]
pub fn get_all_providers(db: State<'_, Database>) -> Result<Vec<Provider>, AppError> {
    provider_service::get_all_providers(&db)
}

#[tauri::command]
pub fn create_provider(
    name: String,
    display_name: String,
    icon: Option<String>,
    category: String,
    openai_base_url: Option<String>,
    anthropic_base_url: Option<String>,
    description: Option<String>,
    db: State<'_, Database>,
    auth_state: State<'_, AuthState>,
) -> Result<Provider, AppError> {
    require_unlocked(&auth_state)?;
    if name.trim().is_empty() {
        return Err(AppError::InvalidInput("Provider name cannot be empty".to_string()));
    }
    if display_name.trim().is_empty() {
        return Err(AppError::InvalidInput("Display name cannot be empty".to_string()));
    }
    provider_service::create_provider(
        &db,
        &name,
        &display_name,
        icon.as_deref(),
        &category,
        openai_base_url.as_deref(),
        anthropic_base_url.as_deref(),
        description.as_deref(),
    )
}

#[tauri::command]
pub fn get_providers_with_keys(
    db: State<'_, Database>,
    auth_state: State<'_, AuthState>,
) -> Result<Vec<Provider>, AppError> {
    require_unlocked(&auth_state)?;
    provider_service::get_providers_with_keys(&db)
}

#[tauri::command]
pub fn get_provider_keys(
    provider_id: i64,
    db: State<'_, Database>,
    auth_state: State<'_, AuthState>,
) -> Result<Vec<ApiKeyView>, AppError> {
    require_unlocked(&auth_state)?;
    key_service::get_keys_for_provider(&db, provider_id)
}

#[tauri::command]
pub fn update_provider_metadata(
    id: i64,
    openai_base_url: Option<String>,
    anthropic_base_url: Option<String>,
    description: Option<String>,
    db: State<'_, Database>,
    auth_state: State<'_, AuthState>,
) -> Result<Provider, AppError> {
    require_unlocked(&auth_state)?;
    provider_service::update_provider_metadata(
        &db,
        id,
        openai_base_url.as_deref(),
        anthropic_base_url.as_deref(),
        description.as_deref(),
    )
}
