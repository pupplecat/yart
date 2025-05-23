#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::sync::Arc;
use anyhow::Result;
use mcp_core::{
    server::Server,
    types::{ServerCapabilities, TextContent, ToolCapabilities, ToolResponseContent},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
/// Context structure for tool execution, holding shared state.
pub struct ToolContext {
    value: String,
}
#[automatically_derived]
impl ::core::clone::Clone for ToolContext {
    #[inline]
    fn clone(&self) -> ToolContext {
        ToolContext {
            value: ::core::clone::Clone::clone(&self.value),
        }
    }
}
/// Input arguments for the tool, deserialized from the request.
pub struct ToolInput {
    value: String,
}
#[automatically_derived]
impl ::core::default::Default for ToolInput {
    #[inline]
    fn default() -> ToolInput {
        ToolInput {
            value: ::core::default::Default::default(),
        }
    }
}
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for ToolInput {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "value" => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"value" => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<ToolInput>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = ToolInput;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct ToolInput",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        String,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct ToolInput with 1 element",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(ToolInput { value: __field0 })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("value"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("value")?
                        }
                    };
                    _serde::__private::Ok(ToolInput { value: __field0 })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["value"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "ToolInput",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<ToolInput>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for ToolInput {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "ToolInput",
                false as usize + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "value",
                &self.value,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
const _: () = {
    #[automatically_derived]
    #[allow(unused_braces)]
    impl schemars::JsonSchema for ToolInput {
        fn schema_name() -> std::string::String {
            "ToolInput".to_owned()
        }
        fn schema_id() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("mix_x::ToolInput")
        }
        fn json_schema(
            generator: &mut schemars::gen::SchemaGenerator,
        ) -> schemars::schema::Schema {
            schemars::_private::metadata::add_description(
                {
                    let mut schema_object = schemars::schema::SchemaObject {
                        instance_type: Some(
                            schemars::schema::InstanceType::Object.into(),
                        ),
                        ..Default::default()
                    };
                    let object_validation = schema_object.object();
                    {
                        schemars::_private::insert_object_property::<
                            String,
                        >(
                            object_validation,
                            "value",
                            false,
                            false,
                            generator.subschema_for::<String>(),
                        );
                    }
                    schemars::schema::Schema::Object(schema_object)
                },
                "Input arguments for the tool, deserialized from the request.",
            )
        }
    }
};
/// Output response from the tool, convertible to ToolResponseContent.
pub struct ToolOutput {
    result: String,
}
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for ToolOutput {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "result" => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"result" => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<ToolOutput>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = ToolOutput;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct ToolOutput",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        String,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct ToolOutput with 1 element",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(ToolOutput { result: __field0 })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("result"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("result")?
                        }
                    };
                    _serde::__private::Ok(ToolOutput { result: __field0 })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["result"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "ToolOutput",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<ToolOutput>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for ToolOutput {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "ToolOutput",
                false as usize + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "result",
                &self.result,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
const _: () = {
    #[automatically_derived]
    #[allow(unused_braces)]
    impl schemars::JsonSchema for ToolOutput {
        fn schema_name() -> std::string::String {
            "ToolOutput".to_owned()
        }
        fn schema_id() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("mix_x::ToolOutput")
        }
        fn json_schema(
            generator: &mut schemars::gen::SchemaGenerator,
        ) -> schemars::schema::Schema {
            schemars::_private::metadata::add_description(
                {
                    let mut schema_object = schemars::schema::SchemaObject {
                        instance_type: Some(
                            schemars::schema::InstanceType::Object.into(),
                        ),
                        ..Default::default()
                    };
                    let object_validation = schema_object.object();
                    {
                        schemars::_private::insert_object_property::<
                            String,
                        >(
                            object_validation,
                            "result",
                            false,
                            false,
                            generator.subschema_for::<String>(),
                        );
                    }
                    schemars::schema::Schema::Object(schema_object)
                },
                "Output response from the tool, convertible to ToolResponseContent.",
            )
        }
    }
};
impl Into<Vec<ToolResponseContent>> for ToolOutput {
    fn into(self) -> Vec<ToolResponseContent> {
        <[_]>::into_vec(
            ::alloc::boxed::box_new([
                ToolResponseContent::Text(TextContent {
                    content_type: "text".to_string(),
                    text: self.result,
                    annotations: None,
                }),
            ]),
        )
    }
}
pub struct ExecuteSearch {
    context: Arc<ToolContext>,
}
impl ExecuteSearch {
    pub fn new(ctx: Arc<ToolContext>) -> Self {
        Self { context: ctx }
    }
    async fn internal_call(
        ctx: Arc<ToolContext>,
        args: ToolInput,
    ) -> Result<ToolOutput, yart::ToolError> {
        {
            Ok(ToolOutput {
                result: ::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!("{0}, {1}", ctx.value, args.value),
                    );
                    res
                }),
            })
        }
    }
}
impl rig::tool::Tool for ExecuteSearch {
    const NAME: &'static str = "execute_search";
    type Error = yart::ToolError;
    type Args = ToolInput;
    type Output = yart::ToolOutput;
    fn name(&self) -> String {
        Self::NAME.to_string()
    }
    async fn definition(&self, _prompt: String) -> rig::completion::ToolDefinition {
        rig::completion::ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Brand new tool".to_string(),
            parameters: yart::derive_parameters::<ToolInput>(),
        }
    }
    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let ctx = self.context.clone();
        let result = yart::wrap_unsafe(move || async move {
                ExecuteSearch::internal_call(ctx, args)
                    .await
                    .map_err(|e| ::anyhow::__private::must_use({
                        use ::anyhow::__private::kind::*;
                        let error = match e.to_string() {
                            error => (&error).anyhow_kind().new(error),
                        };
                        error
                    }))
            })
            .await?;
        let serialized_result = serde_json::to_value(result)
            .map_err(|e| yart::ToolError(
                ::alloc::__export::must_use({
                    let res = ::alloc::fmt::format(
                        format_args!("Serialization error: {0}", e),
                    );
                    res
                }),
            ))?;
        Ok(yart::ToolOutput {
            result: serialized_result,
        })
    }
}
pub struct ExecuteSearchMcp;
#[automatically_derived]
impl ::core::default::Default for ExecuteSearchMcp {
    #[inline]
    fn default() -> ExecuteSearchMcp {
        ExecuteSearchMcp {}
    }
}
impl ExecuteSearchMcp {
    pub fn tool() -> mcp_core::types::Tool {
        let schema = serde_json::to_value(
                ::schemars::gen::SchemaGenerator::default()
                    .into_root_schema_for::<ToolInput>(),
            )
            .expect("Failed to serialize schema");
        let annotations = ::serde_json::Value::Object({
            let mut object = ::serde_json::Map::new();
            let _ = object
                .insert(
                    ("title").into(),
                    ::serde_json::to_value(&"execute_search").unwrap(),
                );
            let _ = object
                .insert(("readOnlyHint").into(), ::serde_json::Value::Bool(false));
            let _ = object
                .insert(("destructiveHint").into(), ::serde_json::Value::Bool(true));
            let _ = object
                .insert(("idempotentHint").into(), ::serde_json::Value::Bool(false));
            let _ = object
                .insert(("openWorldHint").into(), ::serde_json::Value::Bool(true));
            object
        });
        mcp_core::types::Tool {
            name: "execute_search".to_string(),
            description: Some("Brand new tool".to_string()),
            input_schema: schema,
            annotations: Some(mcp_core::types::ToolAnnotations {
                title: Some("execute_search".to_string()),
                read_only_hint: Some(false),
                destructive_hint: Some(true),
                idempotent_hint: Some(false),
                open_world_hint: Some(true),
            }),
        }
    }
    pub fn call(ctx: Arc<ToolContext>) -> mcp_core::tools::ToolHandlerFn {
        Box::new(move |req: mcp_core::types::CallToolRequest| {
            let ctx = ctx.clone();
            Box::pin(async move {
                let params = match req.arguments {
                    Some(args) => serde_json::to_value(args).unwrap_or_default(),
                    None => serde_json::Value::Null,
                };
                let call_response = {
                    let args: ToolInput = match serde_json::from_value(params) {
                        Ok(p) => p,
                        Err(e) => {
                            return mcp_core::types::CallToolResponse {
                                content: <[_]>::into_vec(
                                    ::alloc::boxed::box_new([
                                        mcp_core::types::ToolResponseContent::Text(mcp_core::types::TextContent {
                                            content_type: "text".to_string(),
                                            text: ::alloc::__export::must_use({
                                                let res = ::alloc::fmt::format(
                                                    format_args!("Invalid parameters: {0}", e),
                                                );
                                                res
                                            }),
                                            annotations: None,
                                        }),
                                    ]),
                                ),
                                is_error: Some(true),
                                meta: req.meta,
                            };
                        }
                    };
                    execute_search(ctx, args).await
                };
                let call_tool_response = match call_response {
                    Ok(response) => {
                        mcp_core::types::CallToolResponse {
                            content: response.into(),
                            is_error: Some(false),
                            meta: req.meta,
                        }
                    }
                    Err(e) => {
                        mcp_core::types::CallToolResponse {
                            content: <[_]>::into_vec(
                                ::alloc::boxed::box_new([
                                    mcp_core::types::ToolResponseContent::Text(mcp_core::types::TextContent {
                                        content_type: "text".to_string(),
                                        text: ::alloc::__export::must_use({
                                            let res = ::alloc::fmt::format(
                                                format_args!("Tool execution error: {0}", e),
                                            );
                                            res
                                        }),
                                        annotations: None,
                                    }),
                                ]),
                            ),
                            is_error: Some(true),
                            meta: req.meta,
                        }
                    }
                };
                call_tool_response
            })
        })
    }
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
