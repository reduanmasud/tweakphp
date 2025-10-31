use crate::client::{traits::Client, ConnectionConfig, utils};
use tauri::AppHandle;
use std::process::Command;
use std::path::PathBuf;

pub struct SshClient {
    connection: ConnectionConfig,
    app: AppHandle,
    ssh_config: Option<SshConfig>,
}

struct SshConfig {
    host: String,
    port: u16,
    username: String,
    auth_type: String,
    #[allow(dead_code)]
    password: Option<String>,
    private_key: Option<PathBuf>,
    #[allow(dead_code)]
    passphrase: Option<String>,
}

impl SshClient {
    pub fn new(config: ConnectionConfig, app: AppHandle) -> Result<Self, String> {
        match config {
            ConnectionConfig::Ssh { ref host, ref port, ref username, ref auth_type, ref password, ref private_key, ref passphrase, .. } => {
                let ssh_config = Some(SshConfig {
                    host: host.clone(),
                    port: *port,
                    username: username.clone(),
                    auth_type: auth_type.clone(),
                    password: password.clone(),
                    private_key: private_key.as_ref().map(|p| PathBuf::from(p)),
                    passphrase: passphrase.clone(),
                });
                
                Ok(Self {
                    connection: config,
                    app,
                    ssh_config,
                })
            }
            _ => Err("Invalid connection config for SshClient".to_string()),
        }
    }

    fn build_ssh_command(&self, remote_command: &str) -> Result<Command, String> {
        let config = self.ssh_config.as_ref().ok_or("SSH config not available")?;
        
        let mut cmd = Command::new("ssh");
        
        // Add port
        cmd.arg("-p").arg(config.port.to_string());
        
        // Add identity file if using key auth
        if config.auth_type == "key" {
            if let Some(ref key_path) = config.private_key {
                cmd.arg("-i").arg(key_path);
            }
        }
        
        // For password auth, we'll use sshpass if available
        // Otherwise, user will need to use key-based auth or expect password prompts
        if config.auth_type == "password" {
            // Note: Password-based SSH requires sshpass or expect script
            // For now, we'll rely on SSH key-based auth or user input
            // This could be enhanced with sshpass integration
        }
        
        // Add SSH options
        cmd.arg("-o").arg("StrictHostKeyChecking=no");
        cmd.arg("-o").arg("UserKnownHostsFile=/dev/null");
        
        // Build connection string
        let connection_string = format!("{}@{}", config.username, config.host);
        cmd.arg(connection_string);
        cmd.arg(remote_command);
        
        Ok(cmd)
    }

