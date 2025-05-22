use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    parse_quote,
    punctuated::Punctuated,
    Expr, ExprLit, FnArg, Item, ItemFn, Lit, Meta, Pat, PatType, ReturnType, Token, Type,
};

#[derive(Debug)]
struct ToolArgs {
    name: Option<String>,
    description: Option<String>,
    context_arg: bool,
    annotations: ToolAnnotations,
}

#[derive(Debug)]
struct ToolAnnotations {
    title: Option<String>,
    read_only_hint: Option<bool>,
    destructive_hint: Option<bool>,
    idempotent_hint: Option<bool>,
    open_world_hint: Option<bool>,
}

impl Default for ToolAnnotations {
    fn default() -> Self {
        Self {
            title: None,
            read_only_hint: None,
            destructive_hint: None,
            idempotent_hint: None,
            open_world_hint: None,
        }
    }
}

impl Parse for ToolArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut name = None;
        let mut description = None;
        let mut context_arg = false;
        let mut annotations = ToolAnnotations::default();

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
                                annotations.read_only_hint = Some(lit_bool.value);
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
                                annotations.destructive_hint = Some(lit_bool.value);
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
                                annotations.idempotent_hint = Some(lit_bool.value);
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
                                annotations.open_world_hint = Some(lit_bool.value);
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
                                    annotations.title = Some(lit_str.value());
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
                                        annotations.read_only_hint = Some(lit_bool.value)
                                    }
                                    "destructive_hint" | "destructiveHint" => {
                                        annotations.destructive_hint = Some(lit_bool.value)
                                    }
                                    "idempotent_hint" | "idempotentHint" => {
                                        annotations.idempotent_hint = Some(lit_bool.value)
                                    }
                                    "open_world_hint" | "openWorldHint" => {
                                        annotations.open_world_hint = Some(lit_bool.value)
                                    }
                                    _ => {
                                        return Err(syn::Error::new_spanned(
                                            nv.path,
                                            format!("Unknown boolean annotation: {}", key),
                                        ));
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
                    ));
                }
            }
        }

        Ok(ToolArgs {
            name,
            description,
            context_arg,
            annotations,
        })
    }
}

fn extract_input(
    input_fn: &ItemFn,
    context_arg: bool,
) -> (
    Option<Box<Type>>,
    Option<Box<Type>>,
    Option<Box<Pat>>,
    Option<Box<Pat>>,
) {
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

    // Debug: Ensure context is detected correctly
    assert!(
        (context.is_some() && ctx_ident.is_some()) || (context.is_none() && ctx_ident.is_none()),
        "Context and ctx_ident mismatch: context={:?}, ctx_ident={:?}",
        context.is_some(),
        ctx_ident.is_some()
    );

    (context, args, ctx_ident, args_ident)
}

fn extract_return_type(input_fn: &ItemFn) -> Type {
    // Extract return type
    let return_ty = match &input_fn.sig.output {
        ReturnType::Type(_, ty) => {
            if let Type::Path(type_path) = &**ty {
                if let Some(result) = type_path.path.segments.last() {
                    if result.ident == "Result" {
                        if let syn::PathArguments::AngleBracketed(args) = &result.arguments {
                            if args.args.len() >= 1 {
                                if let Some(syn::GenericArgument::Type(inner_ty)) =
                                    args.args.first()
                                {
                                    inner_ty.clone()
                                } else {
                                    panic!("Expected Result<T> with type argument");
                                }
                            } else {
                                panic!("Expected Result<T> with type argument");
                            }
                        } else {
                            panic!("Expected Result<T> with type argument");
                        }
                    } else {
                        panic!("Expected Result return type");
                    }
                } else {
                    panic!("Expected Result return type");
                }
            } else {
                panic!("Expected Result return type");
            }
        }
        _ => panic!("rig_tool function must return Result"),
    };

    return_ty
}

