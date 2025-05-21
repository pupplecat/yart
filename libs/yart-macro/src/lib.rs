mod common;
mod mcp_macro;
mod rig_macro;

extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn rig_tool(attr: TokenStream, item: TokenStream) -> TokenStream {
    rig_macro::rig_tool(attr, item)
}

#[proc_macro_attribute]
pub fn mcp_tool(attr: TokenStream, item: TokenStream) -> TokenStream {
    mcp_macro::mcp_tool(attr, item)
}
