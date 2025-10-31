use rusqlite::{Connection, Result as SqliteResult};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use tauri::path::BaseDirectory;

pub fn get_app_data_dir(app: &AppHandle) -> Result<PathBuf, String> {
    // Use Tauri's path API to resolve the user's home directory reliably
    let home_path = app
        .path()
        .resolve("", BaseDirectory::Home)
        .map_err(|e| format!("Failed to resolve home directory: {}", e))?;

    Ok(home_path.join(".tweakphp"))
}

pub fn get_db_path(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(get_app_data_dir(app)?.join("tweakphp.db"))
}

pub fn get_connection(app: &AppHandle) -> Result<Connection, String> {
    let dir = get_app_data_dir(app)?;
    if !dir.exists() {
        std::fs::create_dir_all(&dir)
            .map_err(|e| format!("Failed to create app data directory: {}", e))?;
    }
    
    let path = get_db_path(app)?;
    let conn = Connection::open(&path)
        .map_err(|e| format!("Failed to open database at {:?}: {}", path, e))?;
    
    // Enable WAL mode for better concurrency.
    // For PRAGMAs that return a row, use query_row (execute would error).
    let _ = conn
        .query_row("PRAGMA journal_mode = WAL", [], |_row| Ok(()))
        .map_err(|e| format!("Failed to enable WAL mode: {}", e));
    
    // Initialize migrations table
    init_migrations_table(&conn)
        .map_err(|e| format!("Failed to initialize migrations table: {}", e))?;
    
    // Run migrations
    run_migrations(&conn)?;
    
    Ok(conn)
}

fn init_migrations_table(conn: &Connection) -> SqliteResult<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS migrations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            run_on DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;
    Ok(())
}

fn get_applied_migrations(conn: &Connection) -> Result<Vec<String>, String> {
    let mut stmt = conn
        .prepare("SELECT name FROM migrations ORDER BY id")
        .map_err(|e| format!("Failed to prepare migrations query: {}", e))?;
    
    let rows = stmt
        .query_map([], |row| Ok(row.get::<_, String>(0)?))
        .map_err(|e| format!("Failed to query migrations: {}", e))?;
    
    let mut migrations = Vec::new();
    for row in rows {
        migrations.push(row.map_err(|e| format!("Failed to read migration row: {}", e))?);
    }
    
    Ok(migrations)
}

fn run_migrations(conn: &Connection) -> Result<(), String> {
    let applied = get_applied_migrations(conn)?;
    
    // Get migration files (embedded in binary for now)
    let migrations = vec!["001_create_code_histories_and_tab_states_table.sql"];
    
    for migration_name in migrations {
        if !applied.contains(&migration_name.to_string()) {
            let sql = match migration_name {
                "001_create_code_histories_and_tab_states_table.sql" => {
                    include_str!("../migrations/001_create_code_histories_and_tab_states_table.sql")
                }
                _ => {
                    return Err(format!("Unknown migration: {}", migration_name));
                }
            };
            
            // Run migration in a transaction
            let tx = conn
                .unchecked_transaction()
                .map_err(|e| format!("Failed to start transaction: {}", e))?;
            
            tx.execute_batch(sql)
                .map_err(|e| format!("Failed to execute migration {}: {}", migration_name, e))?;
            
            // Record migration
            tx.execute(
                "INSERT INTO migrations (name) VALUES (?1)",
                [migration_name],
            )
            .map_err(|e| format!("Failed to record migration {}: {}", migration_name, e))?;
            
            tx.commit()
                .map_err(|e| format!("Failed to commit migration {}: {}", migration_name, e))?;
            
            println!("Applied migration: {}", migration_name);
        }
    }
    
    Ok(())
}

