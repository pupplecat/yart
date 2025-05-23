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

async fn execute_search_inner(ctx: Arc<ToolContext>, args: ToolInput) -> Result<ToolOutput> {
    Ok(ToolOutput {
        result: format!("{}, {}", ctx.value, args.value),
    })
}

/// Executes a search tool with context and input arguments.
#[yart::rig_tool(description = "Brand new tool")]
async fn execute_search_rig(ctx: Arc<ToolContext>, args: ToolInput) -> Result<ToolOutput> {
    execute_search_inner(ctx, args).await
}

#[yart::mcp_tool(description = "Brand new tool")]
async fn execute_search_mcp(ctx: Arc<ToolContext>, args: ToolInput) -> Result<ToolOutput> {
    execute_search_inner(ctx, args).await
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
        ExecuteSearchMcp::tool(),
        ExecuteSearchMcp::call(ctx.clone()),
    )
    .build();

    Ok(())
}
