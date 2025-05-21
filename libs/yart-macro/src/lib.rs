use proc_macro::TokenStream;
use quote::quote;

// Shared utilities
mod common;

// mcp_tool implementation
#[proc_macro_attribute]
pub fn mcp_tool(attr: TokenStream, item: TokenStream) -> TokenStream {
    let output = quote! {};

    output.into()
}

#[proc_macro_attribute]
pub fn rig_tool(attr: TokenStream, item: TokenStream) -> TokenStream {
    let output = quote! {};

    output.into()
}
