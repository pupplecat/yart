#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::sync::Arc;
use anyhow::Result;
use mcp_core::types::{TextContent, ToolResponseContent};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use yart::mcp_tool;
pub struct SearchToolContext {
    value: String,
}
#[automatically_derived]
impl ::core::clone::Clone for SearchToolContext {
    #[inline]
    fn clone(&self) -> SearchToolContext {
        SearchToolContext {
            value: ::core::clone::Clone::clone(&self.value),
        }
    }
}
pub struct SearchToolArgs {
    value: String,
}
#[automatically_derived]
impl ::core::default::Default for SearchToolArgs {
    #[inline]
    fn default() -> SearchToolArgs {
        SearchToolArgs {
            value: ::core::default::Default::default(),
        }
    }
}
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for SearchToolArgs {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
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
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
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
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<SearchToolArgs>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SearchToolArgs;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct SearchToolArgs")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<String>(&mut __seq)?
                    {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                0usize,
                                &"struct SearchToolArgs with 1 element",
                            ))
                        }
                    };
                    _serde::__private::Ok(SearchToolArgs { value: __field0 })
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
                    while let _serde::__private::Some(__key) =
                        _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("value"),
                                    );
                                }
                                __field0 =
                                    _serde::__private::Some(_serde::de::MapAccess::next_value::<
                                        String,
                                    >(
                                        &mut __map
                                    )?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(
                                    &mut __map,
                                )?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => _serde::__private::de::missing_field("value")?,
                    };
                    _serde::__private::Ok(SearchToolArgs { value: __field0 })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["value"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "SearchToolArgs",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<SearchToolArgs>,
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
    clippy::absolute_paths
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for SearchToolArgs {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "SearchToolArgs",
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
    impl schemars::JsonSchema for SearchToolArgs {
        fn schema_name() -> std::string::String {
            "SearchToolArgs".to_owned()
        }
        fn schema_id() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("mcp_tool::SearchToolArgs")
        }
        fn json_schema(generator: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
            {
                let mut schema_object = schemars::schema::SchemaObject {
                    instance_type: Some(schemars::schema::InstanceType::Object.into()),
                    ..Default::default()
                };
                let object_validation = schema_object.object();
                {
                    schemars::_private::insert_object_property::<String>(
                        object_validation,
                        "value",
                        false,
                        false,
                        generator.subschema_for::<String>(),
                    );
                }
                schemars::schema::Schema::Object(schema_object)
            }
        }
    };
};
pub struct SearchToolResponse {
    result: String,
}
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for SearchToolResponse {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
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
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
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
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<SearchToolResponse>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SearchToolResponse;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct SearchToolResponse",
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
                    let __field0 = match _serde::de::SeqAccess::next_element::<String>(&mut __seq)?
                    {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                0usize,
                                &"struct SearchToolResponse with 1 element",
                            ))
                        }
                    };
                    _serde::__private::Ok(SearchToolResponse { result: __field0 })
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
                    while let _serde::__private::Some(__key) =
                        _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "result",
                                        ),
                                    );
                                }
                                __field0 =
                                    _serde::__private::Some(_serde::de::MapAccess::next_value::<
                                        String,
                                    >(
                                        &mut __map
                                    )?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(
                                    &mut __map,
                                )?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => _serde::__private::de::missing_field("result")?,
                    };
                    _serde::__private::Ok(SearchToolResponse { result: __field0 })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["result"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "SearchToolResponse",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<SearchToolResponse>,
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
    clippy::absolute_paths
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for SearchToolResponse {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "SearchToolResponse",
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
    impl schemars::JsonSchema for SearchToolResponse {
        fn schema_name() -> std::string::String {
            "SearchToolResponse".to_owned()
        }
        fn schema_id() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("mcp_tool::SearchToolResponse")
        }
        fn json_schema(generator: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
            {
                let mut schema_object = schemars::schema::SchemaObject {
                    instance_type: Some(schemars::schema::InstanceType::Object.into()),
                    ..Default::default()
                };
                let object_validation = schema_object.object();
                {
                    schemars::_private::insert_object_property::<String>(
                        object_validation,
                        "result",
                        false,
                        false,
                        generator.subschema_for::<String>(),
                    );
                }
                schemars::schema::Schema::Object(schema_object)
            }
        }
    };
};
impl Into<ToolResponseContent> for SearchToolResponse {
    fn into(self) -> ToolResponseContent {
        ToolResponseContent::Text(TextContent {
            content_type: "text".to_string(),
            text: "Hello, world!".to_string(),
            annotations: None,
        })
    }
}
pub struct SearchToolMcpParameters {
    args: SearchToolArgs,
}
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for SearchToolMcpParameters {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "SearchToolMcpParameters",
                false as usize + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "args", &self.args)?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for SearchToolMcpParameters {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
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
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
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
                        "args" => _serde::__private::Ok(__Field::__field0),
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
                        b"args" => _serde::__private::Ok(__Field::__field0),
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
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<SearchToolMcpParameters>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SearchToolMcpParameters;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct SearchToolMcpParameters",
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
                    let __field0 =
                        match _serde::de::SeqAccess::next_element::<SearchToolArgs>(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct SearchToolMcpParameters with 1 element",
                                ))
                            }
                        };
                    _serde::__private::Ok(SearchToolMcpParameters { args: __field0 })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<SearchToolArgs> =
                        _serde::__private::None;
                    while let _serde::__private::Some(__key) =
                        _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("args"),
                                    );
                                }
                                __field0 =
                                    _serde::__private::Some(_serde::de::MapAccess::next_value::<
                                        SearchToolArgs,
                                    >(
                                        &mut __map
                                    )?);
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<_serde::de::IgnoredAny>(
                                    &mut __map,
                                )?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => _serde::__private::de::missing_field("args")?,
                    };
                    _serde::__private::Ok(SearchToolMcpParameters { args: __field0 })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["args"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "SearchToolMcpParameters",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<SearchToolMcpParameters>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
const _: () = {
    #[automatically_derived]
    #[allow(unused_braces)]
    impl schemars::JsonSchema for SearchToolMcpParameters {
        fn schema_name() -> std::string::String {
            "SearchToolMcpParameters".to_owned()
        }
        fn schema_id() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("mcp_tool::SearchToolMcpParameters")
        }
        fn json_schema(generator: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
            {
                let mut schema_object = schemars::schema::SchemaObject {
                    instance_type: Some(schemars::schema::InstanceType::Object.into()),
                    ..Default::default()
                };
                let object_validation = schema_object.object();
                {
                    schemars::_private::insert_object_property::<SearchToolArgs>(
                        object_validation,
                        "args",
                        false,
                        false,
                        generator.subschema_for::<SearchToolArgs>(),
                    );
                }
                schemars::schema::Schema::Object(schema_object)
            }
        }
    };
};
#[automatically_derived]
impl ::core::default::Default for SearchToolMcpParameters {
    #[inline]
    fn default() -> SearchToolMcpParameters {
        SearchToolMcpParameters {
            args: ::core::default::Default::default(),
        }
    }
}
async fn search_tool(
    ctx: Arc<SearchToolContext>,
    args: SearchToolArgs,
) -> Result<SearchToolResponse> {
    Ok(SearchToolResponse {
        result: "response".to_string(),
    })
}
pub struct SearchToolMcp;
#[automatically_derived]
impl ::core::default::Default for SearchToolMcp {
    #[inline]
    fn default() -> SearchToolMcp {
        SearchToolMcp {}
    }
}
impl SearchToolMcp {
    pub fn tool() -> mcp_core::types::Tool {
        let schema = serde_json::to_value(
            ::schemars::gen::SchemaGenerator::default().into_root_schema_for::<SearchToolArgs>(),
        )
        .expect("Failed to serialize schema");
        let annotations = ::serde_json::Value::Object({
            let mut object = ::serde_json::Map::new();
            let _ = object.insert(
                ("title").into(),
                ::serde_json::to_value(&"search_tool").unwrap(),
            );
            let _ = object.insert(("readOnlyHint").into(), ::serde_json::Value::Bool(false));
            let _ = object.insert(("destructiveHint").into(), ::serde_json::Value::Bool(true));
            let _ = object.insert(("idempotentHint").into(), ::serde_json::Value::Bool(false));
            let _ = object.insert(("openWorldHint").into(), ::serde_json::Value::Bool(true));
            object
        });
        mcp_core::types::Tool {
            name: "search_tool".to_string(),
            description: Some("Brand new tool".to_string()),
            input_schema: schema,
            annotations: Some(mcp_core::types::ToolAnnotations {
                title: Some("search_tool".to_string()),
                read_only_hint: Some(false),
                destructive_hint: Some(true),
                idempotent_hint: Some(false),
                open_world_hint: Some(true),
            }),
        }
    }
    pub fn call(ctx: Arc<SearchToolContext>) -> mcp_core::tools::ToolHandlerFn {
        let ctx = ctx.clone();
        move |req: mcp_core::types::CallToolRequest| {
            Box::pin(async move {
                let params = match req.arguments {
                    Some(args) => serde_json::to_value(args).unwrap_or_default(),
                    None => serde_json::Value::Null,
                };
                let call_response = yart::wrap_unsafe(move || async move {
                    let params: SearchToolMcpParameters = match serde_json::from_value(params) {
                        Ok(p) => p,
                        Err(e) => {
                            return mcp_core::types::CallToolResponse {
                                content: <[_]>::into_vec(::alloc::boxed::box_new([
                                    mcp_core::types::ToolResponseContent::Text(
                                        mcp_core::types::TextContent {
                                            content_type: "text".to_string(),
                                            text: ::alloc::__export::must_use({
                                                let res = ::alloc::fmt::format(format_args!(
                                                    "Invalid parameters: {0}",
                                                    e
                                                ));
                                                res
                                            }),
                                            annotations: None,
                                        },
                                    ),
                                ])),
                                is_error: Some(true),
                                meta: req.meta,
                            }
                        }
                    };
                    search_tool(ctx, params.args).await.map_err(|e| {
                        ::anyhow::__private::must_use({
                            use ::anyhow::__private::kind::*;
                            let error = match e.to_string() {
                                error => (&error).anyhow_kind().new(error),
                            };
                            error
                        })
                    })
                })
                .await?;
                let call_tool_response = match call_response {
                    Ok::<SearchToolResponse, _>(response) => {
                        let content = <[_]>::into_vec(::alloc::boxed::box_new([response.into()]));
                        mcp_core::types::CallToolResponse {
                            content,
                            is_error: Some(false),
                            meta: req.meta,
                        }
                    }
                    Err(e) => mcp_core::types::CallToolResponse {
                        content: <[_]>::into_vec(::alloc::boxed::box_new([
                            mcp_core::types::ToolResponseContent::Text(
                                mcp_core::types::TextContent {
                                    content_type: "text".to_string(),
                                    text: ::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(format_args!(
                                            "Tool execution error: {0}",
                                            e
                                        ));
                                        res
                                    }),
                                    annotations: None,
                                },
                            ),
                        ])),
                        is_error: Some(true),
                        meta: req.meta,
                    },
                };
                call_tool_response;
                mcp_core::types::CallToolResponse {
                    content: <[_]>::into_vec(::alloc::boxed::box_new([
                        mcp_core::types::ToolResponseContent::Text(mcp_core::types::TextContent {
                            content_type: "text".to_string(),
                            text: ::alloc::__export::must_use({
                                let res =
                                    ::alloc::fmt::format(format_args!("Tool execution error: "));
                                res
                            }),
                            annotations: None,
                        }),
                    ])),
                    is_error: Some(true),
                    meta: req.meta,
                }
            })
        }
    }
}
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[])
}
