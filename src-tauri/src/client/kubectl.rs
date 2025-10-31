use crate::client::{traits::Client, ConnectionConfig, utils};
use tauri::AppHandle;
use std::process::Command;

pub struct KubectlClient {
    connection: ConnectionConfig,
    app: AppHandle,
}

impl KubectlClient {
    pub fn new(config: ConnectionConfig, app: AppHandle) -> Result<Self, String> {
        match config {
            ConnectionConfig::Kubectl { .. } => Ok(Self {
                connection: config,
                app,
            }),
            _ => Err("Invalid connection config for KubectlClient".to_string()),
        }
    }

    fn get_kubectl_path() -> String {
        match Command::new("which").arg("kubectl").output() {
            Ok(output) => {
                let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if path.is_empty() {
                    "kubectl".to_string()
                } else {
                    path
                }
            }
            Err(_) => "kubectl".to_string(),
        }
    }

    fn command(&self) -> String {
        let ConnectionConfig::Kubectl { ref php, ref path, ref client_path, .. } = self.connection else {
            return String::new();
        };

        let php_path = php.as_ref().map(|s| s.as_str()).unwrap_or("php");
        let client_path = client_path.as_ref().map(|s| s.as_str()).unwrap_or("");
        format!("{} {} {}", php_path, client_path, path)
    }
}

#[async_trait::async_trait]
impl Client for KubectlClient {
    async fn connect(&mut self) -> Result<(), String> {
        Ok(())
    }

    async fn setup(&mut self) -> Result<(), String> {
        let ConnectionConfig::Kubectl { ref mut php, ref pod, ref context, ref namespace, .. } = self.connection else {
            return Err("Invalid connection config".to_string());
        };

        // Get PHP version
        let kubectl_path = Self::get_kubectl_path();
        let php_version_cmd = format!(
            "{} exec {} --context=\"{}\" --namespace=\"{}\" -- php -r \"echo PHP_MAJOR_VERSION . '.' . PHP_MINOR_VERSION . PHP_EOL;\"",
            kubectl_path, pod, context, namespace
        );

        let output = Command::new("sh")
            .arg("-c")
            .arg(&php_version_cmd)
            .output()
            .map_err(|e| format!("Failed to execute kubectl command: {}", e))?;

        if !output.status.success() {
            return Err("Failed to get PHP version".to_string());
        }

        let php_version = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if php_version.parse::<f64>().unwrap_or(0.0) < 7.4 {
            return Err("PHP version must be at least 7.4".to_string());
        }

        *php = Some(php_version.clone());

        // Upload PHAR client if needed
        let ConnectionConfig::Kubectl { ref mut client_path, ref pod, ref context, ref namespace, .. } = self.connection else {
            return Err("Invalid connection config".to_string());
        };

        // Get home directory
        let home_cmd = format!(
            "{} exec {} --context=\"{}\" --namespace=\"{}\" -- sh -c 'echo $HOME'",
            kubectl_path, pod, context, namespace
        );

        let home_output = Command::new("sh")
            .arg("-c")
            .arg(&home_cmd)
            .output()
            .map_err(|e| format!("Failed to get home directory: {}", e))?;

        let home_path = String::from_utf8_lossy(&home_output.stdout).trim().to_string();
        let phar_client_remote_path = format!("{}/.tweakphp/client-{}.phar", home_path, php_version);

        // Check if client exists
        let check_cmd = format!(
            "{} exec {} --context=\"{}\" --namespace=\"{}\" -- [ -e \"{}\" ] || echo \"not_found\"",
            kubectl_path, pod, context, namespace, phar_client_remote_path
        );

        let check_output = Command::new("sh")
            .arg("-c")
            .arg(&check_cmd)
            .output()
            .map_err(|e| format!("Failed to check client path: {}", e))?;

        let check_result = String::from_utf8_lossy(&check_output.stdout).trim().to_string();

        if check_result == "not_found" {
            // Create directory
            let mkdir_cmd = format!(
                "{} exec {} --context=\"{}\" --namespace=\"{}\" -- mkdir -p {}/.tweakphp",
                kubectl_path, pod, context, namespace, home_path
            );
            Command::new("sh")
                .arg("-c")
                .arg(&mkdir_cmd)
                .output()
                .map_err(|e| format!("Failed to create directory: {}", e))?;

            // Upload PHAR file
            let phar_client_local = utils::get_local_phar_client(&self.app, &php_version)?;
            let upload_cmd = format!(
                "{} cp \"{}\" {}:{} --context=\"{}\" --namespace=\"{}\"",
                kubectl_path, phar_client_local.display(), pod, phar_client_remote_path, context, namespace
            );

            Command::new("sh")
                .arg("-c")
                .arg(&upload_cmd)
                .output()
                .map_err(|e| format!("Failed to upload PHAR client: {}", e))?;
        }

        *client_path = Some(phar_client_remote_path);

        Ok(())
    }

