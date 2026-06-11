use crate::crypto::aes;
use crate::db::connection::Database;
use crate::error::AppError;
use crate::models::api_key::{ApiKey, ApiKeyView, NewApiKey, UpdateApiKey};

const SELECT_VIEW_COLS: &str =
    "k.id, k.provider_id, p.name, p.display_name, k.name, k.masked_preview, k.parent_id, k.created_at, k.updated_at";

fn row_to_view(row: &rusqlite::Row) -> rusqlite::Result<ApiKeyView> {
    Ok(ApiKeyView {
        id: row.get(0)?,
        provider_id: row.get(1)?,
        provider_name: row.get(2)?,
        provider_display_name: row.get(3)?,
        name: row.get(4)?,
        masked_preview: row.get(5)?,
        parent_id: row.get(6)?,
        created_at: row.get(7)?,
        updated_at: row.get(8)?,
    })
}

pub fn get_all_keys(db: &Database) -> Result<Vec<ApiKeyView>, AppError> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn
        .prepare(&format!(
            "SELECT {} FROM api_keys k JOIN providers p ON k.provider_id = p.id ORDER BY k.updated_at DESC",
            SELECT_VIEW_COLS,
        ))
        .map_err(AppError::Database)?;

    let rows = stmt
        .query_map([], row_to_view)
        .map_err(AppError::Database)?;

    let mut keys = Vec::new();
    for row in rows {
        keys.push(row.map_err(AppError::Database)?);
    }

    Ok(keys)
}

pub fn search_keys(db: &Database, query: &str) -> Result<Vec<ApiKeyView>, AppError> {
    let conn = db.conn.lock().unwrap();
    let search_pattern = format!("%{}%", query);
    let mut stmt = conn
        .prepare(&format!(
            "SELECT {} FROM api_keys k JOIN providers p ON k.provider_id = p.id WHERE (k.name LIKE ?1 OR p.display_name LIKE ?1 OR p.name LIKE ?1) ORDER BY k.updated_at DESC",
            SELECT_VIEW_COLS,
        ))
        .map_err(AppError::Database)?;

    let rows = stmt
        .query_map([&search_pattern], row_to_view)
        .map_err(AppError::Database)?;

    let mut keys = Vec::new();
    for row in rows {
        keys.push(row.map_err(AppError::Database)?);
    }

    Ok(keys)
}

pub fn get_keys_for_provider(db: &Database, provider_id: i64) -> Result<Vec<ApiKeyView>, AppError> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn
        .prepare(&format!(
            "SELECT {} FROM api_keys k JOIN providers p ON k.provider_id = p.id WHERE k.provider_id = ?1 ORDER BY k.created_at ASC",
            SELECT_VIEW_COLS,
        ))
        .map_err(AppError::Database)?;

    let rows = stmt
        .query_map([provider_id], row_to_view)
        .map_err(AppError::Database)?;

    let mut keys = Vec::new();
    for row in rows {
        keys.push(row.map_err(AppError::Database)?);
    }

    Ok(keys)
}

pub fn create_key(
    db: &Database,
    enc_key: &[u8; 32],
    new_key: &NewApiKey,
) -> Result<ApiKeyView, AppError> {
    let (ciphertext, iv) = aes::encrypt(enc_key, new_key.raw_key.as_bytes())?;
    let masked = aes::mask_api_key(&new_key.raw_key);

    let conn = db.conn.lock().unwrap();
    conn.execute(
        "INSERT INTO api_keys (provider_id, name, encrypted_key, iv, masked_preview, parent_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        rusqlite::params![
            new_key.provider_id,
            new_key.name,
            ciphertext,
            iv,
            masked,
            new_key.parent_id,
        ],
    )
    .map_err(AppError::Database)?;

    let id = conn.last_insert_rowid();

    let mut stmt = conn
        .prepare(&format!(
            "SELECT {} FROM api_keys k JOIN providers p ON k.provider_id = p.id WHERE k.id = ?1",
            SELECT_VIEW_COLS,
        ))
        .map_err(AppError::Database)?;

    let key = stmt
        .query_row([id], row_to_view)
        .map_err(AppError::Database)?;

    Ok(key)
}

