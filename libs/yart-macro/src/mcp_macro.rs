use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_quote,
    punctuated::Punctuated,
    Expr, ExprLit, FnArg, Ident, ItemFn, Lit, Meta, Pat, Token, Type,
};

/// Attributes parsed from the `mcp_tool` macro invocation.
#[derive(Debug)]
struct ToolAttributes {
    name: Option<String>,
    description: Option<String>,
    context_arg: bool,
    metadata: ToolMetadata,
}

/// Represents a function parameter, including its identifier and type.
struct FunctionParameter {
    ident: Option<Box<Pat>>,
    ty: Type,
}

impl FunctionParameter {
    fn has_value(&self) -> bool {
        self.ident.is_some()
    }

    fn get_ident(&self) -> Box<Pat> {
        self.ident.clone().unwrap()
    }

    fn get_call_signature(&self) -> (TokenStream, TokenStream) {
        match self.has_value() {
            true => {
                let ctx_ident = self.ident.as_ref().unwrap();
                let ctx_ty = &self.ty;
                (
                    quote! {#ctx_ident: #ctx_ty},
                    quote! {let #ctx_ident = #ctx_ident.clone();},
                )
            }
            false => (quote! {}, quote! {}),
        }
    }
}

/// Metadata annotations for the tool, such as hints and titles.
#[derive(Debug, Default)]
struct ToolMetadata {
    title: Option<String>,
    read_only_hint: Option<bool>,
    destructive_hint: Option<bool>,
    idempotent_hint: Option<bool>,
    open_world_hint: Option<bool>,
}

impl Parse for ToolAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut name = None;
        let mut description = None;
        let mut context_arg = false;
        let mut metadata = ToolMetadata::default();

        let meta_list: Punctuated<Meta, Token![,]> = Punctuated::parse_terminated(input)?;

        for meta in meta_list {
            match meta {
                Meta::NameValue(nv) => {
                    let ident = nv.path.get_ident().unwrap().to_string();
                    match ident.as_str() {
                        "name" => {
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Str(lit_str),
                                ..
                            }) = nv.value
                            {
                                name = Some(lit_str.value());
                            } else {
                                return Err(syn::Error::new_spanned(
                                    nv.value,
                                    "Expected string literal",
                                ));
                            }
                        }
                        "description" => {
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Str(lit_str),
                                ..
                            }) = nv.value
                            {
                                description = Some(lit_str.value());
                            } else {
                                return Err(syn::Error::new_spanned(
                                    nv.value,
                                    "Expected string literal",
                                ));
                            }
                        }
                        "context_arg" => {
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Bool(lit_bool),
                                ..
                            }) = nv.value
                            {
                                context_arg = lit_bool.value;
                            } else {
                                return Err(syn::Error::new_spanned(
                                    nv.value,
                                    "Expected boolean literal",
                                ));
                            }
                        }
                        "read_only_hint" | "readOnlyHint" => {
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Bool(lit_bool),
                                ..
                            }) = nv.value
                            {
                                metadata.read_only_hint = Some(lit_bool.value);
                            } else {
                                return Err(syn::Error::new_spanned(
                                    nv.value,
                                    "Expected boolean literal",
                                ));
                            }
                        }
                        "destructive_hint" | "destructiveHint" => {
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Bool(lit_bool),
                                ..
                            }) = nv.value
                            {
                                metadata.destructive_hint = Some(lit_bool.value);
                            } else {
                                return Err(syn::Error::new_spanned(
                                    nv.value,
                                    "Expected boolean literal",
                                ));
                            }
                        }
                        "idempotent_hint" | "idempotentHint" => {
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Bool(lit_bool),
                                ..
                            }) = nv.value
                            {
                                metadata.idempotent_hint = Some(lit_bool.value);
                            } else {
                                return Err(syn::Error::new_spanned(
                                    nv.value,
                                    "Expected boolean literal",
                                ));
                            }
                        }
                        "open_world_hint" | "openWorldHint" => {
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Bool(lit_bool),
                                ..
                            }) = nv.value
                            {
                                metadata.open_world_hint = Some(lit_bool.value);
                            } else {
                                return Err(syn::Error::new_spanned(
                                    nv.value,
                                    "Expected boolean literal",
                                ));
                            }
                        }
                        _ => {
                            return Err(syn::Error::new_spanned(
                                nv.path,
                                format!("Unknown attribute: {}", ident),
                            ));
                        }
                    }
                }
                Meta::List(list) if list.path.is_ident("annotations") => {
                    let nested: Punctuated<Meta, Token![,]> =
                        list.parse_args_with(Punctuated::parse_terminated)?;
                    for meta in nested {
                        if let Meta::NameValue(nv) = meta {
                            let key = nv.path.get_ident().unwrap().to_string();
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Str(lit_str),
                                ..
                            }) = nv.value
                            {
                                if key == "title" {
                                    metadata.title = Some(lit_str.value());
                                } else {
                                    return Err(syn::Error::new_spanned(
                                        nv.path,
                                        format!("Unknown string annotation: {}", key),
                                    ));
                                }
                            } else if let Expr::Lit(ExprLit {
                                lit: Lit::Bool(lit_bool),
                                ..
                            }) = nv.value
                            {
                                match key.as_str() {
                                    "read_only_hint" | "readOnlyHint" => {
                                        metadata.read_only_hint = Some(lit_bool.value)
                                    }
                                    "destructive_hint" | "destructiveHint" => {
                                        metadata.destructive_hint = Some(lit_bool.value)
                                    }
                                    "idempotent_hint" | "idempotentHint" => {
                                        metadata.idempotent_hint = Some(lit_bool.value)
                                    }
                                    "open_world_hint" | "openWorldHint" => {
                                        metadata.open_world_hint = Some(lit_bool.value)
                                    }
                                    _ => {
                                        return Err(syn::Error::new_spanned(
                                            nv.path,
                                            format!("Unknown boolean annotation: {}", key),
                                        ))
                                    }
                                }
                            } else {
                                return Err(syn::Error::new_spanned(
                                    nv.value,
                                    "Expected string or boolean literal for annotation value",
                                ));
                            }
                        } else {
                            return Err(syn::Error::new_spanned(
                                meta,
                                "Expected name-value pair for annotation",
                            ));
                        }
                    }
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        meta,
                        "Expected name-value pair or list",
                    ))
                }
            }
        }

        Ok(ToolAttributes {
            name,
            description,
            context_arg,
            metadata,
        })
    }
}

