use tauri::State;

use crate::db::connection::Database;
use crate::error::AppError;
use crate::models::provider::Provider;
use crate::services::provider_service;

#[tauri::command]
pub fn get_all_providers(db: State<'_, Database>) -> Result<Vec<Provider>, AppError> {
    provider_service::get_all_providers(&db)
}
