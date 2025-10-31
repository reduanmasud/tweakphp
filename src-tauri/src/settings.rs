use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::{command, AppHandle, Manager};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub version: String,
    pub laravel_path: String,
    pub php: String,
    pub theme: String,
    pub editor_font_size: u32,
    pub editor_word_wrap: String,
    pub layout: String,
    pub output: String,
    pub vim_mode: String,
    pub stacked_dump: String,
    pub window_width: u32,
    pub window_height: u32,
    pub intelephense_license_key: Option<String>,
    pub navigation_display: Option<String>,
}

fn get_app_data_dir(app: &AppHandle) -> Result<PathBuf, String> {
    use tauri::path::BaseDirectory;
    // Use the same logic as db module - resolve home directory and join .tweakphp
    let home_path = app
        .path()
        .resolve("", BaseDirectory::Home)
        .map_err(|e| format!("Failed to resolve home directory: {}", e))?;
    
    Ok(home_path.join(".tweakphp"))
}

fn get_default_settings(app: &AppHandle) -> Result<Settings, String> {
    let app_data_dir = get_app_data_dir(app)?;
    let laravel_path = if cfg!(debug_assertions) {
        // Development mode: assume laravel directory is in original/laravel or public/laravel
        // Try to find it relative to the executable or use a default
        format!("{}/laravel", app_data_dir.display())
    } else {
        // Production mode: use resources path
        // Tauri will handle resource path resolution
        format!("{}/laravel", app_data_dir.display())
    };

    Ok(Settings {
        version: env!("CARGO_PKG_VERSION").to_string(),
        laravel_path,
        php: String::new(),
        theme: "dracula".to_string(),
        editor_font_size: 15,
        editor_word_wrap: "on".to_string(),
        layout: "vertical".to_string(),
        output: "code".to_string(),
        vim_mode: "off".to_string(),
        stacked_dump: "extended".to_string(),
        window_width: 1100,
        window_height: 700,
        intelephense_license_key: None,
        navigation_display: Some("collapsed".to_string()),
    })
}

