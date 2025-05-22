mod common;

#[cfg(not(test))]
mod mcp_macro;
#[cfg(test)]
pub mod mcp_macro;

mod rig_macro;

extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn rig_tool(attr: TokenStream, item: TokenStream) -> TokenStream {
    rig_macro::rig_tool(attr, item)
}

#[proc_macro_attribute]
pub fn mcp_tool(args: TokenStream, input_fn: TokenStream) -> TokenStream {
    mcp_macro::mcp_tool(args.into(), input_fn.into()).into()
}
