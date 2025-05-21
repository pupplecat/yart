use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{Expr, ExprLit, FnArg, ItemFn, Lit, Meta, Pat, ReturnType, Token, Type};

pub struct MacroArgs {
    pub description: String,
    pub name: Option<String>,
    pub context_arg: bool,
    pub read_only_hint: Option<bool>,
    pub destructive_hint: Option<bool>,
    pub idempotent_hint: Option<bool>,
    pub open_world_hint: Option<bool>,
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

pub fn generate_input_schema(
    ty: &Type,
    required_params: Vec<String>,
    param_descriptions: Vec<(String, String)>,
    hidden_params: Vec<String>,
) -> TokenStream {
    quote! {
        {
            let schema = schemars::schema_for!(#ty);
            let mut schema = serde_json::to_value(schema.schema).expect("Failed to serialize schema");
            if let serde_json::Value::Object(ref mut map) = schema {
                // Add required fields
                map.insert("required".to_string(), serde_json::Value::Array(
                    vec![#(
                        serde_json::Value::String(#required_params.to_string())
                    ),*]
                ));
                map.remove("title");

                // Normalize property types
                if let Some(serde_json::Value::Object(props)) = map.get_mut("properties") {
                    for (name, prop) in props.iter_mut() {
                        if let serde_json::Value::Object(prop_obj) = prop {
                            // Fix number types
                            if let Some(type_val) = prop_obj.get("type") {
                                if type_val == "integer" || type_val == "number" || prop_obj.contains_key("format") {
                                    prop_obj.insert("type".to_string(), serde_json::Value::String("number".to_string()));
                                    prop_obj.remove("format");
                                    prop_obj.remove("minimum");
                                    prop_obj.remove("maximum");
                                }
                            }

                            // Fix optional types
                            if let Some(serde_json::Value::Array(types)) = prop_obj.get("type") {
                                if types.len() == 2 && types.contains(&serde_json::Value::String("null".to_string())) {
                                    let mut main_type = types.iter()
                                        .find(|&t| t != &serde_json::Value::String("null".to_string()))
                                        .cloned()
                                        .unwrap_or(serde_json::Value::String("string".to_string()));
                                    if main_type == serde_json::Value::String("integer".to_string()) {
                                        main_type = serde_json::Value::String("number".to_string());
                                    }
                                    prop_obj.insert("type".to_string(), main_type);
                                }
                            }

                            // Add descriptions
                            #(
                                if name == #param_descriptions.0 {
                                    prop_obj.insert("description".to_string(), serde_json::Value::String(#param_descriptions.1.to_string()));
                                }
                            )*
                        }
                    }

                    // Remove hidden parameters
                    #(
                        props.remove(#hidden_params);
                    )*
                }
            }
            schema
        }
    }
}
