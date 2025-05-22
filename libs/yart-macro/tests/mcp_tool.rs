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