pub fn update_key(
    db: &Database,
    enc_key: &[u8; 32],
    update: &UpdateApiKey,
) -> Result<ApiKeyView, AppError> {
    let conn = db.conn.lock().unwrap();

    if let Some(ref raw_key) = update.raw_key {
        let (ciphertext, iv) = aes::encrypt(enc_key, raw_key.as_bytes())?;
        let masked = aes::mask_api_key(raw_key);
        conn.execute(
            "UPDATE api_keys SET provider_id = ?1, name = ?2, encrypted_key = ?3, iv = ?4, masked_preview = ?5, updated_at = datetime('now') WHERE id = ?6",
            rusqlite::params![
                update.provider_id,
                update.name,
                ciphertext,
                iv,
                masked,
                update.id,
            ],
        )
        .map_err(AppError::Database)?;
    } else {
        conn.execute(
            "UPDATE api_keys SET provider_id = ?1, name = ?2, updated_at = datetime('now') WHERE id = ?3",
            rusqlite::params![
                update.provider_id,
                update.name,
                update.id,
            ],
        )
        .map_err(AppError::Database)?;
    }

    let key = conn
        .query_row(
            &format!(
                "SELECT {} FROM api_keys k JOIN providers p ON k.provider_id = p.id WHERE k.id = ?1",
                SELECT_VIEW_COLS,
            ),
            [update.id],
            row_to_view,
        )
        .map_err(AppError::Database)?;

    Ok(key)
}

pub fn delete_key(db: &Database, id: i64) -> Result<(), AppError> {
    let conn = db.conn.lock().unwrap();
    let affected = conn
        .execute("DELETE FROM api_keys WHERE id = ?1", [id])
        .map_err(AppError::Database)?;

    if affected == 0 {
        return Err(AppError::NotFound(format!(
            "Key with id {} not found",
            id
        )));
    }

    Ok(())
}

pub fn get_decrypted_key(
    db: &Database,
    enc_key: &[u8; 32],
    id: i64,
) -> Result<String, AppError> {
    let conn = db.conn.lock().unwrap();
    let result = conn.query_row(
        "SELECT encrypted_key, iv FROM api_keys WHERE id = ?1",
        [id],
        |row| Ok((row.get::<_, Vec<u8>>(0)?, row.get::<_, Vec<u8>>(1)?)),
    ).map_err(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => {
            AppError::NotFound(format!("Key with id {} not found", id))
        }
        other => AppError::Database(other),
    })?;

    let (ciphertext, iv) = result;
    let plaintext = aes::decrypt(enc_key, &ciphertext, &iv)?;
    let raw_key = String::from_utf8(plaintext)
        .map_err(|e| AppError::Crypto(format!("Invalid UTF-8 in decrypted key: {}", e)))?;

    Ok(raw_key)
}

pub fn get_key_by_id(db: &Database, id: i64) -> Result<ApiKey, AppError> {
    let conn = db.conn.lock().unwrap();
    let key = conn.query_row(
        "SELECT id, provider_id, name, encrypted_key, iv, masked_preview, parent_id, created_at, updated_at FROM api_keys WHERE id = ?1",
        [id],
        |row| {
            Ok(ApiKey {
                id: row.get(0)?,
                provider_id: row.get(1)?,
                name: row.get(2)?,
                encrypted_key: row.get(3)?,
                iv: row.get(4)?,
                masked_preview: row.get(5)?,
                parent_id: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
            })
        },
    ).map_err(|e| match e {
        rusqlite::Error::QueryReturnedNoRows => {
            AppError::NotFound(format!("Key with id {} not found", id))
        }
        other => AppError::Database(other),
    })?;

    Ok(key)
}
