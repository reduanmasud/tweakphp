use crate::client::{traits::Client, ConnectionConfig, utils};
use tauri::AppHandle;
use std::process::Command;
use std::collections::HashMap;

pub struct DockerClient {
    connection: ConnectionConfig,
    app: AppHandle,
    #[allow(dead_code)]
    docker_path_cache: HashMap<String, String>,
}

impl DockerClient {
    pub fn new(config: ConnectionConfig, app: AppHandle) -> Result<Self, String> {
        match config {
            ConnectionConfig::Docker { .. } => Ok(Self {
                connection: config,
                app,
                docker_path_cache: HashMap::new(),
            }),
            _ => Err("Invalid connection config for DockerClient".to_string()),
        }
    }

    async fn get_docker_path(&self) -> Result<String, String> {
        // TODO: Implement SSH-based docker path detection
        // For now, assume local docker
        match Command::new("which").arg("docker").output() {
            Ok(output) => {
                let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                Ok(if path.is_empty() { "docker".to_string() } else { path })
            }
            Err(_) => Ok("docker".to_string()),
        }
    }

    fn parse_docker_error(error: &str) -> String {
        if let Some(end_index) = error.find("See 'docker") {
            error[..end_index].trim().to_string()
        } else {
            error.to_string()
        }
    }
}

#[async_trait::async_trait]
impl Client for DockerClient {
    async fn connect(&mut self) -> Result<(), String> {
        // TODO: Implement SSH connection if needed
        Ok(())
    }

    async fn setup(&mut self) -> Result<(), String> {
        let container_name = match &self.connection {
            ConnectionConfig::Docker { container_name, .. } => container_name.clone(),
            _ => return Err("Invalid connection config".to_string()),
        };

        if container_name.is_empty() {
            return Err("Container name is required".to_string());
        }

        let docker_path = self.get_docker_path().await?;

        // Get PHP version
        let php_version_cmd = format!(
            "{} exec {} php -r \"echo PHP_MAJOR_VERSION . '.' . PHP_MINOR_VERSION . PHP_EOL;\"",
            docker_path, container_name
        );

        let output = Command::new("sh")
            .arg("-c")
            .arg(&php_version_cmd)
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Self::parse_docker_error(&stderr));
        }

        let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if version.parse::<f64>().unwrap_or(0.0) < 7.4 {
            return Err("PHP version must be 7.4 or higher".to_string());
        }

        // Get PHP path
        let php_path_cmd = format!("{} exec {} which php", docker_path, container_name);
        let php_path_output = Command::new("sh")
            .arg("-c")
            .arg(&php_path_cmd)
            .output()
            .map_err(|e| format!("Failed to get PHP path: {}", e))?;

        if !php_path_output.status.success() {
            let stderr = String::from_utf8_lossy(&php_path_output.stderr);
            return Err(Self::parse_docker_error(&stderr));
        }

        let php_path_val = String::from_utf8_lossy(&php_path_output.stdout).trim().to_string();

        // Upload PHAR client
        let phar_client_local = utils::get_local_phar_client(&self.app, &version)?;
        let phar_path = format!("/tmp/client-{}.phar", version);

        let copy_cmd = format!(
            "{} cp \"{}\" {}:'{}'",
            docker_path, phar_client_local.display(), container_name, phar_path
        );

        let copy_output = Command::new("sh")
            .arg("-c")
            .arg(&copy_cmd)
            .output()
            .map_err(|e| format!("Failed to copy PHAR to container: {}", e))?;

        if !copy_output.status.success() {
            let stderr = String::from_utf8_lossy(&copy_output.stderr);
            return Err(Self::parse_docker_error(&stderr));
        }

        // Update connection fields
        if let ConnectionConfig::Docker { ref mut php_version, ref mut php_path, ref mut client_path, .. } = self.connection {
            *php_version = version;
            *php_path = php_path_val;
            *client_path = phar_path;
        }

        Ok(())
    }

    async fn execute(&mut self, code: String, loader: Option<String>) -> Result<String, String> {
        let ConnectionConfig::Docker { ref container_name, ref php_path, ref client_path, ref working_directory, .. } = self.connection else {
            return Err("Invalid connection config".to_string());
        };

        let docker_path = self.get_docker_path().await?;
        let encoded_code = utils::base64_encode(&code);
        let mut command = format!(
            "{} exec {} {} {} {} execute {}",
            docker_path, container_name, php_path, client_path, working_directory, encoded_code
        );

        if let Some(ref loader) = loader {
            let encoded_loader = utils::base64_encode(loader);
            command.push_str(&format!(" --loader={}", encoded_loader));
        }

        let output = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output()
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    async fn info(&mut self, loader: Option<String>) -> Result<String, String> {
        let ConnectionConfig::Docker { ref container_name, ref php_path, ref client_path, ref working_directory, .. } = self.connection else {
            return Err("Invalid connection config".to_string());
        };

        let docker_path = self.get_docker_path().await?;
        let mut command = format!(
            "{} exec {} {} {} {} info",
            docker_path, container_name, php_path, client_path, working_directory
        );

        if let Some(ref loader) = loader {
            let encoded_loader = utils::base64_encode(loader);
            command.push_str(&format!(" --loader={}", encoded_loader));
        }

        let output = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output()
            .map_err(|e| format!("Failed to execute docker info command: {}", e))?;

        Ok(String::from_utf8_lossy(&output.stdout).replace('\n', ""))
    }

    async fn action(&mut self, action_type: String, _data: Option<serde_json::Value>) -> Result<serde_json::Value, String> {
        let docker_path = self.get_docker_path().await?;
        
        match action_type.as_str() {
            "getContainers" => {
                let output = Command::new(&docker_path)
                    .arg("ps")
                    .arg("--format")
                    .arg("{{.ID}}|{{.Names}}|{{.Image}}")
                    .output()
                    .map_err(|e| format!("Failed to get containers: {}", e))?;

                let containers: Vec<serde_json::Value> = String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .filter_map(|line| {
                        let parts: Vec<&str> = line.split('|').collect();
                        if parts.len() == 3 {
                            Some(serde_json::json!({
                                "id": parts[0],
                                "name": parts[1],
                                "image": parts[2],
                            }))
                        } else {
                            None
                        }
                    })
                    .collect();

                Ok(serde_json::json!(containers))
            }
            "getPHPVersion" => {
                let ConnectionConfig::Docker { ref container_name, .. } = self.connection else {
                    return Err("Invalid connection config".to_string());
                };

                let php_version_cmd = format!(
                    "{} exec {} php -r \"echo PHP_MAJOR_VERSION . '.' . PHP_MINOR_VERSION . PHP_EOL;\"",
                    docker_path, container_name
                );

                let output = Command::new("sh")
                    .arg("-c")
                    .arg(&php_version_cmd)
                    .output()
                    .map_err(|e| format!("Failed to get PHP version: {}", e))?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(Self::parse_docker_error(&stderr));
                }

                let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
                Ok(serde_json::json!(version))
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

