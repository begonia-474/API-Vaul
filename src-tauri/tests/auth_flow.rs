use api_vaul::db::connection::Database;
use api_vaul::crypto::password;

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

#[test]
fn test_password_hash_and_verify() {
    let password_str = "123456";
    let hash = password::hash_password(password_str).unwrap();
    assert!(!hash.is_empty(), "Hash should not be empty");
    eprintln!("Hash: {}", hash);
    
    let result = password::verify_password(password_str, &hash).unwrap();
    assert!(result, "Password should verify correctly");
}

#[test]
fn test_full_setup_password_flow() {
    let db = test_db();
    let conn = db.conn.lock().unwrap();
    
    // Check first run
    let hash: Option<String> = conn
        .query_row("SELECT password_hash FROM settings WHERE id = 1", [], |row| row.get(0))
        .unwrap();
    assert!(hash.is_none(), "Should be first run (no password hash)");
    drop(conn);
    
    // Setup password
    let password_str = "123456";
    let hash = password::hash_password(password_str).unwrap();
    
    let conn = db.conn.lock().unwrap();
    let affected = conn.execute(
        "UPDATE settings SET password_hash = ?1, updated_at = datetime('now') WHERE id = 1",
        [&hash],
    ).unwrap();
    assert_eq!(affected, 1, "Should update exactly 1 row");
    
    // Verify password was saved
    let saved_hash: Option<String> = conn
        .query_row("SELECT password_hash FROM settings WHERE id = 1", [], |row| row.get(0))
        .unwrap();
    assert!(saved_hash.is_some(), "Password hash should be saved");
    drop(conn);
    
    // Verify password
    let conn = db.conn.lock().unwrap();
    let saved_hash: String = conn
        .query_row("SELECT password_hash FROM settings WHERE id = 1", [], |row| row.get(0))
        .unwrap();
    drop(conn);
    
    let result = password::verify_password(password_str, &saved_hash).unwrap();
    assert!(result, "Password should verify after setup");
}