pub fn mcp_tool(args: TokenStream, input_fn: TokenStream) -> TokenStream {
    let args = match syn::parse2::<ToolArgs>(args) {
        Ok(args) => args,
        Err(e) => return e.to_compile_error().into(),
    };

    let input_fn = match syn::parse2::<ItemFn>(input_fn.clone()) {
        Ok(input_fn) => input_fn,
        Err(e) => return e.to_compile_error().into(),
    };

    let fn_name = &input_fn.sig.ident;
    let fn_name_str = fn_name.to_string();
    let struct_name = format_ident!("{}Mcp", fn_name_str.to_case(Case::Pascal));
    let tool_name = args.name.unwrap_or(fn_name_str.clone());
    let tool_description = args.description.unwrap_or_default();
    let context_arg = args.context_arg;

    // Tool annotations
    let title = args.annotations.title.unwrap_or(fn_name_str.clone());
    let read_only_hint = args.annotations.read_only_hint.unwrap_or(false);
    let destructive_hint = args.annotations.destructive_hint.unwrap_or(true);
    let idempotent_hint = args.annotations.idempotent_hint.unwrap_or(false);
    let open_world_hint = args.annotations.open_world_hint.unwrap_or(true);

    let (context, args, ctx_ident, args_ident) = extract_input(&input_fn, context_arg);

    let args_ty = args
        .as_ref()
        .map_or_else(|| parse_quote! { () }, |ty| *ty.clone());
    let ctx_ty = context
        .as_ref()
        .map_or_else(|| parse_quote! { () }, |ty| *ty.clone());

    let return_type = extract_return_type(&input_fn);

    let call_signature = match context.is_some() {
        true => quote! {#ctx_ident: #ctx_ty},
        false => quote! {},
    };

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
                   move |req: mcp_core::types::CallToolRequest| {
                   }
            }
        }
    };

    expanded.into()

    // let debug_msg = format!(
    //     "Context and ctx_ident mismatch: context={:?}, ctx_ident={:?}",
    //     ctx_ty.to_token_stream().to_string(),
    //     args_ty.to_token_stream().to_string(),
    // );

    // quote! {
    //     compile_error!(#debug_msg)
    // }
    // .into()
}

#[cfg(test)]
mod test_mcp_macro {
    use anyhow::Result;
    use proc_macro2::TokenStream;
    use quote::ToTokens;
    use syn::{parse_quote, Attribute};

    use crate::mcp_macro::{extract_input, extract_return_type};

    use super::{mcp_tool, ToolArgs};

    fn get_inner_token_stream(attr: Attribute) -> Result<TokenStream> {
        let args_ts = match attr.meta {
            syn::Meta::List(syn::MetaList { tokens, .. }) => Ok(tokens),
            _ => Err(syn::Error::new_spanned(attr.meta, "Expected mcp_tool(...)")),
        }
        .map_err(|e| anyhow::anyhow!("{}", e))?;

        Ok(args_ts)
    }

