use tauri::State;

use crate::crypto::password;
use crate::db::connection::Database;
use crate::error::AppError;
use crate::AuthState;

fn require_unlocked(auth_state: &State<'_, AuthState>) -> Result<(), AppError> {
    if !*auth_state.unlocked.lock().unwrap() {
        return Err(AppError::AuthFailed);
    }
    Ok(())
}

#[tauri::command]
pub fn is_first_run(db: State<'_, Database>) -> Result<bool, AppError> {
    let conn = db.conn.lock().unwrap();
    let hash: Option<String> = conn
        .query_row(
            "SELECT password_hash FROM settings WHERE id = 1",
            [],
            |row| row.get(0),
        )
        .map_err(AppError::Database)?;

    Ok(hash.is_none())
}

#[tauri::command]
pub fn setup_password(
    password_str: String,
    db: State<'_, Database>,
    auth_state: State<'_, AuthState>,
) -> Result<bool, AppError> {
    eprintln!("[setup_password] called with len={}", password_str.len());

    if password_str.len() < 6 {
        return Err(AppError::InvalidInput(
            "Password must be at least 6 characters".to_string(),
        ));
    }

    let hash = match password::hash_password(&password_str) {
        Ok(h) => {
            eprintln!("[setup_password] hash generated, len={}", h.len());
            h
        }
        Err(e) => {
            eprintln!("[setup_password] hash failed: {}", e);
            return Err(e);
        }
    };

    let conn = db.conn.lock().unwrap();
    let affected = conn
        .execute(
            "UPDATE settings SET password_hash = ?1, updated_at = datetime('now') WHERE id = 1",
            [&hash],
        )
        .map_err(|e| {
            eprintln!("[setup_password] db update failed: {}", e);
            AppError::Database(e)
        })?;

    eprintln!("[setup_password] updated {} rows", affected);
    *auth_state.unlocked.lock().unwrap() = true;
    Ok(true)
}

#[tauri::command]
pub fn verify_password(
    password_str: String,
    db: State<'_, Database>,
    auth_state: State<'_, AuthState>,
) -> Result<bool, AppError> {
    let conn = db.conn.lock().unwrap();
    let hash: Option<String> = conn
        .query_row(
            "SELECT password_hash FROM settings WHERE id = 1",
            [],
            |row| row.get(0),
        )
        .map_err(AppError::Database)?;

    let result = match hash {
        Some(h) => password::verify_password(&password_str, &h)?,
        None => {
            return Err(AppError::InvalidInput(
                "Password not set up yet".to_string(),
            ));
        }
    };

    if result {
        *auth_state.unlocked.lock().unwrap() = true;
    }
    Ok(result)
}

#[tauri::command]
pub fn lock_app(auth_state: State<'_, AuthState>) -> Result<(), AppError> {
    *auth_state.unlocked.lock().unwrap() = false;
    Ok(())
}

#[tauri::command]
pub fn change_password(
    old_password: String,
    new_password: String,
    db: State<'_, Database>,
    auth_state: State<'_, AuthState>,
) -> Result<bool, AppError> {
    require_unlocked(&auth_state)?;
    if new_password.len() < 6 {
        return Err(AppError::InvalidInput(
            "New password must be at least 6 characters".to_string(),
        ));
    }

    let conn = db.conn.lock().unwrap();
    let hash: Option<String> = conn
        .query_row(
            "SELECT password_hash FROM settings WHERE id = 1",
            [],
            |row| row.get(0),
        )
        .map_err(AppError::Database)?;

    let hash = hash.ok_or_else(|| AppError::InvalidInput("Password not set up yet".to_string()))?;

    if !password::verify_password(&old_password, &hash)? {
        return Err(AppError::AuthFailed);
    }

    drop(conn);

    let new_hash = password::hash_password(&new_password)?;
    let conn = db.conn.lock().unwrap();
    conn.execute(
        "UPDATE settings SET password_hash = ?1, updated_at = datetime('now') WHERE id = 1",
        [&new_hash],
    )
    .map_err(AppError::Database)?;

    Ok(true)
}