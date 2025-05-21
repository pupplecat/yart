use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use yart_macro::mcp_tool;

#[derive(Clone, Default)]
struct TestContext {
    value: String,
}

#[derive(Serialize, Deserialize, schemars::JsonSchema)]
struct TestArgs {
    param: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    optional_param: Option<String>,
}

#[mcp_tool(
    description = "Test tool",
    read_only_hint = false,
    destructive_hint = true
)]
async fn example_tool(
    ctx: Arc<TestContext>,
    required_param: mcp_core_macros::tool_param!(String, description = "A required parameter"),
    optional_param: mcp_core_macros::tool_param!(
        Option<String>,
        description = "An optional parameter"
    ),
    internal_param: mcp_core_macros::tool_param!(String, hidden),
) -> Result<mcp_core::types::ToolResponseContent, anyhow::Error> {
    Ok(mcp_core::types::ToolResponseContent::Text(
        mcp_core::types::TextContent {
            content_type: "text".to_string(),
            text: format!(
                "{}: {} {} {}",
                ctx.value,
                required_param,
                optional_param.unwrap_or_default(),
                internal_param
            ),
            annotations: None,
        },
    ))
}

#[tokio::test]
async fn test_mcp_tool() {
    let handler = ExampleTool::handler();
    let req = mcp_core::types::CallToolRequest {
        name: "example_tool".to_string(),
        arguments: Some(HashMap::from_iter(vec![
            ("required_param".to_string(), json!("input")),
            ("internal_param".to_string(), json!("hidden")),
        ])),
        meta: None,
    };
    let response = (handler.f)(req).await;
    assert_eq!(response.is_error, None);
    assert_eq!(response.content.len(), 1);
    if let mcp_core::types::ToolResponseContent::Text(text) = &response.content[0] {
        assert_eq!(text.text, ": input  hidden");
    } else {
        panic!("Expected Text content");
    }
}

#[tokio::test]
async fn test_mcp_tool_error() {
    #[mcp_tool(description = "Error tool")]
    async fn error_tool(
        _args: TestArgs,
    ) -> Result<mcp_core::types::ToolResponseContent, anyhow::Error> {
        Err(anyhow::anyhow!("Tool failed"))
    }

    let handler = ErrorTool::handler();
    let req = mcp_core::types::CallToolRequest {
        name: "error_tool".to_string(),
        arguments: Some(HashMap::from_iter(vec![(
            "param".to_string(),
            json!("input"),
        )])),
        meta: None,
    };
    let response = (handler.f)(req).await;
    assert_eq!(response.is_error, Some(true));
    assert_eq!(response.content.len(), 1);
    if let mcp_core::types::ToolResponseContent::Text(text) = &response.content[0] {
        assert_eq!(text.text, "Tool execution error: Tool failed");
    } else {
        panic!("Expected Text content");
    }
}

#[mcp_tool(description = "No args tool")]
async fn no_args_tool() -> Result<mcp_core::types::ToolResponseContent, anyhow::Error> {
    Ok(mcp_core::types::ToolResponseContent::Text(
        mcp_core::types::TextContent {
            content_type: "text".to_string(),
            text: "no args".to_string(),
            annotations: None,
        },
    ))
}

#[tokio::test]
async fn test_no_args_mcp_tool() {
    let handler = NoArgsTool::handler();
    let req = mcp_core::types::CallToolRequest {
        name: "no_args_tool".to_string(),
        arguments: None,
        meta: None,
    };
    let response = (handler.f)(req).await;
    assert_eq!(response.is_error, None);
    assert_eq!(response.content.len(), 1);
    if let mcp_core::types::ToolResponseContent::Text(text) = &response.content[0] {
        assert_eq!(text.text, "no args");
    } else {
        panic!("Expected Text content");
    }
}
