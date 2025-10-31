use base64::{Engine as _, engine::general_purpose};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use tauri::path::BaseDirectory;

pub fn base64_encode(input: &str) -> String {
    // Match JavaScript's btoa behavior with URI encoding
    // JavaScript: btoa(encodeURIComponent(input).replace(/%([0-9A-F]{2})/g, (_, p1) => String.fromCharCode(parseInt(p1, 16))))
    // This means: encode URI component, then replace %XX with actual character, then base64 encode
    
    // Step 1: URI encode
    let uri_encoded = percent_encoding::percent_encode(
        input.as_bytes(),
        percent_encoding::NON_ALPHANUMERIC
    ).to_string();
    
    // Step 2: Replace %XX with actual character
    let mut decoded = String::new();
    let mut chars = uri_encoded.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '%' {
            if let (Some(d1), Some(d2)) = (chars.next(), chars.next()) {
                let hex_str = format!("{}{}", d1, d2);
                if let Ok(byte) = u8::from_str_radix(&hex_str, 16) {
                    decoded.push(byte as char);
                    continue;
                }
            }
            decoded.push(ch);
        } else {
            decoded.push(ch);
        }
    }
    
    // Step 3: Base64 encode
    general_purpose::STANDARD.encode(decoded.as_bytes())
}

pub fn get_local_phar_client(app: &AppHandle, php_version: &str) -> Result<PathBuf, String> {
    let phar_name = format!("client-{}.phar", php_version);
    
    // For development: try multiple locations
    if cfg!(debug_assertions) {
        // 1. Try relative to current working directory
        if let Ok(current_dir) = std::env::current_dir() {
            let phar_path = current_dir.join("public").join(&phar_name);
            if phar_path.exists() {
                return Ok(phar_path);
            }
            
            // Also try going up one level (in case we're in src-tauri directory)
            let phar_path = current_dir.parent().map(|p| p.join("public").join(&phar_name));
            if let Some(path) = phar_path {
                if path.exists() {
                    return Ok(path);
                }
            }
        }
        
        // 2. Try relative to executable location (for dev builds)
        if let Ok(exe_path) = std::env::current_exe() {
            if let Some(exe_dir) = exe_path.parent() {
                // Try going up to project root (assuming exe is in target/debug or similar)
                // Structure: target/debug/tweakphp-app -> target -> project_root
                let phar_path = exe_dir.parent()
                    .and_then(|p| p.parent())
                    .map(|p| p.join("public").join(&phar_name));
                if let Some(path) = phar_path {
                    if path.exists() {
                        return Ok(path);
                    }
                }
            }
        }
        
        // 3. Try relative path from current directory
        let phar_path = PathBuf::from("public").join(&phar_name);
        if phar_path.exists() {
            return Ok(phar_path);
        }
        
        // 4. Try finding project root by looking for package.json or tauri.conf.json
        if let Ok(mut current_dir) = std::env::current_dir() {
            loop {
                let phar_path = current_dir.join("public").join(&phar_name);
                if phar_path.exists() {
                    return Ok(phar_path);
                }
                
                // Check if this is the project root
                if current_dir.join("package.json").exists() || current_dir.join("tauri.conf.json").exists() {
                    break;
                }
                
                if let Some(parent) = current_dir.parent() {
                    current_dir = parent.to_path_buf();
                } else {
                    break;
                }
            }
        }
    }
    
    // For production: try resource directory
    if let Ok(resource_dir) = app.path().resolve("public", BaseDirectory::Resource) {
        let phar_path = resource_dir.join(&phar_name);
        if phar_path.exists() {
            return Ok(phar_path);
        }
    }
    
    // Fallback: try resource directory without subdirectory
    if let Ok(resource_dir) = app.path().resolve("", BaseDirectory::Resource) {
        let phar_path = resource_dir.join("public").join(&phar_name);
        if phar_path.exists() {
            return Ok(phar_path);
        }
    }
    
    // Final fallback: try current directory as last resort
    if let Ok(current_dir) = std::env::current_dir() {
        let phar_path = current_dir.join("public").join(&phar_name);
        if phar_path.exists() {
            return Ok(phar_path);
        }
    }
    
    Err(format!("PHAR client not found for PHP version {} at any of the checked locations", php_version))
}

pub fn get_php_version(php_path: &str) -> Result<String, String> {
    use std::process::Command;
    
    let output = Command::new(php_path)
        .arg("-r")
        .arg("echo PHP_MAJOR_VERSION . '.' . PHP_MINOR_VERSION . PHP_EOL;")
        .output()
        .map_err(|e| format!("Failed to execute PHP: {}", e))?;
    
    if !output.status.success() {
        return Err("Failed to get PHP version".to_string());
    }
    
    let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(version)
}