#[command]
pub async fn get_settings(app: AppHandle) -> Result<Settings, String> {
    // Ensure app data directory exists
    let app_data_dir = get_app_data_dir(&app)?;
    if !app_data_dir.exists() {
        std::fs::create_dir_all(&app_data_dir)
            .map_err(|e| format!("Failed to create app data directory: {}", e))?;
    }

    let settings_path = app_data_dir.join("settings.json");
    
    // Get default settings
    let mut settings = get_default_settings(&app)?;
    
    if settings_path.exists() {
        match std::fs::read_to_string(&settings_path) {
            Ok(content) => {
                if let Ok(stored_settings) = serde_json::from_str::<serde_json::Value>(&content) {
                    // Merge stored settings with defaults
                    if let Some(v) = stored_settings.get("version") {
                        if let Some(s) = v.as_str() {
                            settings.version = s.to_string();
                        }
                    }
                    if let Some(v) = stored_settings.get("laravelPath") {
                        if let Some(s) = v.as_str() {
                            settings.laravel_path = s.to_string();
                        }
                    }
                    if let Some(v) = stored_settings.get("php") {
                        if let Some(s) = v.as_str() {
                            settings.php = s.to_string();
                        }
                    }
                    if let Some(v) = stored_settings.get("theme") {
                        if let Some(s) = v.as_str() {
                            settings.theme = s.to_string();
                        }
                    }
                    if let Some(v) = stored_settings.get("editorFontSize") {
                        if let Some(n) = v.as_u64() {
                            settings.editor_font_size = n as u32;
                        }
                    }
                    if let Some(v) = stored_settings.get("editorWordWrap") {
                        if let Some(s) = v.as_str() {
                            settings.editor_word_wrap = s.to_string();
                        }
                    }
                    if let Some(v) = stored_settings.get("layout") {
                        if let Some(s) = v.as_str() {
                            settings.layout = s.to_string();
                        }
                    }
                    if let Some(v) = stored_settings.get("output") {
                        if let Some(s) = v.as_str() {
                            settings.output = s.to_string();
                        }
                    }
                    if let Some(v) = stored_settings.get("vimMode") {
                        if let Some(s) = v.as_str() {
                            settings.vim_mode = s.to_string();
                        }
                    }
                    if let Some(v) = stored_settings.get("stackedDump") {
                        if let Some(s) = v.as_str() {
                            settings.stacked_dump = s.to_string();
                        }
                    }
                    if let Some(v) = stored_settings.get("windowWidth") {
                        if let Some(n) = v.as_u64() {
                            settings.window_width = n as u32;
                        }
                    }
                    if let Some(v) = stored_settings.get("windowHeight") {
                        if let Some(n) = v.as_u64() {
                            settings.window_height = n as u32;
                        }
                    }
                    if let Some(v) = stored_settings.get("intelephenseLicenseKey") {
                        if let Some(s) = v.as_str() {
                            settings.intelephense_license_key = Some(s.to_string());
                        } else if v.is_null() {
                            settings.intelephense_license_key = None;
                        }
                    }
                    if let Some(v) = stored_settings.get("navigationDisplay") {
                        if let Some(s) = v.as_str() {
                            settings.navigation_display = Some(s.to_string());
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to read settings file: {}", e);
            }
        }
    }

    Ok(settings)
}

#[command]
pub async fn store_settings(app: AppHandle, settings_data: serde_json::Value) -> Result<String, String> {
    // Ensure app data directory exists
    let app_data_dir = get_app_data_dir(&app)?;
    if !app_data_dir.exists() {
        std::fs::create_dir_all(&app_data_dir)
            .map_err(|e| format!("Failed to create app data directory: {}", e))?;
    }

    let settings_path = app_data_dir.join("settings.json");

    // Handle PHP path validation (similar to original handlePhpExecutable)
    let mut php_path = settings_data
        .get("php")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let mut found_php_path: Option<String> = None;

    // If PHP path is a directory, try to find php executable inside it
    let php_path_buf = PathBuf::from(&php_path);
    if php_path_buf.exists() && php_path_buf.is_dir() {
        let php_executable = if cfg!(target_os = "windows") {
            "php.exe"
        } else {
            "php"
        };
        let potential_path = php_path_buf.join(php_executable);
        if potential_path.exists() {
            let resolved_path = potential_path.to_string_lossy().to_string();
            found_php_path = Some(resolved_path.clone());
            php_path = resolved_path;
        }
    }

    // Update settings with validated PHP path
    let mut updated_settings = settings_data.clone();
    if let Some(obj) = updated_settings.as_object_mut() {
        obj.insert("php".to_string(), serde_json::Value::String(php_path.clone()));
    }

    // Write to file
    let json_string = serde_json::to_string_pretty(&updated_settings)
        .map_err(|e| format!("Failed to serialize settings: {}", e))?;

    std::fs::write(&settings_path, json_string)
        .map_err(|e| format!("Failed to write settings file: {}", e))?;

    // Return the found PHP path (if any) so frontend can emit the event
    Ok(found_php_path.unwrap_or_default())
}

// Convert Settings struct to frontend format (camelCase)
pub fn settings_to_frontend(settings: &Settings) -> serde_json::Value {
    serde_json::json!({
        "version": settings.version,
        "laravelPath": settings.laravel_path,
        "php": settings.php,
        "theme": settings.theme,
        "editorFontSize": settings.editor_font_size,
        "editorWordWrap": settings.editor_word_wrap,
        "layout": settings.layout,
        "output": settings.output,
        "vimMode": settings.vim_mode,
        "stackedDump": settings.stacked_dump,
        "windowWidth": settings.window_width,
        "windowHeight": settings.window_height,
        "intelephenseLicenseKey": settings.intelephense_license_key,
        "navigationDisplay": settings.navigation_display,
    })
}

