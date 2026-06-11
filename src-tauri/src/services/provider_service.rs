use crate::db::connection::Database;
use crate::error::AppError;
use crate::models::provider::Provider;

const SELECT_COLUMNS: &str = "id, name, display_name, icon, base_url, api_type, compat_type, category, website_url, api_key_url, preset_id, openai_base_url, anthropic_base_url, description, created_at";

fn map_provider(row: &rusqlite::Row<'_>) -> rusqlite::Result<Provider> {
    Ok(Provider {
        id: row.get(0)?,
        name: row.get(1)?,
        display_name: row.get(2)?,
        icon: row.get(3)?,
        base_url: row.get(4)?,
        api_type: row.get(5)?,
        compat_type: row.get(6)?,
        category: row.get(7)?,
        website_url: row.get(8)?,
        api_key_url: row.get(9)?,
        preset_id: row.get(10)?,
        openai_base_url: row.get(11)?,
        anthropic_base_url: row.get(12)?,
        description: row.get(13)?,
        created_at: row.get(14)?,
    })
}

pub fn get_all_providers(db: &Database) -> Result<Vec<Provider>, AppError> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn
        .prepare(&format!("SELECT {} FROM providers ORDER BY id", SELECT_COLUMNS))
        .map_err(AppError::Database)?;

    let rows = stmt
        .query_map([], map_provider)
        .map_err(AppError::Database)?;

    let mut providers = Vec::new();
    for row in rows {
        providers.push(row.map_err(AppError::Database)?);
    }

    Ok(providers)
}

pub fn get_providers_with_keys(db: &Database) -> Result<Vec<Provider>, AppError> {
    let conn = db.conn.lock().unwrap();
    let mut stmt = conn
        .prepare(&format!(
            "SELECT {} FROM providers WHERE EXISTS (SELECT 1 FROM api_keys WHERE provider_id = providers.id) ORDER BY id",
            SELECT_COLUMNS
        ))
        .map_err(AppError::Database)?;

    let rows = stmt
        .query_map([], map_provider)
        .map_err(AppError::Database)?;

    let mut providers = Vec::new();
    for row in rows {
        providers.push(row.map_err(AppError::Database)?);
    }

    Ok(providers)
}

pub fn get_provider_by_id(db: &Database, id: i64) -> Result<Provider, AppError> {
    let conn = db.conn.lock().unwrap();
    let provider = conn
        .query_row(
            &format!("SELECT {} FROM providers WHERE id = ?1", SELECT_COLUMNS),
            [id],
            map_provider,
        )
        .map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => {
                AppError::NotFound(format!("Provider with id {} not found", id))
            }
            other => AppError::Database(other),
        })?;

    Ok(provider)
}

pub fn create_provider(
    db: &Database,
    name: &str,
    display_name: &str,
    icon: Option<&str>,
    category: &str,
    openai_base_url: Option<&str>,
    anthropic_base_url: Option<&str>,
    description: Option<&str>,
    preset_id: Option<&str>,
) -> Result<Provider, AppError> {
    let conn = db.conn.lock().unwrap();
    conn.execute(
        "INSERT INTO providers (name, display_name, icon, base_url, api_type, compat_type, category, openai_base_url, anthropic_base_url, description, preset_id) VALUES (?1, ?2, ?3, '', 'openai', 'openai_compatible', ?4, ?5, ?6, ?7, ?8)",
        rusqlite::params![name, display_name, icon, category, openai_base_url, anthropic_base_url, description, preset_id],
    )
    .map_err(AppError::Database)?;

    let id = conn.last_insert_rowid();
    let provider = conn
        .query_row(
            &format!("SELECT {} FROM providers WHERE id = ?1", SELECT_COLUMNS),
            [id],
            map_provider,
        )
        .map_err(AppError::Database)?;

    Ok(provider)
}

pub fn update_provider_metadata(
    db: &Database,
    id: i64,
    name: Option<&str>,
    display_name: Option<&str>,
    openai_base_url: Option<&str>,
    anthropic_base_url: Option<&str>,
    description: Option<&str>,
) -> Result<Provider, AppError> {
    let conn = db.conn.lock().unwrap();
    conn.execute(
        "UPDATE providers SET name = COALESCE(?1, name), display_name = COALESCE(?2, display_name), openai_base_url = ?3, anthropic_base_url = ?4, description = ?5 WHERE id = ?6",
        rusqlite::params![name, display_name, openai_base_url, anthropic_base_url, description, id],
    )
    .map_err(AppError::Database)?;

    let provider = conn
        .query_row(
            &format!("SELECT {} FROM providers WHERE id = ?1", SELECT_COLUMNS),
            [id],
            map_provider,
        )
        .map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => {
                AppError::NotFound(format!("Provider with id {} not found", id))
            }
            other => AppError::Database(other),
        })?;

    Ok(provider)
}
