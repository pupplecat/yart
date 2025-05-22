// #[cfg(test)]
// mod test_mcp_tool {
//     use anyhow::Result;
//     use syn::parse_quote;

//     #[test]
//     fn get_code() -> Result<()> {
//         let input_fn = parse_quote! {
//             async fn test_tool(args: MyArgs) -> Result<()> {
//                 Ok(())
//             }
//         };
//         let args = parse_quote! { description = "Test" };

//         let ret = yart_macro::mcp_macro::mcp_tool(args, input_fn);
//         println!("{}", ret);
//         Ok(())
//     }
// }

use std::sync::Arc;

use anyhow::Result;
use mcp_core::types::{TextContent, ToolResponseContent};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use yart::mcp_tool;

#[derive(Clone)]
pub struct SearchToolContext {
    value: String,
}

#[derive(Default, Deserialize, Serialize, JsonSchema)]
pub struct SearchToolArgs {
    value: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SearchToolResponse {
    result: String,
}

impl Into<ToolResponseContent> for SearchToolResponse {
    fn into(self) -> ToolResponseContent {
        ToolResponseContent::Text(TextContent {
            content_type: "text".to_string(),
            text: "Hello, world!".to_string(),
            annotations: None,
        })
    }
}

#[mcp_tool(description = "Brand new tool", name = "search_tool")]
async fn search_tool(
    ctx: Arc<SearchToolContext>,
    args: SearchToolArgs,
) -> Result<SearchToolResponse> {
    Ok(SearchToolResponse {
        result: "response".to_string(),
    })
}
