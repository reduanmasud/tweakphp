mod db;
mod settings;
mod tools;
mod system;
mod client;
mod lsp;

use settings::{get_settings, store_settings, settings_to_frontend};
use system::{
    update_check, update_download,
    window_show, window_hide, window_minimize, window_maximize, window_close
};
use client::commands::{client_connect, client_execute, client_action, client_info};
use tauri::Emitter;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_platform() -> String {
    // Return a simple platform identifier
    std::env::consts::OS.to_string()
}

#[tauri::command]
async fn init(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    // Initialize database on app start (ensures migrations are run),
    // but never fail init just because DB couldn't initialize.
    if let Err(e) = db::get_connection(&app) {
        eprintln!("Warning: Failed to initialize database: {}", e);
    }
    
    let settings = get_settings(app).await?;
    let settings_json = settings_to_frontend(&settings);
    
    Ok(serde_json::json!({
        "settings": settings_json,
        "platform": std::env::consts::OS,
    }))
}

#[tauri::command]
fn link_open(url: String) -> Result<(), String> {
    // Use opener plugin via frontend API - this command is just for consistency
    // The frontend will call opener plugin directly
    if url.is_empty() {
        return Err("URL cannot be empty".to_string());
    }
    Ok(())
}

#[tauri::command]
async fn source_open_path(path: String) -> Result<(), String> {
    if path.is_empty() {
        return Err("Path cannot be empty".to_string());
    }
    
    // Use system command to open path in file manager
    // On macOS: open
    // On Linux: xdg-open
    // On Windows: explorer
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open path: {}", e))?;
    }
    #[cfg(target_os = "linux")]
    {
        use std::process::Command;
        Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open path: {}", e))?;
    }
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        Command::new("explorer")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open path: {}", e))?;
    }
    
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            // Setup app lifecycle handlers
            // Window events will be handled in .on_window_event() below
            
            // Setup updater event listeners
            // The updater plugin emits events automatically that the frontend will listen to
            // Start LSP server so the editor can connect immediately
            let _ = lsp::start_server_if_needed(app.handle().clone());
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            get_platform,
            init,
            link_open,
            source_open_path,
            get_settings,
            store_settings,
            tools::code_history::code_add,
            tools::code_history::code_undo,
            tools::code_history::code_redo,
            update_check,
            update_download,
            window_show,
            window_hide,
            window_minimize,
            window_maximize,
            window_close,
            client_connect,
            client_execute,
            client_action,
            client_info,
            lsp_restart
        ])
        .on_window_event(|window, event| {
            // Handle window events globally
            match event {
                tauri::WindowEvent::CloseRequested { .. } => {
                    // Emit ssh.disconnect event (if needed by frontend)
                    let _ = window.emit("ssh:disconnect", ());
                    // Don't prevent close by default
                    // The app will quit when all windows are closed
                }
                tauri::WindowEvent::Resized { .. } => {
                    // Window size is saved by frontend through settings.store
                    // No action needed here
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn lsp_restart(app: tauri::AppHandle) -> Result<(), String> {
    // Restart the LSP WebSocket server
    lsp::restart_server(app).await
}