/// Extracts input parameters (context and arguments) from the function signature.
fn extract_input(input_fn: &ItemFn, context_arg: bool) -> (FunctionParameter, FunctionParameter) {
    let inputs = &input_fn.sig.inputs;
    let (context, args, ctx_ident, args_ident) = match inputs.len() {
        0 => (None, None, None, None),
        1 => {
            let arg = inputs.first().unwrap();
            if let FnArg::Typed(pat_type) = arg {
                if context_arg {
                    (
                        Some(pat_type.ty.clone()),
                        None,
                        Some(pat_type.pat.clone()),
                        None,
                    )
                } else {
                    (
                        None,
                        Some(pat_type.ty.clone()),
                        None,
                        Some(pat_type.pat.clone()),
                    )
                }
            } else {
                panic!("Expected typed argument");
            }
        }
        2 => {
            let mut iter = inputs.iter();
            let ctx_arg = iter.next().unwrap();
            let args_arg = iter.next().unwrap();
            if let (FnArg::Typed(ctx_pat), FnArg::Typed(args_pat)) = (ctx_arg, args_arg) {
                (
                    Some(ctx_pat.ty.clone()),
                    Some(args_pat.ty.clone()),
                    Some(ctx_pat.pat.clone()),
                    Some(args_pat.pat.clone()),
                )
            } else {
                panic!("Expected typed arguments");
            }
        }
        _ => panic!("mcp_tool expects 0-2 arguments (context and/or args)"),
    };

    let args_ty = args.unwrap_or_else(|| parse_quote! { () });
    let ctx_ty = context.unwrap_or_else(|| parse_quote! { () });

    (
        FunctionParameter {
            ident: ctx_ident,
            ty: *ctx_ty,
        },
        FunctionParameter {
            ident: args_ident,
            ty: *args_ty,
        },
    )
}

