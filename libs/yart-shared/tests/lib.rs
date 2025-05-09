use anyhow::{anyhow, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use yart_shared::{derive_parameters, wrap_unsafe, ToolError, ToolOutput};

#[tokio::test]
async fn test_wrap_unsafe_success() {
    async fn sample_async() -> Result<String> {
        Ok("Success".to_string())
    }

    let result = wrap_unsafe(sample_async).await;
    assert_eq!(result.unwrap(), "Success");
}

#[tokio::test]
async fn test_wrap_unsafe_error() {
    async fn sample_async() -> Result<String> {
        Err(anyhow!("Test error"))
    }

    let result = wrap_unsafe(sample_async).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Test error");
}

#[test]
fn test_tool_error_new() {
    let error = ToolError::new("Custom error");
    assert_eq!(error.0, "Custom error");
    assert_eq!(error.to_string(), "Custom error");
}

#[test]
fn test_tool_error_from_anyhow() {
    let anyhow_error = anyhow!("Anyhow error");
    let tool_error = ToolError::from(anyhow_error);
    assert_eq!(tool_error.0, "Anyhow error");
    assert_eq!(tool_error.to_string(), "Anyhow error");
}

#[test]
fn test_tool_error_from_boxed_error() {
    let boxed_error: Box<dyn std::error::Error + Send + Sync + 'static> = Box::new(
        std::io::Error::new(std::io::ErrorKind::Other, "Boxed error"),
    );
    let tool_error = ToolError::from(boxed_error);
    assert_eq!(tool_error.0, "Boxed error");
    assert_eq!(tool_error.to_string(), "Boxed error");
}

#[test]
fn test_tool_output_serialization() {
    let output = ToolOutput {
        result: json!({ "key": "value" }),
    };
    let serialized = serde_json::to_string(&output).unwrap();
    assert_eq!(serialized, r#"{"result":{"key":"value"}}"#);

    let deserialized: ToolOutput = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized.result, json!({ "key": "value" }));
}

#[test]
fn test_derive_parameters() {
    #[derive(Serialize, Deserialize, JsonSchema)]
    struct TestArgs {
        input: String,
        count: i32,
    }

    let schema = derive_parameters::<TestArgs>();
    let expected = json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "title": "TestArgs",
        "type": "object",
        "required": ["count", "input"],
        "properties": {
            "input": { "type": "string" },
            "count": { "type": "integer", "format": "int32" }
        }
    });
    assert_eq!(schema, expected);
}

#[tokio::test]
async fn test_wrap_unsafe_with_context() {
    #[derive(Clone)]
    struct TestContext {
        value: String,
    }

    async fn sample_async(ctx: Arc<TestContext>) -> Result<String> {
        Ok(ctx.value.clone())
    }

    let ctx = Arc::new(TestContext {
        value: "Context".to_string(),
    });
    let ctx_clone = Arc::clone(&ctx);
    let result = wrap_unsafe(move || sample_async(ctx_clone)).await;
    assert_eq!(result.unwrap(), "Context");
}
