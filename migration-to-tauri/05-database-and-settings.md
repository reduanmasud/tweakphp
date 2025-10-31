# Step 5: Database and Settings Implementation

## Overview
Fully implement the SQLite database operations and settings management in Rust, replacing the Node.js implementations.

## Tasks

### 5.1 Database Schema Migration
- Run database migrations in Rust
- Copy migration SQL files from `original/migrations/` to `src-tauri/migrations/` or embed in Rust
- Create Rust migration runner that:
  - Checks current database version
  - Runs pending migrations
  - Handles migration errors
- **Use Tauri SQL Plugin**: Configure `tauri-plugin-sql` with SQLite backend in `Cargo.toml`

### 5.2 Code History Database Operations
Migrate `original/src/main/tools/code-history.ts` to Rust:
- Create Rust module `src-tauri/src/tools/code_history.rs`
- Implement `code_add` command:
  - Validate input with serde/serde_json
  - Insert code history with cursor position
  - Update tab states
  - Handle duplicate detection
- Implement `code_undo` command:
  - Query previous state
  - Update current history ID
  - Return code and cursor position
- Implement `code_redo` command:
  - Query next state
  - Update current history ID
  - Return code and cursor position

### 5.3 Settings Management
Migrate `original/src/main/settings.ts` fully:
- Use `tauri-plugin-store` for settings persistence (recommended) or implement custom JSON file storage
- Create Rust commands:
  - `get_settings()` -> reads and returns Settings struct
  - `store_settings(settings: Settings)` -> saves settings
  - Handle PHP path validation (check if directory and find php executable)
  - Handle default settings merging
  - Include `intelephenseLicenseKey` in Settings struct
- Define Settings struct in Rust with serde serialization
- Store in Tauri's app data directory (`~/.tweakphp/settings.json`)

### 5.4 Tab States Database (if needed)
- Check if tab states are stored in database or memory
- Implement any database-backed tab state operations in Rust
- Ensure tab history persistence works correctly

### 5.5 Database Transaction Handling
- Ensure all database operations use proper transactions
- Handle concurrent access correctly
- Implement proper error handling and rollback

### 5.6 Settings Events
- Emit settings change events to frontend
- Handle PHP path resolution events (`settings.php-located`)
- Update frontend when settings change

## Deliverables
- ✅ Database migrations run on app start
- ✅ Code history (add/undo/redo) fully working
- ✅ Settings load and save correctly
- ✅ PHP path validation and resolution working
- ✅ All database operations are transaction-safe
- ✅ Error handling matches original behavior

## Notes
- Use prepared statements for all SQL queries to prevent SQL injection
- Ensure database is properly initialized before app starts
- Handle database corruption gracefully
- Consider using a Rust ORM if complex queries are needed (but SQLite is simple enough for direct SQL)
- Test undo/redo extensively with edge cases (empty history, branching, etc.)
