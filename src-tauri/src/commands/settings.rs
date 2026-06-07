use tauri::State;

use crate::db::connection::Database;
use crate::error::AppError;
use crate::models::settings::{AppSettings, UpdateSettings};

#[tauri::command]
pub fn get_settings(db: State<'_, Database>) -> Result<AppSettings, AppError> {
    let conn = db.conn.lock().unwrap();
    let settings = conn
        .query_row(
            "SELECT id, password_hash, theme, auto_lock_minutes, language, created_at, updated_at FROM settings WHERE id = 1",
            [],
            |row| {
                Ok(AppSettings {
                    id: row.get(0)?,
                    password_hash: row.get(1)?,
                    theme: row.get(2)?,
                    auto_lock_minutes: row.get(3)?,
                    language: row.get(4)?,
                    created_at: row.get(5)?,
                    updated_at: row.get(6)?,
                })
            },
        )
        .map_err(AppError::Database)?;

    Ok(settings)
}

#[tauri::command]
pub fn update_settings(
    update: UpdateSettings,
    db: State<'_, Database>,
) -> Result<AppSettings, AppError> {
    let conn = db.conn.lock().unwrap();

    if let Some(ref theme) = update.theme {
        if theme != "dark" && theme != "light" && theme != "system" {
            return Err(AppError::InvalidInput(
                "Theme must be 'dark', 'light', or 'system'".to_string(),
            ));
        }
        conn.execute(
            "UPDATE settings SET theme = ?1, updated_at = datetime('now') WHERE id = 1",
            [theme],
        )
        .map_err(AppError::Database)?;
    }

    if let Some(minutes) = update.auto_lock_minutes {
        if minutes < 1 || minutes > 60 {
            return Err(AppError::InvalidInput(
                "Auto-lock minutes must be between 1 and 60".to_string(),
            ));
        }
        conn.execute(
            "UPDATE settings SET auto_lock_minutes = ?1, updated_at = datetime('now') WHERE id = 1",
            [minutes],
        )
        .map_err(AppError::Database)?;
    }

    if let Some(ref language) = update.language {
        conn.execute(
            "UPDATE settings SET language = ?1, updated_at = datetime('now') WHERE id = 1",
            [language],
        )
        .map_err(AppError::Database)?;
    }

    let settings = conn
        .query_row(
            "SELECT id, password_hash, theme, auto_lock_minutes, language, created_at, updated_at FROM settings WHERE id = 1",
            [],
            |row| {
                Ok(AppSettings {
                    id: row.get(0)?,
                    password_hash: row.get(1)?,
                    theme: row.get(2)?,
                    auto_lock_minutes: row.get(3)?,
                    language: row.get(4)?,
                    created_at: row.get(5)?,
                    updated_at: row.get(6)?,
                })
            },
        )
        .map_err(AppError::Database)?;

    Ok(settings)
}