extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote,
    punctuated::Punctuated,
    Expr, ExprLit, FnArg, ItemFn, Lit, Meta, Pat, PatType, ReturnType, Token, Type,
};

use crate::common::to_upper_camel_case;

struct MacroArgs {
    description: String,
    name: Option<String>,
    context_arg: bool,
    read_only_hint: Option<bool>,
    destructive_hint: Option<bool>,
    idempotent_hint: Option<bool>,
    open_world_hint: Option<bool>,
}

impl Parse for MacroArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut description = None;
        let mut name = None;
        let mut context_arg = false;
        let mut read_only_hint = None;
        let mut destructive_hint = None;
        let mut idempotent_hint = None;
        let mut open_world_hint = None;

        if !input.is_empty() {
            let meta_list: Punctuated<Meta, Token![,]> = Punctuated::parse_terminated(input)?;
            for meta in meta_list {
                if let Meta::NameValue(nv) = meta {
                    let ident = nv.path.get_ident().unwrap().to_string();
                    match ident.as_str() {
                        "description" => {
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Str(lit_str),
                                ..
                            }) = nv.value
                            {
                                description = Some(lit_str.value());
                            }
                        }
                        "name" => {
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Str(lit_str),
                                ..
                            }) = nv.value
                            {
                                name = Some(lit_str.value());
                            }
                        }
                        "context_arg" => {
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Bool(lit_bool),
                                ..
                            }) = nv.value
                            {
                                context_arg = lit_bool.value();
                            }
                        }
                        "read_only_hint" => {
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Bool(lit_bool),
                                ..
                            }) = nv.value
                            {
                                read_only_hint = Some(lit_bool.value());
                            }
                        }
                        "destructive_hint" => {
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Bool(lit_bool),
                                ..
                            }) = nv.value
                            {
                                destructive_hint = Some(lit_bool.value());
                            }
                        }
                        "idempotent_hint" => {
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Bool(lit_bool),
                                ..
                            }) = nv.value
                            {
                                idempotent_hint = Some(lit_bool.value());
                            }
                        }
                        "open_world_hint" => {
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Bool(lit_bool),
                                ..
                            }) = nv.value
                            {
                                open_world_hint = Some(lit_bool.value());
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(MacroArgs {
            description: description.expect("rig_tool/mcp_tool requires a description attribute"),
            name,
            context_arg,
            read_only_hint,
            destructive_hint,
            idempotent_hint,
            open_world_hint,
        })
    }
}

pub fn parse_inputs(
    inputs: &Punctuated<FnArg, Token![,]>,
    context_arg: bool,
) -> (Option<Type>, Option<Type>, Option<Pat>, Option<Pat>) {
    match inputs.len() {
        0 => (None, None, None, None),
        1 => {
            let arg = inputs.first().unwrap();
            if let FnArg::Typed(pat_type) = arg {
                if context_arg {
                    (
                        Some(*pat_type.ty.clone()),
                        None,
                        Some(*pat_type.pat.clone()),
                        None,
                    )
                } else {
                    (
                        None,
                        Some(*pat_type.ty.clone()),
                        None,
                        Some(*pat_type.pat.clone()),
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
                    Some(*ctx_pat.ty.clone()),
                    Some(*args_pat.ty.clone()),
                    Some(*ctx_pat.pat.clone()),
                    Some(*args_pat.pat.clone()),
                )
            } else {
                panic!("Expected typed arguments");
            }
        }
        _ => panic!("rig_tool/mcp_tool expects 0-2 arguments (context and/or args)"),
    }
}

pub fn parse_return_type(item: &ItemFn) -> Type {
    match &item.sig.output {
        ReturnType::Type(_, ty) => {
            if let Type::Path(type_path) = &**ty {
                if let Some(result) = type_path.path.segments.last() {
                    if result.ident == "Result" {
                        if let syn::PathArguments::AngleBracketed(args) = &result.arguments {
                            if args.args.len() == 2 {
                                if let Some(syn::GenericArgument::Type(inner_ty)) =
                                    args.args.first()
                                {
                                    return inner_ty.clone();
                                }
                            }
                            panic!("Expected Result<T, E> with two type arguments");
                        }
                        panic!("Expected Result<T, E> with type arguments");
                    }
                    panic!("Expected Result return type");
                }
                panic!("Expected Result return type");
            }
            panic!("Expected Result return type");
        }
        _ => panic!("rig_tool/mcp_tool function must return Result"),
    }
}

// fn generate_input_schema(
//     ty: &Type,
//     required_params: Vec<String>,
//     param_descriptions: Vec<(String, String)>,
//     hidden_params: Vec<String>,
// ) -> TokenStream {
//     let description_pairs = param_descriptions.iter().map(|(name, desc)| {
//         quote! {
//             if name == #name {
//                 prop_obj.insert("description".to_string(), serde_json::Value::String(#desc.to_string()));
//             }
//         }
//     });

//     quote! {
//         // {
//             // let mut schema = yart::derive_parameters::<#ty>();
//             // if let serde_json::Value::Object(ref mut map) = schema {
//         //         // Add required fields
//         //         map.insert("required".to_string(), serde_json::Value::Array(
//         //             vec![#(
//         //                 serde_json::Value::String(#required_params.to_string())
//         //             ),*]
//         //         ));
//         //         map.remove("title");

//         //         // Normalize property types
//         //         if let Some(serde_json::Value::Object(props)) = map.get_mut("properties") {
//         //             for (name, prop) in props.iter_mut() {
//         //                 if let serde_json::Value::Object(prop_obj) = prop {
//         //                     // Fix number types
//         //                     if let Some(type_val) = prop_obj.get("type") {
//         //                         if type_val == "integer" || type_val == "number" || prop_obj.contains_key("format") {
//         //                             prop_obj.insert("type".to_string(), serde_json::Value::String("number".to_string()));
//         //                             prop_obj.remove("format");
//         //                             prop_obj.remove("minimum");
//         //                             prop_obj.remove("maximum");
//         //                         }
//         //                     }

//         //                     // Fix optional types
//         //                     if let Some(serde_json::Value::Array(types)) = prop_obj.get("type") {
//         //                         if types.len() == 2 && types.contains(&serde_json::Value::String("null".to_string())) {
//         //                             let mut main_type = types.iter()
//         //                                 .find(|&t| t != &serde_json::Value::String("null".to_string()))
//         //                                 .cloned()
//         //                                 .unwrap_or(serde_json::Value::String("string".to_string()));
//         //                             if main_type == serde_json::Value::String("integer".to_string()) {
//         //                                 main_type = serde_json::Value::String("number".to_string());
//         //                             }
//         //                             prop_obj.insert("type".to_string(), main_type);
//         //                         }
//         //                     }

//         //                     // Add descriptions
//         //                     #(#description_pairs)*
//         //                 }
//         //             }

//         //             // Remove hidden parameters
//         //             #(
//         //                 props.remove(#hidden_params);
//         //             )*
//         //         }
//             // }
//             // schema
//     }
// }

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
    let struct_name = syn::Ident::new(&to_upper_camel_case(&fn_name_str), fn_name.span());
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

    // let input_schema = generate_input_schema(
    //     &args_ty,
    //     required_params.iter().map(|s| s.to_string()).collect(),
    //     param_descriptions
    //         .iter()
    //         .map(|(n, d)| (n.to_string(), d.to_string()))
    //         .collect(),
    //     hidden_params.iter().map(|s| s.to_string()).collect(),
    // );

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
                    input_schema: yart::derive_parameters::<#args_ty>(),
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
