use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provider {
    pub id: i64,
    pub name: String,
    pub display_name: String,
    pub icon: Option<String>,
    pub base_url: String,
    pub api_type: String,
    pub compat_type: Option<String>,
    pub category: Option<String>,
    pub website_url: Option<String>,
    pub api_key_url: Option<String>,
    pub preset_id: Option<String>,
    pub openai_base_url: Option<String>,
    pub anthropic_base_url: Option<String>,
    pub description: Option<String>,
    pub created_at: String,
}