    async fn execute(&mut self, code: String, loader: Option<String>) -> Result<String, String> {
        let ConnectionConfig::Kubectl { ref pod, ref context, ref namespace, .. } = self.connection else {
            return Err("Invalid connection config".to_string());
        };

        let kubectl_path = Self::get_kubectl_path();
        let base_command = self.command();
        let encoded_code = utils::base64_encode(&code);
        let mut command = format!("{} execute {}", base_command, encoded_code);
        
        if let Some(ref loader) = loader {
            let encoded_loader = utils::base64_encode(loader);
            command.push_str(&format!(" --loader={}", encoded_loader));
        }

        let full_cmd = format!(
            "{} exec {} --context=\"{}\" --namespace=\"{}\" -- {}",
            kubectl_path, pod, context, namespace, command
        );

        let output = Command::new("sh")
            .arg("-c")
            .arg(&full_cmd)
            .output()
            .map_err(|e| format!("Failed to execute kubectl command: {}", e))?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    async fn info(&mut self, loader: Option<String>) -> Result<String, String> {
        let ConnectionConfig::Kubectl { ref pod, ref context, ref namespace, .. } = self.connection else {
            return Err("Invalid connection config".to_string());
        };

        let kubectl_path = Self::get_kubectl_path();
        let base_command = self.command();
        let mut command = format!("{} info", base_command);
        
        if let Some(ref loader) = loader {
            let encoded_loader = utils::base64_encode(loader);
            command.push_str(&format!(" --loader={}", encoded_loader));
        }

        let full_cmd = format!(
            "{} exec {} --context=\"{}\" --namespace=\"{}\" -- {}",
            kubectl_path, pod, context, namespace, command
        );

        let output = Command::new("sh")
            .arg("-c")
            .arg(&full_cmd)
            .output()
            .map_err(|e| format!("Failed to execute kubectl info command: {}", e))?;

        Ok(String::from_utf8_lossy(&output.stdout).replace('\n', ""))
    }

    async fn action(&mut self, action_type: String, data: Option<serde_json::Value>) -> Result<serde_json::Value, String> {
        let kubectl_path = Self::get_kubectl_path();
        
        match action_type.as_str() {
            "getContexts" => {
                let output = Command::new(&kubectl_path)
                    .arg("config")
                    .arg("get-contexts")
                    .arg("-o")
                    .arg("name")
                    .output()
                    .map_err(|e| format!("Failed to get contexts: {}", e))?;

                let contexts = String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>();
                
                Ok(serde_json::json!(contexts))
            }
            "getNamespaces" => {
                let context = data.and_then(|d| d.get("context").and_then(|c| c.as_str().map(|s| s.to_string())))
                    .ok_or("Missing context in data")?;

                let output = Command::new(&kubectl_path)
                    .arg("get")
                    .arg("namespaces")
                    .arg(format!("--context={}", context))
                    .arg("-o")
                    .arg("custom-columns=NAME:.metadata.name")
                    .arg("--no-headers")
                    .output()
                    .map_err(|e| format!("Failed to get namespaces: {}", e))?;

                let namespaces = String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>();
                
                Ok(serde_json::json!(namespaces))
            }
            "getPods" => {
                let context = data.as_ref().and_then(|d| d.get("context").and_then(|c| c.as_str().map(|s| s.to_string())))
                    .ok_or("Missing context in data")?;
                let namespace = data.as_ref().and_then(|d| d.get("namespace").and_then(|n| n.as_str().map(|s| s.to_string())))
                    .ok_or("Missing namespace in data")?;

                let output = Command::new(&kubectl_path)
                    .arg("get")
                    .arg("pods")
                    .arg(format!("--context={}", context))
                    .arg(format!("--namespace={}", namespace))
                    .arg("--field-selector=status.phase=Running")
                    .arg("-o")
                    .arg("custom-columns=NAME:.metadata.name")
                    .arg("--no-headers")
                    .output()
                    .map_err(|e| format!("Failed to get pods: {}", e))?;

                let pods = String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>();
                
                Ok(serde_json::json!(pods))
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

