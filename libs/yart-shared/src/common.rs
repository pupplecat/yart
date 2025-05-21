use proc_macro2::TokenStream;
use quote::quote;
use serde_json::{Map, Value};
use syn::Type;

pub fn to_upper_camel_case(s: &str) -> String {
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

#[cfg(feature = "schema")]
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
