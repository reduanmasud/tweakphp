use crate::client::ConnectionConfig;

#[async_trait::async_trait]
pub trait Client: Send + Sync {
    async fn connect(&mut self) -> Result<(), String>;
    async fn setup(&mut self) -> Result<(), String>;
    async fn execute(&mut self, code: String, loader: Option<String>) -> Result<String, String>;
    async fn info(&mut self, loader: Option<String>) -> Result<String, String>;
    async fn action(&mut self, action_type: String, data: Option<serde_json::Value>) -> Result<serde_json::Value, String>;
    async fn disconnect(&mut self) -> Result<(), String>;
    fn get_connection(&self) -> ConnectionConfig;
}

