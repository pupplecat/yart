use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fmt::format, sync::Arc};
use yart::ToolError;

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

#[derive(Serialize, Deserialize, schemars::JsonSchema)]
struct TestOutput {
    value: String,
}

#[yart::mcp_tool(
    description = "Test tool",
    read_only_hint = false,
    destructive_hint = true
)]
async fn example_tool(
    ctx: Arc<TestContext>,
    args: TestArgs,
) -> anyhow::Result<TestOutput, ToolError> {
    Ok(TestOutput {
        value: format!("Echo: {} {}", ctx.value, args.param),
    })
}

// #[tokio::test]
// async fn test_mcp_tool() {
//     let handler = ExampleTool::handler();
//     let req = mcp_core::types::CallToolRequest {
//         name: "example_tool".to_string(),
//         arguments: Some(HashMap::from_iter(vec![
//             ("required_param".to_string(), json!("input")),
//             ("internal_param".to_string(), json!("hidden")),
//         ])),
//         meta: None,
//     };
//     let response = (handler.f)(req).await;
//     assert_eq!(response.is_error, None);
//     assert_eq!(response.content.len(), 1);
//     if let mcp_core::types::ToolResponseContent::Text(text) = &response.content[0] {
//         assert_eq!(text.text, ": input  hidden");
//     } else {
//         panic!("Expected Text content");
//     }
// }

// #[tokio::test]
// async fn test_mcp_tool_error() {
//     #[mcp_tool(description = "Error tool")]
//     async fn error_tool(
//         _args: TestArgs,
//     ) -> Result<mcp_core::types::ToolResponseContent, anyhow::Error> {
//         Err(anyhow::anyhow!("Tool failed"))
//     }

//     let handler = ErrorTool::handler();
//     let req = mcp_core::types::CallToolRequest {
//         name: "error_tool".to_string(),
//         arguments: Some(HashMap::from_iter(vec![(
//             "param".to_string(),
//             json!("input"),
//         )])),
//         meta: None,
//     };
//     let response = (handler.f)(req).await;
//     assert_eq!(response.is_error, Some(true));
//     assert_eq!(response.content.len(), 1);
//     if let mcp_core::types::ToolResponseContent::Text(text) = &response.content[0] {
//         assert_eq!(text.text, "Tool execution error: Tool failed");
//     } else {
//         panic!("Expected Text content");
//     }
// }

// #[yart::mcp_tool(description = "No args tool")]
// async fn no_args_tool() -> Result<mcp_core::types::ToolResponseContent, anyhow::Error> {
//     Ok(mcp_core::types::ToolResponseContent::Text(
//         mcp_core::types::TextContent {
//             content_type: "text".to_string(),
//             text: "no args".to_string(),
//             annotations: None,
//         },
//     ))
// }

// #[tokio::test]
// async fn test_no_args_mcp_tool() {
//     let handler = NoArgsTool::handler();
//     let req = mcp_core::types::CallToolRequest {
//         name: "no_args_tool".to_string(),
//         arguments: None,
//         meta: None,
//     };
//     let response = (handler.f)(req).await;
//     assert_eq!(response.is_error, None);
//     assert_eq!(response.content.len(), 1);
//     if let mcp_core::types::ToolResponseContent::Text(text) = &response.content[0] {
//         assert_eq!(text.text, "no args");
//     } else {
//         panic!("Expected Text content");
//     }
// }
