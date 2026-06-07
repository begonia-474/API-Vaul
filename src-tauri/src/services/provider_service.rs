use crate::db::connection::Database;
use crate::error::AppError;
use crate::models::provider::Provider;

const SELECT_COLUMNS: &str = "id, name, display_name, icon, base_url, api_type, compat_type, category, website_url, api_key_url, preset_id, created_at";

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
        created_at: row.get(11)?,
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
