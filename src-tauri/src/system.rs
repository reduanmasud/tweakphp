use serde::{Deserialize, Serialize};
use tauri::{command, AppHandle, Emitter, Manager};

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UpdateInfo {
    pub version: String,
    pub release_date: Option<String>,
}

// Auto-Updater Commands
#[command]
pub async fn update_check(app: AppHandle) -> Result<(), String> {
    // Emit checking event to frontend
    let _ = app.emit("update:checking", ());
    
    // The Tauri updater plugin will handle the actual check
    // It emits events: tauri://update-available, tauri://update-not-available
    // We'll map these to the frontend's expected event names in the frontend code
    
    Ok(())
}

#[command]
pub async fn update_download(_app: AppHandle) -> Result<(), String> {
    // The updater plugin handles the actual download
    // It emits tauri://update-downloaded when complete
    // We'll map this in the frontend code
    
    Ok(())
}

// Window Management Commands
#[command]
pub async fn window_show(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.show().map_err(|e| format!("Failed to show window: {}", e))?;
    } else {
        return Err("Main window not found".to_string());
    }
    Ok(())
}

#[command]
pub async fn window_hide(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.hide().map_err(|e| format!("Failed to hide window: {}", e))?;
    } else {
        return Err("Main window not found".to_string());
    }
    Ok(())
}

#[command]
pub async fn window_minimize(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.minimize().map_err(|e| format!("Failed to minimize window: {}", e))?;
    } else {
        return Err("Main window not found".to_string());
    }
    Ok(())
}

#[command]
pub async fn window_maximize(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_maximized().unwrap_or(false) {
            window.unmaximize()
                .map_err(|e| format!("Failed to unmaximize window: {}", e))?;
        } else {
            window.maximize()
                .map_err(|e| format!("Failed to maximize window: {}", e))?;
        }
    } else {
        return Err("Main window not found".to_string());
    }
    Ok(())
}

#[command]
pub async fn window_close(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.close().map_err(|e| format!("Failed to close window: {}", e))?;
    } else {
        return Err("Main window not found".to_string());
    }
    Ok(())
}

