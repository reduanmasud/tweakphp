use crate::client::{traits::Client, ConnectionConfig};
use tauri::AppHandle;
use serde_yaml::Value as YamlValue;
use base64::Engine;

pub struct VaporClient {
    connection: ConnectionConfig,
    #[allow(dead_code)]
    app: AppHandle,
}

impl VaporClient {
    pub fn new(config: ConnectionConfig, app: AppHandle) -> Result<Self, String> {
        match config {
            ConnectionConfig::Vapor { .. } => Ok(Self {
                connection: config,
                app,
            }),
            _ => Err("Invalid connection config for VaporClient".to_string()),
        }
    }

    fn remove_blacklisted_tokens(&self, code: &str) -> String {
        let blacklist = vec!["<?php", "?>"];
        let mut cleaned = code.to_string();
        for token in blacklist {
            cleaned = cleaned.replace(token, "");
        }
        cleaned
    }
}

#[async_trait::async_trait]
impl Client for VaporClient {
    async fn connect(&mut self) -> Result<(), String> {
        Ok(())
    }

    async fn setup(&mut self) -> Result<(), String> {
        Ok(())
    }

    async fn execute(&mut self, code: String, loader: Option<String>) -> Result<String, String> {
        let ConnectionConfig::Vapor { ref client_path, ref environment, .. } = self.connection else {
            return Err("Invalid connection config".to_string());
        };

        if loader.is_some() {
            return Ok("The loader option is not supported in connection type vapor.".to_string());
        }

        let client_path = client_path.as_ref().ok_or("Missing client path in connection configuration")?;
        let env = environment.as_ref().map(|s| s.as_str()).unwrap_or("local");

        let cleaned_code = self.remove_blacklisted_tokens(&code);
        let encoded = base64::engine::general_purpose::STANDARD.encode(cleaned_code.as_bytes());
        let command = format!("vapor tinker {} -n --code 'eval(base64_decode(\"{}\"));'", env, encoded);

        let output = tokio::process::Command::new("sh")
            .arg("-c")
            .arg(&command)
            .current_dir(client_path)
            .output()
            .await
            .map_err(|e| format!("Failed to execute vapor command: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Ok(format!("Error: {}", stderr));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.to_string())
    }

    async fn info(&mut self, _loader: Option<String>) -> Result<String, String> {
        Ok("{}".to_string())
    }

    async fn action(&mut self, action_type: String, _data: Option<serde_json::Value>) -> Result<serde_json::Value, String> {
        match action_type.as_str() {
            "getEnvironments" => {
                let ConnectionConfig::Vapor { ref client_path, .. } = self.connection else {
                    return Err("Invalid connection config".to_string());
                };

                let client_path = client_path.as_ref().ok_or("Missing client path")?;
                let vapor_path = std::path::Path::new(client_path).join("vapor.yml");

                if !vapor_path.exists() {
                    return Ok(serde_json::json!([]));
                }

                let content = std::fs::read_to_string(&vapor_path)
                    .map_err(|e| format!("Failed to read vapor.yml: {}", e))?;
                
                let parsed: YamlValue = serde_yaml::from_str(&content)
                    .map_err(|e| format!("Failed to parse vapor.yml: {}", e))?;

                if let YamlValue::Mapping(map) = parsed {
                    if let Some(YamlValue::Mapping(environments)) = map.get(&YamlValue::String("environments".to_string())) {
                        let env_names: Vec<String> = environments.keys()
                            .filter_map(|k| k.as_str().map(|s| s.to_string()))
                            .collect();
                        return Ok(serde_json::json!(env_names));
                    }
                }

                Ok(serde_json::json!([]))
            }
            _ => Err(format!("Unknown action type: {}", action_type)),
        }
    }

    async fn disconnect(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn get_connection(&self) -> ConnectionConfig {
        self.connection.clone()
    }
}

