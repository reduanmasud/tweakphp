# Step 4: File System Operations

## Overview
Migrate all file system operations from Node.js `fs` to Tauri's file system plugin or Rust native file operations.

## Tasks

### 4.1 Settings File Operations
Replace `original/src/main/settings.ts` file operations:
- Use `tauri-plugin-store` for settings persistence
- Create Rust command `get_settings()` that reads from store
- Create Rust command `store_settings(settings: Settings)` that writes to store
- Handle default settings and migrations
- Store settings in Tauri's app data directory

### 4.2 Database File Operations
Migrate SQLite database access:
- Use `tauri-plugin-sql` with SQLite backend
- Or use `rusqlite` directly in Rust
- Create Rust module for database operations in `src-tauri/src/db/`
- Migrate `db_manager.ts` logic to Rust:
  - Database initialization
  - Path management (`~/.tweakphp/tweakphp.db`)
  - Connection handling

### 4.3 Source/Directory Operations
Migrate `original/src/main/source.ts`:
- Use `tauri-plugin-dialog` for directory picker
- Use `tauri-plugin-shell` for opening paths in system
- Replace `dialog.showOpenDialog()` with Tauri dialog API
- Replace `shell.openPath()` with Tauri shell API

### 4.4 Laravel Path Handling
Handle Laravel client paths:
- Determine if packaged or in development
- Use Tauri resource path APIs for packaged resources
- Copy Laravel ZIP to appropriate location
- Handle path resolution in Rust commands

### 4.5 File Watching (if needed)
- Check if any file watching is needed
- Use Tauri file watching capabilities if required
- Or implement in Rust using `notify` crate

### 4.6 App Data Directory Setup
- Ensure `~/.tweakphp` directory exists
- Handle cross-platform paths (Windows, macOS, Linux)
- Create necessary subdirectories
- Use Tauri's `path::app_data_dir()` or similar

## Deliverables
- ✅ Settings persist using Tauri store
- ✅ Database file operations work via Tauri SQL plugin or Rust
- ✅ Directory picker works with Tauri dialog
- ✅ External path opening works with Tauri shell
- ✅ All file paths resolve correctly on all platforms
- ✅ App data directory structure matches original

## Notes
- Use Tauri's built-in path resolution utilities for cross-platform compatibility
- Handle both development and production paths correctly
- Ensure permissions are properly configured in `tauri.conf.json`
- Test on all target platforms (macOS, Linux, Windows)
