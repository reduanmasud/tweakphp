use crate::client::{traits::Client, ConnectionConfig, utils};
use tauri::AppHandle;
use std::process::Command;

pub struct LocalClient {
    connection: ConnectionConfig,
    app: AppHandle,
}

impl LocalClient {
    pub fn new(config: ConnectionConfig, app: AppHandle) -> Result<Self, String> {
        match config {
            ConnectionConfig::Local { .. } => Ok(Self {
                connection: config,
                app,
            }),
            _ => Err("Invalid connection config for LocalClient".to_string()),
        }
    }
}

#[async_trait::async_trait]
impl Client for LocalClient {
    async fn connect(&mut self) -> Result<(), String> {
        Ok(())
    }

    async fn setup(&mut self) -> Result<(), String> {
        Ok(())
    }

    async fn execute(&mut self, code: String, loader: Option<String>) -> Result<String, String> {
        let ConnectionConfig::Local { ref php, ref path } = self.connection else {
            return Err("Invalid connection config".to_string());
        };

        // Get PHP version to determine which PHAR client to use
        let php_version = utils::get_php_version(php)
            .map_err(|e| format!("Failed to get PHP version: {}", e))?;
        
        let phar_client = utils::get_local_phar_client(&self.app, &php_version)?;
        
        let encoded_code = utils::base64_encode(&code);
        let mut command = Command::new(php);
        command.arg(phar_client.to_str().unwrap());
        command.arg(path);
        command.arg("execute");
        command.arg(&encoded_code);
        
        if let Some(ref loader) = loader {
            let encoded_loader = utils::base64_encode(loader);
            command.arg(format!("--loader={}", encoded_loader));
        }

        let output = command.output()
            .map_err(|e| format!("Failed to execute PHP command: {}", e))?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    async fn info(&mut self, loader: Option<String>) -> Result<String, String> {
        let ConnectionConfig::Local { ref php, ref path } = self.connection else {
            return Err("Invalid connection config".to_string());
        };

        let php_version = utils::get_php_version(php)
            .map_err(|e| format!("Failed to get PHP version: {}", e))?;
        
        let phar_client = utils::get_local_phar_client(&self.app, &php_version)?;
        
        let mut command = Command::new(php);
        command.arg(phar_client.to_str().unwrap());
        command.arg(path);
        command.arg("info");
        
        if let Some(ref loader) = loader {
            let encoded_loader = utils::base64_encode(loader);
            command.arg(format!("--loader={}", encoded_loader));
        }

        let output = command.output()
            .map_err(|e| format!("Failed to execute PHP info command: {}", e))?;

        if !output.status.success() {
            return Err(format!("PHP info command failed: {}", String::from_utf8_lossy(&output.stderr)));
        }

        Ok(String::from_utf8_lossy(&output.stdout).replace('\n', ""))
    }

    async fn action(&mut self, _action_type: String, _data: Option<serde_json::Value>) -> Result<serde_json::Value, String> {
        Err("Local client does not support actions".to_string())
    }

    async fn disconnect(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn get_connection(&self) -> ConnectionConfig {
        self.connection.clone()
    }
}

