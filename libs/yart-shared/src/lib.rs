use anyhow::{anyhow, Result};
use schemars::{schema_for, JsonSchema};
use serde::{Deserialize, Serialize};
use serde_json::{to_value, Value};
use std::future::Future;
use tokio::{spawn, sync::mpsc};

pub async fn wrap_unsafe<F, Fut, T>(f: F) -> Result<T>
where
    F: FnOnce() -> Fut + Send + 'static,
    Fut: Future<Output = anyhow::Result<T>> + Send + 'static,
    T: Send + 'static,
{
    let (tx, mut rx) = mpsc::channel(1);

    spawn(async move {
        let result = f().await;
        let _ = tx.send(result).await;
    });

    rx.recv().await.ok_or_else(|| anyhow!("Channel closed"))?
}

#[derive(Debug)]
pub struct ToolError(pub String);

impl ToolError {
    pub fn new(s: impl Into<String>) -> Self {
        ToolError(s.into())
    }
}

impl std::fmt::Display for ToolError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ToolError {}

impl From<anyhow::Error> for ToolError {
    fn from(e: anyhow::Error) -> Self {
        ToolError(e.to_string())
    }
}

impl From<Box<dyn std::error::Error + Send + Sync + 'static>> for ToolError {
    fn from(e: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        ToolError(e.to_string())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolOutput {
    pub result: Value,
}

pub fn derive_parameters<T: JsonSchema + for<'de> Deserialize<'de>>() -> serde_json::Value {
    to_value(schema_for!(T)).expect("Failed to serialize schema")
}
