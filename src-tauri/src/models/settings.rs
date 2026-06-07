use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub id: i64,
    pub password_hash: Option<String>,
    pub theme: String,
    pub auto_lock_minutes: i64,
    pub language: String,
    pub created_at: String,
    pub updated_at: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            id: 1,
            password_hash: None,
            theme: "dark".to_string(),
            auto_lock_minutes: 5,
            language: "en".to_string(),
            created_at: String::new(),
            updated_at: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSettings {
    pub theme: Option<String>,
    pub auto_lock_minutes: Option<i64>,
    pub language: Option<String>,
}