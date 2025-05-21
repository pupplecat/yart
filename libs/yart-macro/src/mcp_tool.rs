use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, punctuated::Punctuated, Expr, ExprLit, FnArg, ItemFn, Lit,
    Meta, Pat, PatType, Token, Type,
};

use crate::common::{parse_inputs, parse_return_type, MacroArgs};

#[proc_macro_attribute]
pub fn mcp_tool(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as MacroArgs);
    let input_fn = parse_macro_input!(item as ItemFn);

    let description = args.description;
    let name = args.name;
    let context_arg = args.context_arg;
    let read_only_hint = args.read_only_hint.unwrap_or(false);
    let destructive_hint = args.destructive_hint.unwrap_or(true);
    let idempotent_hint = args.idempotent_hint.unwrap_or(false);
    let open_world_hint = args.open_world_hint.unwrap_or(true);

    let vis = &input_fn.vis;
    let fn_name = &input_fn.sig.ident;
    let fn_name_str = fn_name.to_string();
    let struct_name = syn::Ident::new(
        &yart_shared::common::to_upper_camel_case(&fn_name_str),
        fn_name.span(),
    );
    let tool_name = name.unwrap_or_else(|| fn_name_str.clone());
    let title = tool_name.clone();

    let (context, args, ctx_ident, args_ident) = parse_inputs(&input_fn.sig.inputs, context_arg);
    let args_ty = args
        .as_ref()
        .map_or_else(|| parse_quote! { () }, |ty| ty.clone());
    let ctx_ty = context
        .as_ref()
        .map_or_else(|| parse_quote! { () }, |ty| ty.clone());
    let return_ty = parse_return_type(&input_fn);
    let error_ty: Type = parse_quote! { anyhow::Error };

    let params_struct_name = syn::Ident::new(&format!("{}Parameters", struct_name), fn_name.span());

    let mut param_defs = Vec::new();
    let mut param_names = Vec::new();
    let mut required_params = Vec::new();
    let mut hidden_params = Vec::new();
    let mut param_descriptions = Vec::new();

    let params = if context_arg && input_fn.sig.inputs.len() == 2 {
        input_fn.sig.inputs.iter().skip(1).collect::<Vec<_>>()
    } else if !context_arg && input_fn.sig.inputs.len() >= 1 {
        input_fn.sig.inputs.iter().collect::<Vec<_>>()
    } else {
        Vec::new()
    };

    for arg in params {
        if let FnArg::Typed(PatType { pat, ty, .. }) = arg {
            let mut is_hidden = false;
            let mut description: Option<String> = None;
            let mut is_optional = false;

            // Check for tool_param macro
            if let Type::Macro(type_macro) = &**ty {
                if let Some(ident) = type_macro.mac.path.get_ident() {
                    if ident == "tool_param" {
                        if let Ok(tool_param_args) =
                            syn::parse2::<ToolParamArgs>(type_macro.mac.tokens.clone())
                        {
                            is_hidden = tool_param_args.hidden;
                            description = tool_param_args.description;

                            if let Type::Path(type_path) = &tool_param_args.ty {
                                is_optional = type_path
                                    .path
                                    .segments
                                    .last()
                                    .map_or(false, |segment| segment.ident == "Option");
                            }
                        }
                    }
                }
            }

            // Fallback: check if type is Option<T>
            if !is_optional {
                if let Type::Path(type_path) = &**ty {
                    is_optional = type_path
                        .path
                        .segments
                        .last()
                        .map_or(false, |segment| segment.ident == "Option");
                }
            }

            if let Pat::Ident(param_ident) = &**pat {
                let param_name = &param_ident.ident;
                let param_name_str = param_name.to_string();

                param_names.push(param_name.clone());
                param_defs.push(quote! { #param_name: #ty });

                if is_hidden {
                    hidden_params.push(param_name_str.clone());
                } else if !is_optional {
                    required_params.push(param_name_str.clone());
                }

                if let Some(desc) = description {
                    param_descriptions.push((param_name_str.clone(), desc));
                }
            }
        }
    }

    let input_schema = yart_shared::common::generate_input_schema(
        &args_ty,
        required_params.iter().map(|s| s.to_string()).collect(),
        param_descriptions
            .iter()
            .map(|(n, d)| (n.to_string(), d.to_string()))
            .collect(),
        hidden_params.iter().map(|s| s.to_string()).collect(),
    );

    let call_args = if context_arg {
        let ctx_ident = ctx_ident.as_ref().unwrap();
        quote! { #ctx_ident, #(params.#param_names),* }
    } else {
        quote! { #(params.#param_names),* }
    };

    let call_body = quote! {
        let params = match req.arguments {
            Some(args) => serde_json::to_value(args).unwrap_or_default(),
            None => serde_json::Value::Null,
        };

        let params: #params_struct_name = match serde_json::from_value(params) {
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

        let #ctx_ident = self.context.clone();
        match #struct_name::internal_call(#call_args).await {
            Ok(response) => {
                let content = if let Ok(vec_content) = serde_json::from_value::<Vec<mcp_core::types::ToolResponseContent>>(serde_json::to_value(&response).unwrap_or_default()) {
                    vec_content
                } else if let Ok(single_content) = serde_json::from_value::<mcp_core::types::ToolResponseContent>(serde_json::to_value(&response).unwrap_or_default()) {
                    vec![single_content]
                } else {
                    vec![mcp_core::types::ToolResponseContent::Text(
                        mcp_core::types::TextContent {
                            content_type: "text".to_string(),
                            text: format!("Invalid response type: {:?}", response),
                            annotations: None,
                        }
                    )]
                };
                mcp_core::types::CallToolResponse {
                    content,
                    is_error: None,
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
        }
    };

    let new_method = if context_arg {
        let ctx_ident = ctx_ident.as_ref().expect("Context identifier missing");
        quote! {
            pub fn new(#ctx_ident: #ctx_ty) -> Self {
                Self { context: #ctx_ident }
            }
        }
    } else {
        quote! {
            pub fn new() -> Self {
                Self { context: () }
            }
        }
    };

    let internal_call_inputs = if context_arg {
        let ctx_ident = ctx_ident.as_ref().unwrap();
        if !param_defs.is_empty() {
            quote! { #ctx_ident: #ctx_ty, #(#param_defs),* }
        } else {
            quote! { #ctx_ident: #ctx_ty }
        }
    } else {
        quote! { #(#param_defs),* }
    };

    let fn_body = &input_fn.block;

    let handler_fn = quote! {
        |req: mcp_core::types::CallToolRequest| -> std::pin::Pin<Box<dyn std::future::Future<Output = mcp_core::types::CallToolResponse> + Send>> {
            Box::pin(async move {
                let tool = #struct_name::new(#ctx_ty::default());
                #call_body
            })
        }
    };

    let output = quote! {
        #[derive(serde::Deserialize, schemars::JsonSchema)]
        struct #params_struct_name {
            #(#param_defs,)*
        }

        #vis pub struct #struct_name {
            context: #ctx_ty,
        }

        impl #struct_name {
            #new_method

            pub fn tool() -> mcp_core::types::Tool {
                mcp_core::types::Tool {
                    name: #tool_name.to_string(),
                    description: Some(#description.to_string()),
                    input_schema: #input_schema,
                    annotations: Some(mcp_core::types::ToolAnnotations {
                        title: Some(#title.to_string()),
                        read_only_hint: Some(#read_only_hint),
                        destructive_hint: Some(#destructive_hint),
                        idempotent_hint: Some(#idempotent_hint),
                        open_world_hint: Some(#open_world_hint),
                    }),
                }
            }

            pub fn handler() -> mcp_core::tools::ToolHandler {
                mcp_core::tools::ToolHandler {
                    tool: Self::tool(),
                    f: Box::new(#handler_fn),
                }
            }

            async fn internal_call(#internal_call_inputs) -> Result<#return_ty, #error_ty> {
                #fn_body
            }
        }
    };

    output.into()
}

#[derive(Debug)]
struct ToolParamArgs {
    ty: Type,
    hidden: bool,
    description: Option<String>,
}

impl syn::parse::Parse for ToolParamArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut hidden = false;
        let mut description = None;
        let ty = input.parse()?;

        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            let meta_list: Punctuated<Meta, Token![,]> = Punctuated::parse_terminated(input)?;

            for meta in meta_list {
                match meta {
                    Meta::Path(path) if path.is_ident("hidden") => {
                        hidden = true;
                    }
                    Meta::NameValue(nv) if nv.path.is_ident("description") => {
                        if let Expr::Lit(ExprLit {
                            lit: Lit::Str(lit_str),
                            ..
                        }) = &nv.value
                        {
                            description = Some(lit_str.value().to_string());
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(ToolParamArgs {
            ty,
            hidden,
            description,
        })
    }
}