/// Generates the call logic based on the presence of context and arguments.
fn generate_call_logic(
    ctx_parameter: &FunctionParameter,
    args_parameter: &FunctionParameter,
    fn_name: &Ident,
) -> TokenStream {
    match (ctx_parameter.has_value(), args_parameter.has_value()) {
        (true, true) => {
            let ctx_ident = ctx_parameter.get_ident();
            let args_ident = args_parameter.get_ident();
            let args_ty = &args_parameter.ty;
            quote! {
                let #args_ident: #args_ty = match serde_json::from_value(params) {
                    Ok(p) => p,
                    Err(e) => return mcp_core::types::CallToolResponse {
                        content: vec![mcp_core::types::ToolResponseContent::Text(
                            mcp_core::types::TextContent {
                                content_type: "text".to_string(),
                                text: format!("Invalid parameters: {}", e),
                                annotations: None,
                            }
                        )],
                        is_error: Some(true),
                        meta: req.meta,
                    },
                };
                #fn_name(#ctx_ident, #args_ident).await
            }
        }
        (true, false) => {
            let ctx_ident = ctx_parameter.get_ident();
            quote! {
                #fn_name(#ctx_ident).await
            }
        }
        (false, true) => {
            let args_ident = args_parameter.get_ident();
            let args_ty = &args_parameter.ty;
            quote! {
                let #args_ident: #args_ty = match serde_json::from_value(params) {
                    Ok(p) => p,
                    Err(e) => return mcp_core::types::CallToolResponse {
                        content: vec![mcp_core::types::ToolResponseContent::Text(
                            mcp_core::types::TextContent {
                                content_type: "text".to_string(),
                                text: format!("Invalid parameters: {}", e),
                                annotations: None,
                            }
                        )],
                        is_error: Some(true),
                        meta: req.meta,
                    },
                };
                #fn_name(#args_ident).await
            }
        }
        (false, false) => quote! {
            #fn_name().await
        },
    }
}

