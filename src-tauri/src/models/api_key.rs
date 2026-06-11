use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: i64,
    pub provider_id: i64,
    pub name: String,
    pub encrypted_key: Vec<u8>,
    pub iv: Vec<u8>,
    pub masked_preview: String,
    pub parent_id: Option<i64>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyView {
    pub id: i64,
    pub provider_id: i64,
    pub provider_name: String,
    pub provider_display_name: String,
    pub name: String,
    pub masked_preview: String,
    pub parent_id: Option<i64>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewApiKey {
    pub provider_id: i64,
    pub name: String,
    pub raw_key: String,
    pub parent_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateApiKey {
    pub id: i64,
    pub provider_id: i64,
    pub name: String,
    pub raw_key: Option<String>,
    pub parent_id: Option<i64>,
}
