pub mod local;
pub mod ssh;
pub mod docker;
pub mod kubectl;
pub mod vapor;
pub mod traits;
pub mod utils;
pub mod commands;

use serde::{Deserialize, Serialize};
use tauri::AppHandle;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ConnectionConfig {
    #[serde(rename = "local")]
    Local {
        php: String,
        path: String,
    },
    #[serde(rename = "ssh")]
    Ssh {
        id: u32,
        name: String,
        color: String,
        host: String,
        port: u16,
        username: String,
        #[serde(rename = "auth_type")]
        auth_type: String,
        password: Option<String>,
        #[serde(rename = "privateKey")]
        private_key: Option<String>,
        passphrase: Option<String>,
        path: String,
        php: Option<String>,
        #[serde(rename = "client_path")]
        client_path: Option<String>,
    },
    #[serde(rename = "docker")]
    Docker {
        #[serde(rename = "working_directory")]
        working_directory: String,
        #[serde(rename = "container_id")]
        container_id: String,
        #[serde(rename = "container_name")]
        container_name: String,
        #[serde(rename = "php_version")]
        php_version: String,
        #[serde(rename = "php_path")]
        php_path: String,
        #[serde(rename = "client_path")]
        client_path: String,
        #[serde(rename = "ssh_id")]
        ssh_id: u32,
        ssh: Option<Box<ConnectionConfig>>,
    },
    #[serde(rename = "kubectl")]
    Kubectl {
        id: u32,
        name: String,
        color: String,
        context: String,
        namespace: String,
        pod: String,
        path: String,
        php: Option<String>,
        #[serde(rename = "client_path")]
        client_path: Option<String>,
    },
    #[serde(rename = "vapor")]
    Vapor {
        id: u32,
        #[serde(rename = "client_path")]
        client_path: Option<String>,
        environment: Option<String>,
        environments: Vec<String>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientConnectPayload {
    pub connection: ConnectionConfig,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientExecutePayload {
    pub connection: ConnectionConfig,
    pub code: String,
    pub loader: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientActionPayload {
    pub connection: ConnectionConfig,
    pub type_field: String,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientInfoPayload {
    pub connection: ConnectionConfig,
    pub loader: Option<String>,
}

use traits::Client;

pub async fn get_client(config: ConnectionConfig, app: AppHandle) -> Result<Box<dyn Client>, String> {
    match config {
        ConnectionConfig::Local { .. } => {
            Ok(Box::new(local::LocalClient::new(config, app)?))
        }
        ConnectionConfig::Ssh { .. } => {
            Ok(Box::new(ssh::SshClient::new(config, app)?))
        }
        ConnectionConfig::Docker { .. } => {
            Ok(Box::new(docker::DockerClient::new(config, app)?))
        }
        ConnectionConfig::Kubectl { .. } => {
            Ok(Box::new(kubectl::KubectlClient::new(config, app)?))
        }
        ConnectionConfig::Vapor { .. } => {
            Ok(Box::new(vapor::VaporClient::new(config, app)?))
        }
    }
}

