extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote,
    punctuated::Punctuated,
    Expr, ExprLit, FnArg, ItemFn, Lit, Meta, ReturnType, Token, Type,
};

use crate::common::to_upper_camel_case;

struct MacroArgs {
    description: String,
    name: Option<String>,
    context_arg: bool,
}

impl Parse for MacroArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut description = None;
        let mut name = None;
        let mut context_arg = false;

        if !input.is_empty() {
            let meta_list: Punctuated<Meta, Token![,]> = Punctuated::parse_terminated(input)?;
            for meta in meta_list {
                if let Meta::NameValue(nv) = meta {
                    let ident = nv.path.get_ident().unwrap().to_string();
                    if ident == "description" {
                        if let Expr::Lit(ExprLit {
                            lit: Lit::Str(lit_str),
                            ..
                        }) = nv.value
                        {
                            description = Some(lit_str.value());
                        }
                    } else if ident == "name" {
                        if let Expr::Lit(ExprLit {
                            lit: Lit::Str(lit_str),
                            ..
                        }) = nv.value
                        {
                            name = Some(lit_str.value());
                        }
                    } else if ident == "context_arg" {
                        if let Expr::Lit(ExprLit {
                            lit: Lit::Bool(lit_bool),
                            ..
                        }) = nv.value
                        {
                            context_arg = lit_bool.value();
                        }
                    }
                }
            }
        }

        Ok(MacroArgs {
            description: description.expect("rig_tool requires a description attribute"),
            name,
            context_arg,
        })
    }
}

pub fn rig_tool(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as MacroArgs);
    let item = parse_macro_input!(item as ItemFn);

    let description = args.description;
    let name = args.name;
    let context_arg = args.context_arg;

    let vis = &item.vis;
    let fn_name = &item.sig.ident;
    // Convert function name to UpperCamelCase for struct name
    let struct_name = syn::Ident::new(&to_upper_camel_case(&fn_name.to_string()), fn_name.span());
    // Use provided name or function name
    let tool_name = name.unwrap_or_else(|| format!("{}", fn_name));

    // Extract inputs (context and args)
    let inputs = &item.sig.inputs;
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
        _ => panic!("rig_tool expects 0-2 arguments (context and/or args)"),
    };

    let args_ty = args
        .as_ref()
        .map_or_else(|| parse_quote! { () }, |ty| *ty.clone());
    let ctx_ty = context
        .as_ref()
        .map_or_else(|| parse_quote! { () }, |ty| *ty.clone());

    // Debug: Ensure context is detected correctly
    assert!(
        (context.is_some() && ctx_ident.is_some()) || (context.is_none() && ctx_ident.is_none()),
        "Context and ctx_ident mismatch: context={:?}, ctx_ident={:?}",
        context.is_some(),
        ctx_ident.is_some()
    );

    // Extract return type
    let return_ty = match &item.sig.output {
        ReturnType::Type(_, ty) => {
            if let Type::Path(type_path) = &**ty {
                if let Some(result) = type_path.path.segments.last() {
                    if result.ident == "Result" {
                        if let syn::PathArguments::AngleBracketed(args) = &result.arguments {
                            if let Some(syn::GenericArgument::Type(inner_ty)) = args.args.first() {
                                inner_ty.clone()
                            } else {
                                panic!("Expected Result<T, E> with type argument");
                            }
                        } else {
                            panic!("Expected Result<T, E> with type arguments");
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

    // Error type
    let error_ty: Type = parse_quote! { yart::ToolError };

    // Generate internal_call with original parameter names
    let internal_call_inputs = match (context.is_some(), args.is_some()) {
        (true, true) => {
            let ctx_ident = ctx_ident.as_ref().unwrap();
            let args_ident = args_ident.as_ref().unwrap();
            quote! { #ctx_ident: #ctx_ty, #args_ident: #args_ty }
        }
        (true, false) => {
            let ctx_ident = ctx_ident.as_ref().unwrap();
            quote! { #ctx_ident: #ctx_ty }
        }
        (false, true) => {
            let args_ident = args_ident.as_ref().unwrap();
            quote! { #args_ident: #args_ty }
        }
        (false, false) => quote! {},
    };

    let fn_body = &item.block;

    // Generate call method
    let call_body = match (context.is_some(), args.is_some()) {
        (true, true) => {
            let ctx_ident = ctx_ident.as_ref().unwrap();
            quote! {
                let #ctx_ident = self.context.clone();
                let result = yart::wrap_unsafe(move || async move {
                    #struct_name::internal_call(#ctx_ident, args)
                        .await
                        .map_err(|e| anyhow::anyhow!(e.to_string()))
                })
                .await?;
                let serialized_result = serde_json::to_value(result)
                    .map_err(|e| yart::ToolError(format!("Serialization error: {}", e)))?;
                Ok(yart::ToolOutput {
                    result: serialized_result,
                })
            }
        }
        (true, false) => {
            let ctx_ident = ctx_ident.as_ref().unwrap();
            quote! {
                let _args = args; // Ignore unused args (())
                let #ctx_ident = self.context.clone();
                let result = yart::wrap_unsafe(move || async move {
                    #struct_name::internal_call(#ctx_ident)
                        .await
                        .map_err(|e| anyhow::anyhow!(e.to_string()))
                })
                .await?;
                let serialized_result = serde_json::to_value(result)
                    .map_err(|e| yart::ToolError(format!("Serialization error: {}", e)))?;
                Ok(yart::ToolOutput {
                    result: serialized_result,
                })
            }
        }
        (false, true) => {
            // let args_ident = args_ident.as_ref().unwrap();
            quote! {
                let result = yart::wrap_unsafe(move || async move {
                    #struct_name::internal_call(args)
                        .await
                        .map_err(|e| anyhow::anyhow!(e.to_string()))
                })
                .await?;
                let serialized_result = serde_json::to_value(result)
                    .map_err(|e| yart::ToolError(format!("Serialization error: {}", e)))?;
                Ok(yart::ToolOutput {
                    result: serialized_result,
                })
            }
        }
        (false, false) => quote! {
            let _args = args; // Ignore unused args (())
            let result = yart::wrap_unsafe(move || async move {
                #struct_name::internal_call()
                    .await
                    .map_err(|e| anyhow::anyhow!(e.to_string()))
            })
            .await?;
            let serialized_result = serde_json::to_value(result)
                .map_err(|e| yart::ToolError(format!("Serialization error: {}", e)))?;
            Ok(yart::ToolOutput {
                result: serialized_result,
            })
        },
    };

    let call_method = quote! {
        async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
            #call_body
        }
    };

    // Generate new method
    let new_method = if context.is_some() {
        let ctx_ident = ctx_ident
            .as_ref()
            .expect("Context identifier missing when context is present");
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

    // Generate struct and impls
    let output = quote! {
        #vis pub struct #struct_name {
            context: #ctx_ty,
        }

        impl #struct_name {
            #new_method

            async fn internal_call(#internal_call_inputs) -> Result<#return_ty, #error_ty> {
                #fn_body
            }
        }

        impl rig::tool::Tool for #struct_name {
            const NAME: &'static str = #tool_name;

            type Error = yart::ToolError;
            type Args = #args_ty;
            type Output = yart::ToolOutput;

            fn name(&self) -> String {
                Self::NAME.to_string()
            }

            async fn definition(&self, _prompt: String) -> rig::completion::ToolDefinition {
                rig::completion::ToolDefinition {
                    name: Self::NAME.to_string(),
                    description: #description.to_string(),
                    parameters: yart::derive_parameters::<#args_ty>(),
                }
            }

            #call_method
        }
    };

    output.into()
}
