use rusqlite::Connection;

pub fn run(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    create_tables(conn)?;
    migrate_add_language(conn)?;
    migrate_add_api_key_base_url(conn)?;
    migrate_add_provider_presets(conn)?;
    seed_providers(conn)?;
    seed_settings(conn)?;
    Ok(())
}

fn create_tables(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS providers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            display_name TEXT NOT NULL,
            icon TEXT,
            base_url TEXT NOT NULL,
            api_type TEXT NOT NULL DEFAULT 'openai',
            compat_type TEXT,
            category TEXT,
            icon_color TEXT,
            website_url TEXT,
            api_key_url TEXT,
            preset_id TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        CREATE TABLE IF NOT EXISTS api_keys (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            provider_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            encrypted_key BLOB NOT NULL,
            iv BLOB NOT NULL,
            masked_preview TEXT NOT NULL,
            description TEXT,
            base_url TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (provider_id) REFERENCES providers(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS settings (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            password_hash TEXT,
            theme TEXT NOT NULL DEFAULT 'dark',
            auto_lock_minutes INTEGER NOT NULL DEFAULT 5,
            language TEXT NOT NULL DEFAULT 'en',
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        );

        ",
    )?;

    Ok(())
}

fn migrate_add_language(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    let has_language: bool = conn
        .prepare("SELECT language FROM settings LIMIT 0")
        .is_ok();

    if !has_language {
        conn.execute("ALTER TABLE settings ADD COLUMN language TEXT NOT NULL DEFAULT 'en'", [])?;
    }

    Ok(())
}

fn migrate_add_api_key_base_url(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    let mut existing_columns = std::collections::HashSet::new();
    {
        let mut stmt = conn.prepare("PRAGMA table_info(api_keys)").map_err(|e| Box::<dyn std::error::Error>::from(e))?;
        let mut rows = stmt.query([]).map_err(|e| Box::<dyn std::error::Error>::from(e))?;
        while let Some(row) = rows.next().map_err(|e| Box::<dyn std::error::Error>::from(e))? {
            let column_name: String = row.get(1).map_err(|e| Box::<dyn std::error::Error>::from(e))?;
            existing_columns.insert(column_name);
        }
    }

    if !existing_columns.contains("base_url") {
        conn.execute("ALTER TABLE api_keys ADD COLUMN base_url TEXT", [])?;
    }

    Ok(())
}

fn migrate_add_provider_presets(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    let columns_to_add: Vec<(&str, &str)> = vec![
        ("compat_type", "TEXT"),
        ("category", "TEXT"),
        ("icon_color", "TEXT"),
        ("website_url", "TEXT"),
        ("api_key_url", "TEXT"),
        ("preset_id", "TEXT"),
    ];

    let mut existing_columns = std::collections::HashSet::new();
    {
        let mut stmt = conn.prepare("PRAGMA table_info(providers)").map_err(|e| Box::<dyn std::error::Error>::from(e))?;
        let mut rows = stmt.query([]).map_err(|e| Box::<dyn std::error::Error>::from(e))?;
        while let Some(row) = rows.next().map_err(|e| Box::<dyn std::error::Error>::from(e))? {
            let column_name: String = row.get(1).map_err(|e| Box::<dyn std::error::Error>::from(e))?;
            existing_columns.insert(column_name);
        }
    }

    for (column_name, column_type) in columns_to_add {
        if !existing_columns.contains(column_name) {
            conn.execute(&format!("ALTER TABLE providers ADD COLUMN {} {}", column_name, column_type), [])?;
        }
    }

    Ok(())
}

fn seed_providers(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM providers", [], |row| row.get(0))?;
    if count > 0 {
        seed_missing_providers(conn)?;
        return Ok(());
    }

    // (name, display_name, icon, base_url, api_type, preset_id)
    let providers: Vec<(&str, &str, &str, &str, &str, &str)> = vec![
        ("openai", "OpenAI", "🤖", "https://api.openai.com/v1", "openai", "openai"),
        ("anthropic", "Anthropic", "🟠", "https://api.anthropic.com/v1", "openai", "anthropic"),
        ("google", "Google Gemini", "🔵", "https://generativelanguage.googleapis.com/v1beta", "openai", "google"),
        ("azure_openai", "Azure OpenAI", "☁️", "", "openai", "azure_openai"),
        ("aws_bedrock", "AWS Bedrock", "🟡", "", "openai", "aws_bedrock"),
        ("deepseek", "DeepSeek", "🐋", "https://api.deepseek.com/v1", "openai", "deepseek"),
        ("qwen", "通义千问", "☁️", "https://dashscope.aliyuncs.com/compatible-mode/v1", "openai", "qwen"),
        ("zhipu", "智谱 GLM", "🟣", "https://open.bigmodel.cn/api/paas/v4", "openai", "zhipu"),
        ("moonshot", "Kimi (月之暗面)", "🌙", "https://api.moonshot.cn/v1", "openai", "moonshot"),
        ("wenxin", "百度文心", "🔴", "https://aip.baidubce.com", "openai", "wenxin"),
        ("spark", "讯飞星火", "✨", "https://spark-api-open.xf-yun.com/v1", "openai", "spark"),
        ("yi", "零一万物", "⚡", "https://api.lingyiwanwu.com/v1", "openai", "yi"),
        ("minimax", "MiniMax", "🟢", "https://api.minimax.chat/v1", "openai", "minimax"),
        ("baichuan", "百川智能", "🟤", "https://api.baichuan-ai.com/v1", "openai", "baichuan"),
        ("cohere", "Cohere", "🔷", "https://api.cohere.ai/v1", "openai", "cohere"),
        ("mistral", "Mistral AI", "🟠", "https://api.mistral.ai/v1", "openai", "mistral"),
        ("custom", "自定义供应商", "🔗", "", "openai", ""),
    ];

    let mut stmt = conn.prepare(
        "INSERT INTO providers (name, display_name, icon, base_url, api_type, preset_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
    )?;

    for (name, display_name, icon, base_url, api_type, preset_id) in providers {
        stmt.execute(rusqlite::params![name, display_name, icon, base_url, api_type, preset_id])?;
    }

    Ok(())
}

fn seed_missing_providers(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    // (name, display_name, icon, base_url, api_type, preset_id)
    let new_providers: Vec<(&str, &str, &str, &str, &str, &str)> = vec![
        ("azure_openai", "Azure OpenAI", "☁️", "", "openai", "azure_openai"),
        ("aws_bedrock", "AWS Bedrock", "🟡", "", "openai", "aws_bedrock"),
        ("minimax", "MiniMax", "🟢", "https://api.minimax.chat/v1", "openai", "minimax"),
        ("baichuan", "百川智能", "🟤", "https://api.baichuan-ai.com/v1", "openai", "baichuan"),
        ("cohere", "Cohere", "🔷", "https://api.cohere.ai/v1", "openai", "cohere"),
        ("mistral", "Mistral AI", "🟠", "https://api.mistral.ai/v1", "openai", "mistral"),
    ];

    for (name, display_name, icon, base_url, api_type, preset_id) in new_providers {
        let exists: bool = conn
            .query_row(
                "SELECT COUNT(*) FROM providers WHERE name = ?1",
                [name],
                |row| Ok(row.get::<_, i64>(0)? > 0),
            )
            .unwrap_or(false);

        if !exists {
            conn.execute(
                "INSERT INTO providers (name, display_name, icon, base_url, api_type, preset_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                rusqlite::params![name, display_name, icon, base_url, api_type, preset_id],
            )?;
        }
    }

    let _ = conn.execute(
        "UPDATE providers SET display_name = '\u{667a}\u{8c31} GLM' WHERE name = 'zhipu' AND display_name != '\u{667a}\u{8c31} GLM'",
        [],
    );
    let _ = conn.execute(
        "UPDATE providers SET display_name = 'Kimi (\u{6708}\u{4e4b}\u{6697}\u{9762})' WHERE name = 'moonshot' AND display_name != 'Kimi (\u{6708}\u{4e4b}\u{6697}\u{9762})'",
        [],
    );
    let _ = conn.execute(
        "UPDATE providers SET display_name = '\u{767e}\u{5ea6}\u{6587}\u{5fc3}' WHERE name = 'wenxin' AND display_name != '\u{767e}\u{5ea6}\u{6587}\u{5fc3}'",
        [],
    );
    let _ = conn.execute(
        "UPDATE providers SET icon = '\u{1f534}' WHERE name = 'wenxin' AND icon = '\u{1f535}'",
        [],
    );
    let _ = conn.execute(
        "UPDATE providers SET display_name = '\u{81ea}\u{5b9a}\u{4e49}\u{4f9b}\u{5e94}\u{5546}' WHERE name = 'custom' AND display_name = '\u{7b2c}\u{4e09}\u{65b9}\u{4e2d}\u{8f6c}'",
        [],
    );

    Ok(())
}

fn seed_settings(conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM settings", [], |row| row.get(0))?;
    if count > 0 {
        return Ok(());
    }

    conn.execute(
        "INSERT INTO settings (id, password_hash, theme, auto_lock_minutes, language) VALUES (1, NULL, 'dark', 5, 'en')",
        [],
    )?;

    Ok(())
}