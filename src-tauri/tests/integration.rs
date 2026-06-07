use api_vaul::db::connection::Database;
use api_vaul::models::api_key::{NewApiKey, UpdateApiKey};
use api_vaul::services::{key_service, provider_service};

fn test_db() -> Database {
    let dir = std::env::temp_dir().join("api-vaul-integration-tests");
    std::fs::create_dir_all(&dir).ok();
    let id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = dir.join(format!("test-{}.db", id));
    Database::open_at(&path).expect("Failed to create test database")
}

fn test_enc_key() -> [u8; 32] {
    let mut key = [0u8; 32];
    key[0..16].copy_from_slice(b"test-enc-key----");
    key[16..32].copy_from_slice(b"----test-enc-key");
    key
}

#[test]
fn test_provider_crud() {
    let db = test_db();
    let providers = provider_service::get_all_providers(&db).unwrap();
    assert!(providers.len() > 0, "Should have seeded providers");

    let openai = providers.iter().find(|p| p.name == "openai").unwrap();
    assert_eq!(openai.display_name, "OpenAI");
}

#[test]
fn test_create_and_get_key() {
    let db = test_db();
    let enc = test_enc_key();

    let providers = provider_service::get_all_providers(&db).unwrap();
    let openai = providers.iter().find(|p| p.name == "openai").unwrap();

    let new_key = NewApiKey {
        provider_id: openai.id,
        name: "Test Key".to_string(),
        raw_key: "sk-test12345678901234".to_string(),
        description: Some("A test key".to_string()),
        openai_base_url: Some("https://api.openai.com/v1".to_string()),
        anthropic_base_url: None,
    };

    let created = key_service::create_key(&db, &enc, &new_key).unwrap();
    assert_eq!(created.name, "Test Key");
    assert_eq!(created.provider_name, "openai");
    assert!(created.masked_preview.contains("sk-t"));
    assert_eq!(created.openai_base_url.as_deref(), Some("https://api.openai.com/v1"));
}

#[test]
fn test_search_keys() {
    let db = test_db();
    let enc = test_enc_key();

    let providers = provider_service::get_all_providers(&db).unwrap();
    let openai = providers.iter().find(|p| p.name == "openai").unwrap();

    key_service::create_key(&db, &enc, &NewApiKey {
        provider_id: openai.id,
        name: "Production Key".to_string(),
        raw_key: "sk-prod12345678901234".to_string(),
        description: Some("Main production key".to_string()),
        openai_base_url: Some("https://api.openai.com/v1".to_string()),
        anthropic_base_url: None,
    }).unwrap();

    key_service::create_key(&db, &enc, &NewApiKey {
        provider_id: openai.id,
        name: "Dev Key".to_string(),
        raw_key: "sk-dev123456789012345".to_string(),
        description: Some("Development key".to_string()),
        openai_base_url: None,
        anthropic_base_url: None,
    }).unwrap();

    // Search by name
    let results = key_service::search_keys(&db, "Production").unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Production Key");

    // Search by provider name
    let results = key_service::search_keys(&db, "openai").unwrap();
    assert_eq!(results.len(), 2);

    // Search by description
    let results = key_service::search_keys(&db, "Development").unwrap();
    assert_eq!(results.len(), 1);
}

#[test]
fn test_update_key() {
    let db = test_db();
    let enc = test_enc_key();

    let providers = provider_service::get_all_providers(&db).unwrap();
    let openai = providers.iter().find(|p| p.name == "openai").unwrap();

    let created = key_service::create_key(&db, &enc, &NewApiKey {
        provider_id: openai.id,
        name: "Original".to_string(),
        raw_key: "sk-original1234567890".to_string(),
        description: None,
        openai_base_url: None,
        anthropic_base_url: None,
    }).unwrap();

    let updated = key_service::update_key(&db, &enc, &UpdateApiKey {
        id: created.id,
        provider_id: openai.id,
        name: "Renamed".to_string(),
        raw_key: None,
        description: Some("Updated desc".to_string()),
        openai_base_url: Some("https://api.openai.com/v1".to_string()),
        anthropic_base_url: Some("https://api.anthropic.com/v1".to_string()),
    }).unwrap();

    assert_eq!(updated.name, "Renamed");
    assert_eq!(updated.description, Some("Updated desc".to_string()));
    assert_eq!(updated.openai_base_url.as_deref(), Some("https://api.openai.com/v1"));
    assert_eq!(updated.anthropic_base_url.as_deref(), Some("https://api.anthropic.com/v1"));
}

#[test]
fn test_delete_key() {
    let db = test_db();
    let enc = test_enc_key();

    let providers = provider_service::get_all_providers(&db).unwrap();
    let openai = providers.iter().find(|p| p.name == "openai").unwrap();

    let created = key_service::create_key(&db, &enc, &NewApiKey {
        provider_id: openai.id,
        name: "To Delete".to_string(),
        raw_key: "sk-delete1234567890".to_string(),
        description: None,
        openai_base_url: None,
        anthropic_base_url: None,
    }).unwrap();

    key_service::delete_key(&db, created.id).unwrap();

    let all = key_service::get_all_keys(&db).unwrap();
    assert!(!all.iter().any(|k| k.id == created.id));
}

#[test]
fn test_decrypt_key() {
    let db = test_db();
    let enc = test_enc_key();
    let raw = "sk-my-secret-api-key-12345678";

    let providers = provider_service::get_all_providers(&db).unwrap();
    let openai = providers.iter().find(|p| p.name == "openai").unwrap();

    let created = key_service::create_key(&db, &enc, &NewApiKey {
        provider_id: openai.id,
        name: "Decrypt Test".to_string(),
        raw_key: raw.to_string(),
        description: None,
        openai_base_url: None,
        anthropic_base_url: None,
    }).unwrap();

    let decrypted = key_service::get_decrypted_key(&db, &enc, created.id).unwrap();
    assert_eq!(decrypted, raw);
}