pub fn mcp_tool(args: TokenStream, input_fn: TokenStream) -> TokenStream {
    let attrs = match syn::parse2::<ToolAttributes>(args) {
        Ok(args) => args,
        Err(e) => return e.to_compile_error().into(),
    };

    let input_fn = match syn::parse2::<ItemFn>(input_fn.clone()) {
        Ok(input_fn) => input_fn,
        Err(e) => return e.to_compile_error().into(),
    };

    let fn_name = &input_fn.sig.ident;
    let fn_name_str = fn_name.to_string();
    let struct_name = format_ident!("{}", fn_name_str.to_case(Case::Pascal));
    let tool_name = attrs.name.unwrap_or(fn_name_str.clone());
    let tool_description = attrs.description.unwrap_or_default();
    let context_arg = attrs.context_arg;

    let title = attrs.metadata.title.unwrap_or(fn_name_str.clone());
    let read_only_hint = attrs.metadata.read_only_hint.unwrap_or(false);
    let destructive_hint = attrs.metadata.destructive_hint.unwrap_or(true);
    let idempotent_hint = attrs.metadata.idempotent_hint.unwrap_or(false);
    let open_world_hint = attrs.metadata.open_world_hint.unwrap_or(true);

    let (ctx_parameter, args_parameter) = extract_input(&input_fn, context_arg);

    let (call_signature, ctx_clone) = ctx_parameter.get_call_signature();
    let call_body = generate_call_logic(&ctx_parameter, &args_parameter, fn_name);

    let args_ty = &args_parameter.ty;
    let expanded = quote! {
        #input_fn

        #[derive(Default)]
        pub struct #struct_name;

        impl #struct_name {
            pub fn tool() -> mcp_core::types::Tool {
                let schema = serde_json::to_value(schemars::schema_for!(#args_ty)).expect("Failed to serialize schema");

                let annotations = serde_json::json!({
                    "title": #title,
                    "readOnlyHint": #read_only_hint,
                    "destructiveHint": #destructive_hint,
                    "idempotentHint": #idempotent_hint,
                    "openWorldHint": #open_world_hint
                });

                mcp_core::types::Tool {
                    name: #tool_name.to_string(),
                    description: Some(#tool_description.to_string()),
                    input_schema: schema,
                    annotations: Some(mcp_core::types::ToolAnnotations {
                        title: Some(#title.to_string()),
                        read_only_hint: Some(#read_only_hint),
                        destructive_hint: Some(#destructive_hint),
                        idempotent_hint: Some(#idempotent_hint),
                        open_world_hint: Some(#open_world_hint),
                    }),
                }
            }

            pub fn call(#call_signature) -> mcp_core::tools::ToolHandlerFn {
                Box::new(move |req: mcp_core::types::CallToolRequest| {
                    #ctx_clone
                    Box::pin(async move {
                        let params = match req.arguments {
                            Some(args) => serde_json::to_value(args).unwrap_or_default(),
                            None => serde_json::Value::Null,
                        };

                        let call_response = { #call_body };

                        let call_tool_response = match call_response {
                            Ok(response) => {
                                mcp_core::types::CallToolResponse {
                                    content: response.into(),
                                    is_error: Some(false),
                                    meta: req.meta,
                                }
                            }
                            Err(e) => mcp_core::types::CallToolResponse {
                                content: vec![mcp_core::types::ToolResponseContent::Text(
                                    mcp_core::types::TextContent {
                                        content_type: "text".to_string(),
                                        text: format!("Tool execution error: {}", e),
                                        annotations: None,
                                    }
                                )],
                                is_error: Some(true),
                                meta: req.meta,
                            },
                        };
                        call_tool_response
                    })
                })
            }
        }
    };

    expanded.into()
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use proc_macro2::TokenStream;
    use quote::ToTokens;
    use syn::{parse_quote, Attribute, ItemFn};

    use super::{mcp_tool, ToolAttributes};

    fn get_inner_token_stream(attr: Attribute) -> Result<TokenStream> {
        let args_ts = match attr.meta {
            syn::Meta::List(syn::MetaList { tokens, .. }) => Ok(tokens),
            _ => Err(syn::Error::new_spanned(attr.meta, "Expected mcp_tool(...)")),
        }?;
        Ok(args_ts)
    }

    #[test]
    fn gen_code_with_context_and_args() -> Result<()> {
        let attr =
            parse_quote! { #[mcp_tool(description = "With context and args", context_arg = true)] };
        let args_ts = get_inner_token_stream(attr)?;
        let input_fn: ItemFn = parse_quote! { async fn test_tool(ctx: Arc<MyContext>, args: MyArgs) -> Result<MyResponse> { Ok(MyResponse) } };
        let ret = mcp_tool(args_ts.to_token_stream(), input_fn.to_token_stream());
        let ret_str = ret.to_string();
        assert!(ret_str.contains("pub struct TestToolMcp"));
        assert!(ret_str.contains("pub fn tool ("));
        assert!(ret_str.contains("pub fn call (ctx : Arc < MyContext >)"));
        Ok(())
    }

    #[test]
    fn gen_code_with_context_only() -> Result<()> {
        let attr =
            parse_quote! { #[mcp_tool(description = "With context only", context_arg = true)] };
        let args_ts = get_inner_token_stream(attr)?;
        let input_fn: ItemFn = parse_quote! { async fn test_tool(ctx: Arc<MyContext>) -> Result<MyResponse> { Ok(MyResponse) } };
        let ret = mcp_tool(args_ts.to_token_stream(), input_fn.to_token_stream());
        let ret_str = ret.to_string();
        assert!(ret_str.contains("pub struct TestToolMcp"));
        assert!(ret_str.contains("pub fn tool ("));
        assert!(ret_str.contains("pub fn call (ctx : Arc < MyContext >)"));
        Ok(())
    }

    #[test]
    fn gen_code_with_args_only() -> Result<()> {
        let attr = parse_quote! { #[mcp_tool(description = "With args only")] };
        let args_ts = get_inner_token_stream(attr)?;
        let input_fn: ItemFn = parse_quote! { async fn test_tool(args: MyArgs) -> Result<MyResponse> { Ok(MyResponse) } };
        let ret = mcp_tool(args_ts.to_token_stream(), input_fn.to_token_stream());
        let ret_str = ret.to_string();
        assert!(ret_str.contains("pub struct TestToolMcp"));
        assert!(ret_str.contains("pub fn tool ("));
        assert!(ret_str.contains("pub fn call ()"));
        Ok(())
    }

    #[test]
    fn gen_code_with_no_args() -> Result<()> {
        let attr = parse_quote! { #[mcp_tool(description = "No args")] };
        let args_ts = get_inner_token_stream(attr)?;
        let input_fn: ItemFn =
            parse_quote! { async fn test_tool() -> Result<MyResponse> { Ok(MyResponse) } };
        let ret = mcp_tool(args_ts.to_token_stream(), input_fn.to_token_stream());
        let ret_str = ret.to_string();
        assert!(ret_str.contains("pub struct TestToolMcp"));
        assert!(ret_str.contains("pub fn tool ("));
        assert!(ret_str.contains("pub fn call ()"));
        Ok(())
    }

    #[test]
    fn gen_code_handles_deserialization_error() -> Result<()> {
        let attr = parse_quote! { #[mcp_tool(description = "Test deserialization error")] };
        let args_ts = get_inner_token_stream(attr)?;
        let input_fn: ItemFn = parse_quote! { async fn test_tool(args: MyArgs) -> Result<MyResponse> { Ok(MyResponse) } };
        let ret = mcp_tool(args_ts.to_token_stream(), input_fn.to_token_stream());
        let ret_str = ret.to_string();
        assert!(ret_str.contains("match serde_json :: from_value (params)"));
        assert!(ret_str.contains("Invalid parameters"));
        assert!(ret_str.contains("is_error : Some (true)"));
        Ok(())
    }

    #[test]
    fn parse_description() -> Result<()> {
        let args = get_inner_token_stream(parse_quote! { #[mcp_tool(description = "TEST_XX")] })?;
        let attrs = syn::parse2::<ToolAttributes>(args.into())?;
        assert_eq!(attrs.name, None);
        assert_eq!(attrs.description, Some("TEST_XX".to_string()));
        assert_eq!(attrs.context_arg, false);
        assert_eq!(attrs.metadata.title, None);
        Ok(())
    }

    #[test]
    fn parse_name() -> Result<()> {
        let args = get_inner_token_stream(parse_quote! { #[mcp_tool(name = "TEST_XX")] })?;
        let attrs = syn::parse2::<ToolAttributes>(args.into())?;
        assert_eq!(attrs.name, Some("TEST_XX".to_string()));
        assert_eq!(attrs.description, None);
        assert_eq!(attrs.context_arg, false);
        assert_eq!(attrs.metadata.title, None);
        Ok(())
    }

    #[test]
    fn parse_context_arg() -> Result<()> {
        let args = get_inner_token_stream(parse_quote! { #[mcp_tool(context_arg = true)] })?;
        let attrs = syn::parse2::<ToolAttributes>(args.into())?;
        assert_eq!(attrs.context_arg, true);
        assert_eq!(attrs.name, None);
        assert_eq!(attrs.description, None);
        assert_eq!(attrs.metadata.title, None);
        Ok(())
    }

    #[test]
    fn parse_metadata() -> Result<()> {
        let args = get_inner_token_stream(
            parse_quote! { #[mcp_tool(annotations(title = "MyTool", read_only_hint = true, destructive_hint = false))] },
        )?;
        let attrs = syn::parse2::<ToolAttributes>(args.into())?;
        assert_eq!(attrs.metadata.title, Some("MyTool".to_string()));
        assert_eq!(attrs.metadata.read_only_hint, Some(true));
        assert_eq!(attrs.metadata.destructive_hint, Some(false));
        assert_eq!(attrs.metadata.idempotent_hint, None);
        assert_eq!(attrs.metadata.open_world_hint, None);
        Ok(())
    }

    #[test]
    fn parse_invalid_attribute_value() {
        let attr = parse_quote! { #[mcp_tool(description = 123)] };
        let args_ts = get_inner_token_stream(attr).unwrap();
        let result = syn::parse2::<ToolAttributes>(args_ts.into());
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Expected string literal"));
    }

    #[test]
    #[should_panic(expected = "mcp_tool expects 0-2 arguments")]
    fn gen_code_invalid_signature() {
        let attr = parse_quote! { #[mcp_tool(description = "Invalid signature")] };
        let args_ts = get_inner_token_stream(attr).unwrap();
        let input_fn: ItemFn =
            parse_quote! { async fn test_tool(a: A, b: B, c: C) -> Result<()> { Ok(()) } };
        mcp_tool(args_ts.to_token_stream(), input_fn.to_token_stream());
    }
}