    #[test]
    fn gen_code() -> Result<()> {
        // Parse attribute and extract inner mcp_tool(...) TokenStream
        let attr: syn::Attribute =
            parse_quote! { #[mcp_tool(description = "TEST_XX", context_arg = true)] };
        let args_ts = match attr.meta {
            syn::Meta::List(syn::MetaList { tokens, .. }) => Ok(tokens),
            _ => Err(syn::Error::new_spanned(attr.meta, "Expected mcp_tool(...)")),
        }
        .map_err(|e| anyhow::anyhow!("{}", e))?;

        let input_fn: syn::ItemFn =
            parse_quote! { async fn test_tool(args: MyArgs) -> Result<()> { Ok(()) } };

        // Convert proc_macro2::TokenStream to proc_macro::TokenStream
        let ret = mcp_tool(args_ts.to_token_stream(), input_fn.to_token_stream());

        println!("{}", ret);

        Ok(())
    }

    #[test]
    fn argument_description() -> Result<()> {
        let args = get_inner_token_stream(parse_quote! { #[mcp_tool(description = "TEST_XX")] })?;

        let args = match syn::parse2::<ToolArgs>(args.into()) {
            Ok(args) => args,
            Err(e) => return Err(anyhow::anyhow!("parse error: {}", e)),
        };

        assert_eq!(args.name, None);
        assert_eq!(args.description, Some("TEST_XX".to_string()));
        assert_eq!(args.context_arg, false);
        assert_eq!(args.annotations.title, None);
        assert_eq!(args.annotations.read_only_hint, None);
        assert_eq!(args.annotations.destructive_hint, None);
        assert_eq!(args.annotations.idempotent_hint, None);
        assert_eq!(args.annotations.open_world_hint, None);

        Ok(())
    }

    #[test]
    fn argument_name() -> Result<()> {
        let args = get_inner_token_stream(parse_quote! { #[mcp_tool(name = "TEST_XX")] })?;

        let args = match syn::parse2::<ToolArgs>(args.into()) {
            Ok(args) => args,
            Err(e) => return Err(anyhow::anyhow!("parse error: {}", e)),
        };

        assert_eq!(args.name, Some("TEST_XX".to_string()));
        assert_eq!(args.description, None);
        assert_eq!(args.context_arg, false);
        assert_eq!(args.annotations.title, None);
        assert_eq!(args.annotations.read_only_hint, None);
        assert_eq!(args.annotations.destructive_hint, None);
        assert_eq!(args.annotations.idempotent_hint, None);
        assert_eq!(args.annotations.open_world_hint, None);

        Ok(())
    }

    #[test]
    fn argument_context_arg() -> Result<()> {
        let args = get_inner_token_stream(parse_quote! { #[mcp_tool(context_arg = true)] })?;

        let args = match syn::parse2::<ToolArgs>(args.into()) {
            Ok(args) => args,
            Err(e) => return Err(anyhow::anyhow!("parse error: {}", e)),
        };

        assert_eq!(args.name, None);
        assert_eq!(args.description, None);
        assert_eq!(args.context_arg, true);
        assert_eq!(args.annotations.title, None);
        assert_eq!(args.annotations.read_only_hint, None);
        assert_eq!(args.annotations.destructive_hint, None);
        assert_eq!(args.annotations.idempotent_hint, None);
        assert_eq!(args.annotations.open_world_hint, None);

        Ok(())
    }

    #[test]
    fn argument_read_only_hint() -> Result<()> {
        let args = get_inner_token_stream(parse_quote! { #[mcp_tool(read_only_hint = true)] })?;

        let args = match syn::parse2::<ToolArgs>(args.into()) {
            Ok(args) => args,
            Err(e) => return Err(anyhow::anyhow!("parse error: {}", e)),
        };

        assert_eq!(args.name, None);
        assert_eq!(args.description, None);
        assert_eq!(args.context_arg, false);
        assert_eq!(args.annotations.title, None);
        assert_eq!(args.annotations.read_only_hint, Some(true));
        assert_eq!(args.annotations.destructive_hint, None);
        assert_eq!(args.annotations.idempotent_hint, None);
        assert_eq!(args.annotations.open_world_hint, None);

        Ok(())
    }

    #[test]
    fn argument_destructive_hint() -> Result<()> {
        let args = get_inner_token_stream(parse_quote! { #[mcp_tool(destructive_hint = true)] })?;

        let args = match syn::parse2::<ToolArgs>(args.into()) {
            Ok(args) => args,
            Err(e) => return Err(anyhow::anyhow!("parse error: {}", e)),
        };

        assert_eq!(args.name, None);
        assert_eq!(args.description, None);
        assert_eq!(args.context_arg, false);
        assert_eq!(args.annotations.title, None);
        assert_eq!(args.annotations.read_only_hint, None);
        assert_eq!(args.annotations.destructive_hint, Some(true));
        assert_eq!(args.annotations.idempotent_hint, None);
        assert_eq!(args.annotations.open_world_hint, None);

        Ok(())
    }

    #[test]
    fn argument_idempotent_hint() -> Result<()> {
        let args = get_inner_token_stream(parse_quote! { #[mcp_tool(idempotent_hint = true)] })?;

        let args = match syn::parse2::<ToolArgs>(args.into()) {
            Ok(args) => args,
            Err(e) => return Err(anyhow::anyhow!("parse error: {}", e)),
        };

        assert_eq!(args.name, None);
        assert_eq!(args.description, None);
        assert_eq!(args.context_arg, false);
        assert_eq!(args.annotations.title, None);
        assert_eq!(args.annotations.read_only_hint, None);
        assert_eq!(args.annotations.destructive_hint, None);
        assert_eq!(args.annotations.idempotent_hint, Some(true));
        assert_eq!(args.annotations.open_world_hint, None);

        Ok(())
    }

    #[test]
    fn argument_open_world_hint() -> Result<()> {
        let args = get_inner_token_stream(parse_quote! { #[mcp_tool(open_world_hint = true)] })?;

        let args = match syn::parse2::<ToolArgs>(args.into()) {
            Ok(args) => args,
            Err(e) => return Err(anyhow::anyhow!("parse error: {}", e)),
        };

        assert_eq!(args.name, None);
        assert_eq!(args.description, None);
        assert_eq!(args.context_arg, false);
        assert_eq!(args.annotations.title, None);
        assert_eq!(args.annotations.read_only_hint, None);
        assert_eq!(args.annotations.destructive_hint, None);
        assert_eq!(args.annotations.idempotent_hint, None);
        assert_eq!(args.annotations.open_world_hint, Some(true));

        Ok(())
    }

    #[test]
    fn argument_annotations_title() -> Result<()> {
        let args =
            get_inner_token_stream(parse_quote! { #[mcp_tool(annotations(title = "_title"))] })?;

        let args = match syn::parse2::<ToolArgs>(args.into()) {
            Ok(args) => args,
            Err(e) => return Err(anyhow::anyhow!("parse error: {}", e)),
        };

        assert_eq!(args.name, None);
        assert_eq!(args.description, None);
        assert_eq!(args.context_arg, false);
        assert_eq!(args.annotations.title, Some("_title".to_string()));
        assert_eq!(args.annotations.read_only_hint, None);
        assert_eq!(args.annotations.destructive_hint, None);
        assert_eq!(args.annotations.idempotent_hint, None);
        assert_eq!(args.annotations.open_world_hint, None);

        Ok(())
    }

    #[test]
    fn argument_annotations() -> Result<()> {
        let args = get_inner_token_stream(
            parse_quote! { #[mcp_tool(annotations(title = "_title", read_only_hint=true,  destructive_hint=true,  idempotent_hint=true,  open_world_hint=true, ))] },
        )?;

        let args = match syn::parse2::<ToolArgs>(args.into()) {
            Ok(args) => args,
            Err(e) => return Err(anyhow::anyhow!("parse error: {}", e)),
        };

        assert_eq!(args.name, None);
        assert_eq!(args.description, None);
        assert_eq!(args.context_arg, false);
        assert_eq!(args.annotations.title, Some("_title".to_string()));
        assert_eq!(args.annotations.read_only_hint, Some(true));
        assert_eq!(args.annotations.destructive_hint, Some(true));
        assert_eq!(args.annotations.idempotent_hint, Some(true));
        assert_eq!(args.annotations.open_world_hint, Some(true));

        Ok(())
    }

    #[test]
    fn extract_input_context_and_args() -> Result<()> {
        let input_fn: syn::ItemFn = parse_quote! { async fn test_tool(ctx:Arc<String>,args: MyArgs) -> Result<()> { Ok(()) } };

        // Convert proc_macro2::TokenStream to proc_macro::TokenStream
        let (context, args, ctx_ident, args_ident) = extract_input(&input_fn, false);

        assert_eq!(context.is_some(), true);
        assert_eq!(
            *(context.unwrap()).into_token_stream().to_string(),
            "Arc < String >".to_string()
        );
        assert_eq!(args.is_some(), true);
        assert_eq!(
            *(args.unwrap()).into_token_stream().to_string(),
            "MyArgs".to_string()
        );
        assert_eq!(ctx_ident.is_some(), true);
        assert_eq!(
            *(ctx_ident.unwrap()).into_token_stream().to_string(),
            "ctx".to_string()
        );
        assert_eq!(args_ident.is_some(), true);
        assert_eq!(
            *(args_ident.unwrap()).into_token_stream().to_string(),
            "args".to_string()
        );

        Ok(())
    }

    #[test]
    fn extract_input_context() -> Result<()> {
        let input_fn: syn::ItemFn =
            parse_quote! { async fn test_tool(ctx:Arc<String>) -> Result<()> { Ok(()) } };

        // Convert proc_macro2::TokenStream to proc_macro::TokenStream
        let (context, args, ctx_ident, args_ident) = extract_input(&input_fn, true);

        assert_eq!(context.is_some(), true);
        assert_eq!(
            *(context.unwrap()).into_token_stream().to_string(),
            "Arc < String >".to_string()
        );
        assert_eq!(args.is_some(), false);
        assert_eq!(ctx_ident.is_some(), true);
        assert_eq!(
            *(ctx_ident.unwrap()).into_token_stream().to_string(),
            "ctx".to_string()
        );
        assert_eq!(args_ident.is_some(), false);

        Ok(())
    }

    #[test]
    fn extract_input_args() -> Result<()> {
        let input_fn: syn::ItemFn =
            parse_quote! { async fn test_tool(args: MyArgs) -> Result<()> { Ok(()) } };

        // Convert proc_macro2::TokenStream to proc_macro::TokenStream
        let (context, args, ctx_ident, args_ident) = extract_input(&input_fn, false);

        assert_eq!(context.is_some(), false);
        assert_eq!(args.is_some(), true);
        assert_eq!(
            *(args.unwrap()).into_token_stream().to_string(),
            "MyArgs".to_string()
        );
        assert_eq!(ctx_ident.is_some(), false);
        assert_eq!(args_ident.is_some(), true);
        assert_eq!(
            *(args_ident.unwrap()).into_token_stream().to_string(),
            "args".to_string()
        );

        Ok(())
    }

    #[test]
    fn extract_input_no_input() -> Result<()> {
        let input_fn: syn::ItemFn = parse_quote! { async fn test_tool() -> Result<()> { Ok(()) } };

        // Convert proc_macro2::TokenStream to proc_macro::TokenStream
        let (context, args, ctx_ident, args_ident) = extract_input(&input_fn, false);

        assert_eq!(context.is_some(), false);
        assert_eq!(args.is_some(), false);
        assert_eq!(ctx_ident.is_some(), false);
        assert_eq!(args_ident.is_some(), false);

        Ok(())
    }

    #[test]
    fn extract_return_type_blanket() -> Result<()> {
        let input_fn: syn::ItemFn = parse_quote! { async fn test_tool() -> Result<()> { Ok(()) } };

        // Convert proc_macro2::TokenStream to proc_macro::TokenStream
        let return_type = extract_return_type(&input_fn);

        assert_eq!(
            return_type.into_token_stream().to_string(),
            "()".to_string()
        );

        Ok(())
    }

    #[test]
    fn extract_return_type_named() -> Result<()> {
        let input_fn: syn::ItemFn =
            parse_quote! { async fn test_tool() -> Result<Response> { Ok(()) } };

        // Convert proc_macro2::TokenStream to proc_macro::TokenStream
        let return_type = extract_return_type(&input_fn);

        assert_eq!(
            return_type.into_token_stream().to_string(),
            "Response".to_string()
        );

        Ok(())
    }
}
