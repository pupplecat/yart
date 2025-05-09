extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{
    parse_macro_input, parse_quote, Expr, ExprLit, FnArg, ItemFn, Lit, Meta, ReturnType, Token,
    Type,
};

// Convert snake_case to UpperCamelCase (e.g., find_token_metadata -> FindTokenMetadata)
fn to_upper_camel_case(s: &str) -> String {
    s.split('_')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().to_string() + chars.as_str(),
            }
        })
        .collect()
}

struct MacroArgs {
    description: String,
    name: Option<String>,
}

impl Parse for MacroArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut description = None;
        let mut name = None;

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
                    }
                }
            }
        }

        Ok(MacroArgs {
            description: description.expect("rig_tool requires a description attribute"),
            name,
        })
    }
}

#[proc_macro_attribute]
pub fn rig_tool(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as MacroArgs);
    let item = parse_macro_input!(item as ItemFn);

    let description = args.description;
    let name = args.name;

    let vis = &item.vis;
    let fn_name = &item.sig.ident;
    // Convert function name to UpperCamelCase for struct name
    let struct_name = syn::Ident::new(&to_upper_camel_case(&fn_name.to_string()), fn_name.span());
    // Use provided name or function name
    let tool_name = name.unwrap_or_else(|| format!("{}", fn_name));

    // Extract inputs (context and args)
    let inputs = &item.sig.inputs;
    let (context, args) = match inputs.len() {
        0 => (None, None),
        1 => {
            let arg = inputs.first().unwrap();
            if let FnArg::Typed(pat_type) = arg {
                // Assume single argument is args (no context)
                (None, Some(pat_type.ty.clone()))
            } else {
                panic!("Expected typed argument");
            }
        }
        2 => {
            let mut iter = inputs.iter();
            let ctx_arg = iter.next().unwrap();
            let args_arg = iter.next().unwrap();
            if let (FnArg::Typed(ctx_pat), FnArg::Typed(args_pat)) = (ctx_arg, args_arg) {
                (Some(ctx_pat.ty.clone()), Some(args_pat.ty.clone()))
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

    // Generate internal_call
    let internal_call_inputs = if context.is_some() && args.is_some() {
        quote! { ctx: #ctx_ty, args: #args_ty }
    } else if context.is_some() {
        quote! { ctx: #ctx_ty }
    } else if args.is_some() {
        quote! { args: #args_ty }
    } else {
        quote! {}
    };

    let fn_body = &item.block;

    // Generate call method
    let call_body = if context.is_some() && args.is_some() {
        quote! {
            let ctx = self.ctx.clone();
            let result = yart::wrap_unsafe(move || async move {
                #struct_name::internal_call(ctx, args)
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
    } else if context.is_some() {
        quote! {
            let ctx = self.ctx.clone();
            let result = yart::wrap_unsafe(move || async move {
                #struct_name::internal_call(ctx)
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
    } else if args.is_some() {
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
    } else {
        quote! {
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
        }
    };

    // Generate new method conditionally
    let new_method = if context.is_some() {
        quote! {
            pub fn new(ctx: #ctx_ty) -> Self {
                Self { ctx }
            }
        }
    } else {
        quote! {
            pub fn new() -> Self {
                Self { ctx: () }
            }
        }
    };

    // Generate struct and impls
    let output = quote! {
        #vis pub struct #struct_name {
            ctx: #ctx_ty,
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

            async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
                #call_body
            }
        }
    };

    output.into()
}
