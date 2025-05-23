use std::sync::Arc;

use anyhow::Result;
use mcp_core::{
    server::Server,
    types::{ServerCapabilities, TextContent, ToolCapabilities, ToolResponseContent},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Context structure for tool execution, holding shared state.
#[derive(Clone)]
pub struct ToolContext {
    value: String,
}

/// Input arguments for the tool, deserialized from the request.
#[derive(Default, Deserialize, Serialize, JsonSchema)]
pub struct ToolInput {
    value: String,
}

/// Output response from the tool, convertible to ToolResponseContent.
#[derive(Deserialize, Serialize, JsonSchema)]
pub struct ToolOutput {
    result: String,
}

impl Into<Vec<ToolResponseContent>> for ToolOutput {
    fn into(self) -> Vec<ToolResponseContent> {
        vec![ToolResponseContent::Text(TextContent {
            content_type: "text".to_string(),
            text: self.result,
            annotations: None,
        })]
    }
}

/// Executes a search tool with context and input arguments.
#[yart::mcp_tool(description = "Brand new tool")]
async fn execute_search(ctx: Arc<ToolContext>, args: ToolInput) -> Result<ToolOutput> {
    Ok(ToolOutput {
        result: format!("{}, {}", ctx.value, args.value),
    })
}

/// Executes a search tool with only input arguments.
#[yart::mcp_tool(description = "Brand new tool")]
async fn execute_search_without_context(args: ToolInput) -> Result<ToolOutput> {
    Ok(ToolOutput {
        result: format!("{}", args.value),
    })
}

/// Executes a search tool with only context.
#[yart::mcp_tool(description = "Brand new tool", context_arg = true)]
async fn execute_search_without_argument(ctx: Arc<ToolContext>) -> Result<ToolOutput> {
    Ok(ToolOutput {
        result: format!("{}", ctx.value),
    })
}

/// Executes a search tool with no arguments or context.
#[yart::mcp_tool(description = "Brand new tool", context_arg = true)]
async fn execute_search_bare() -> Result<ToolOutput> {
    Ok(ToolOutput {
        result: "Hello world".to_string(),
    })
}

#[tokio::test]
async fn test_mcp_tool_basic() -> Result<()> {
    let tool = ExecuteSearchMcp::tool();

    assert_eq!(tool.name, "execute_search");
    assert_eq!(tool.description.unwrap(), "Brand new tool");

    Ok(())
}

#[tokio::test]
async fn test_build_server() -> Result<()> {
    let ctx = Arc::new(ToolContext {
        value: "new tool".to_string(),
    });

    let _server_protocol = Server::builder(
        "echo".to_string(),
        "1.0".to_string(),
        mcp_core::types::ProtocolVersion::V2024_11_05,
    )
    .set_capabilities(ServerCapabilities {
        tools: Some(ToolCapabilities::default()),
        ..Default::default()
    })
    .register_tool(
        ExecuteSearchWithoutArgumentMcp::tool(),
        ExecuteSearchWithoutArgumentMcp::call(ctx.clone()),
    )
    .build();

    Ok(())
}