    fn exec_command(&self, command: &str) -> Result<String, String> {
        let mut cmd = self.build_ssh_command(command)?;
        
        let output = cmd.output()
            .map_err(|e| format!("Failed to execute SSH command: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("SSH command failed: {}", stderr));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    fn upload_file(&self, local_path: &str, remote_path: &str) -> Result<(), String> {
        let config = self.ssh_config.as_ref().ok_or("SSH config not available")?;
        
        let mut cmd = Command::new("scp");
        cmd.arg("-P").arg(config.port.to_string());
        
        if config.auth_type == "key" {
            if let Some(ref key_path) = config.private_key {
                cmd.arg("-i").arg(key_path);
            }
        }
        
        cmd.arg("-o").arg("StrictHostKeyChecking=no");
        cmd.arg("-o").arg("UserKnownHostsFile=/dev/null");
        
        cmd.arg(local_path);
        let remote_file = format!("{}@{}:{}", config.username, config.host, remote_path);
        cmd.arg(&remote_file);
        
        let output = cmd.output()
            .map_err(|e| format!("Failed to upload file via SCP: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("SCP upload failed: {}", stderr));
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl Client for SshClient {
    async fn connect(&mut self) -> Result<(), String> {
        // Test connection by running a simple command
        self.exec_command("echo 'connected'")
            .map_err(|e| format!("Failed to connect to SSH server: {}", e))?;
        Ok(())
    }

    async fn setup(&mut self) -> Result<(), String> {
        // Extract mutable references first, then use them
        let path = match &self.connection {
            ConnectionConfig::Ssh { path, .. } => path.clone(),
            _ => return Err("Invalid connection config".to_string()),
        };

        // Check if path exists
        let check_path = self.exec_command(&format!("[ -d \"{}\" ] || echo \"not_found\"", path))?;
        if check_path.trim() == "not_found" {
            return Err("Path not found".to_string());
        }

        // Check if PHP exists
        let check_php = self.exec_command("which php")?;
        if check_php.trim().is_empty() {
            return Ok(());
        }

        // Get PHP version
        let php_version_output = self.exec_command("php -r \"echo PHP_MAJOR_VERSION . '.' . PHP_MINOR_VERSION . PHP_EOL;\"")?;
        let php_version = php_version_output.trim();
        if php_version.parse::<f64>().unwrap_or(0.0) < 7.4 {
            return Err("PHP version must be 7.4 or higher".to_string());
        }

        // Now update the connection
        if let ConnectionConfig::Ssh { ref mut php, .. } = self.connection {
            *php = Some(php_version.to_string());
        }

        // Upload PHAR client if needed
        let home_path = self.exec_command("echo $HOME")?.trim().to_string();
        let phar_client_remote_path = format!("{}/.tweakphp/client-{}.phar", home_path, php_version);
        
        let check_client = self.exec_command(&format!("[ -e \"{}\" ] || echo \"not_found\"", phar_client_remote_path))?;
        if check_client.trim() == "not_found" {
            self.exec_command(&format!("mkdir -p {}/.tweakphp", home_path))?;
            
            let phar_client_local = utils::get_local_phar_client(&self.app, php_version)?;
            self.upload_file(phar_client_local.to_str().unwrap(), &phar_client_remote_path)?;
        }

        // Update client_path
        if let ConnectionConfig::Ssh { ref mut client_path, .. } = self.connection {
            *client_path = Some(phar_client_remote_path);
        }

        Ok(())
    }

    async fn execute(&mut self, code: String, loader: Option<String>) -> Result<String, String> {
        let ConnectionConfig::Ssh { ref php, ref path, ref client_path, .. } = self.connection else {
            return Err("Invalid connection config".to_string());
        };

        let php_path = php.as_ref().map(|s| s.as_str()).unwrap_or("php");
        let client_path = client_path.as_ref().ok_or("Client path not found")?;
        
        let encoded_code = utils::base64_encode(&code);
        let mut command = format!("{} {} {} execute {}", php_path, client_path, path, encoded_code);
        
        if let Some(ref loader) = loader {
            let encoded_loader = utils::base64_encode(loader);
            command.push_str(&format!(" --loader={}", encoded_loader));
        }

        self.exec_command(&command)
    }

    async fn info(&mut self, loader: Option<String>) -> Result<String, String> {
        let ConnectionConfig::Ssh { ref php, ref path, ref client_path, .. } = self.connection else {
            return Err("Invalid connection config".to_string());
        };

        if php.is_none() || client_path.is_none() {
            return Ok("{}".to_string());
        }

        let php_path = php.as_ref().unwrap();
        let client_path = client_path.as_ref().unwrap();
        
        let mut command = format!("{} {} {} info", php_path, client_path, path);
        
        if let Some(ref loader) = loader {
            let encoded_loader = utils::base64_encode(loader);
            command.push_str(&format!(" --loader={}", encoded_loader));
        }

        self.exec_command(&command)
    }

    async fn action(&mut self, _action_type: String, _data: Option<serde_json::Value>) -> Result<serde_json::Value, String> {
        Err("SSH client does not support actions".to_string())
    }

    async fn disconnect(&mut self) -> Result<(), String> {
        // No-op for SSH via command line
        Ok(())
    }

    fn get_connection(&self) -> ConnectionConfig {
        self.connection.clone()
    }
}

