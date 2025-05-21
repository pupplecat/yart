#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fmt::format, sync::Arc};
use yart::ToolError;
struct TestContext {
    value: String,
}
#[automatically_derived]
impl ::core::clone::Clone for TestContext {
    #[inline]
    fn clone(&self) -> TestContext {
        TestContext {
            value: ::core::clone::Clone::clone(&self.value),
        }
    }
}
#[automatically_derived]
impl ::core::default::Default for TestContext {
    #[inline]
    fn default() -> TestContext {
        TestContext {
            value: ::core::default::Default::default(),
        }
    }
}
struct TestArgs {
    param: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    optional_param: Option<String>,
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
    impl _serde::Serialize for TestArgs {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "TestArgs",
                false as usize
                    + 1
                    + if Option::is_none(&self.optional_param) {
                        0
                    } else {
                        1
                    },
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "param",
                &self.param,
            )?;
            if !Option::is_none(&self.optional_param) {
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "optional_param",
                    &self.optional_param,
                )?;
            } else {
                _serde::ser::SerializeStruct::skip_field(&mut __serde_state, "optional_param")?;
            }
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
    impl<'de> _serde::Deserialize<'de> for TestArgs {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
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
                        1u64 => _serde::__private::Ok(__Field::__field1),
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
                        "param" => _serde::__private::Ok(__Field::__field0),
                        "optional_param" => _serde::__private::Ok(__Field::__field1),
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
                        b"param" => _serde::__private::Ok(__Field::__field0),
                        b"optional_param" => _serde::__private::Ok(__Field::__field1),
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
                marker: _serde::__private::PhantomData<TestArgs>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = TestArgs;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct TestArgs")
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
                                &"struct TestArgs with 2 elements",
                            ))
                        }
                    };
                    let __field1 =
                        match _serde::de::SeqAccess::next_element::<Option<String>>(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct TestArgs with 2 elements",
                                ))
                            }
                        };
                    _serde::__private::Ok(TestArgs {
                        param: __field0,
                        optional_param: __field1,
                    })
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
                    let mut __field1: _serde::__private::Option<Option<String>> =
                        _serde::__private::None;
                    while let _serde::__private::Some(__key) =
                        _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("param"),
                                    );
                                }
                                __field0 =
                                    _serde::__private::Some(_serde::de::MapAccess::next_value::<
                                        String,
                                    >(
                                        &mut __map
                                    )?);
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "optional_param",
                                        ),
                                    );
                                }
                                __field1 =
                                    _serde::__private::Some(_serde::de::MapAccess::next_value::<
                                        Option<String>,
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
                        _serde::__private::None => _serde::__private::de::missing_field("param")?,
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("optional_param")?
                        }
                    };
                    _serde::__private::Ok(TestArgs {
                        param: __field0,
                        optional_param: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["param", "optional_param"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "TestArgs",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<TestArgs>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
const _: () = {
    #[automatically_derived]
    #[allow(unused_braces)]
    impl schemars::JsonSchema for TestArgs {
        fn schema_name() -> std::string::String {
            "TestArgs".to_owned()
        }
        fn schema_id() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("mcp_tool::TestArgs")
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
                        "param",
                        false,
                        false,
                        generator.subschema_for::<String>(),
                    );
                }
                {
                    schemars::_private::insert_object_property::<Option<String>>(
                        object_validation,
                        "optional_param",
                        false,
                        false,
                        generator.subschema_for::<Option<String>>(),
                    );
                }
                schemars::schema::Schema::Object(schema_object)
            }
        }
    };
};
struct TestOutput {
    value: String,
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
    impl _serde::Serialize for TestOutput {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "TestOutput",
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
    impl<'de> _serde::Deserialize<'de> for TestOutput {
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
                marker: _serde::__private::PhantomData<TestOutput>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = TestOutput;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct TestOutput")
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
                                &"struct TestOutput with 1 element",
                            ))
                        }
                    };
                    _serde::__private::Ok(TestOutput { value: __field0 })
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
                    _serde::__private::Ok(TestOutput { value: __field0 })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["value"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "TestOutput",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<TestOutput>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
const _: () = {
    #[automatically_derived]
    #[allow(unused_braces)]
    impl schemars::JsonSchema for TestOutput {
        fn schema_name() -> std::string::String {
            "TestOutput".to_owned()
        }
        fn schema_id() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("mcp_tool::TestOutput")
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
struct ExampleToolParameters {
    ctx: Arc<TestContext>,
    args: TestArgs,
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
    impl<'de> _serde::Deserialize<'de> for ExampleToolParameters {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
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
                        1u64 => _serde::__private::Ok(__Field::__field1),
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
                        "ctx" => _serde::__private::Ok(__Field::__field0),
                        "args" => _serde::__private::Ok(__Field::__field1),
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
                        b"ctx" => _serde::__private::Ok(__Field::__field0),
                        b"args" => _serde::__private::Ok(__Field::__field1),
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
                marker: _serde::__private::PhantomData<ExampleToolParameters>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = ExampleToolParameters;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct ExampleToolParameters",
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
                    let __field0 = match _serde::de::SeqAccess::next_element::<Arc<TestContext>>(
                        &mut __seq,
                    )? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(_serde::de::Error::invalid_length(
                                0usize,
                                &"struct ExampleToolParameters with 2 elements",
                            ))
                        }
                    };
                    let __field1 =
                        match _serde::de::SeqAccess::next_element::<TestArgs>(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct ExampleToolParameters with 2 elements",
                                ))
                            }
                        };
                    _serde::__private::Ok(ExampleToolParameters {
                        ctx: __field0,
                        args: __field1,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<Arc<TestContext>> =
                        _serde::__private::None;
                    let mut __field1: _serde::__private::Option<TestArgs> = _serde::__private::None;
                    while let _serde::__private::Some(__key) =
                        _serde::de::MapAccess::next_key::<__Field>(&mut __map)?
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("ctx"),
                                    );
                                }
                                __field0 =
                                    _serde::__private::Some(_serde::de::MapAccess::next_value::<
                                        Arc<TestContext>,
                                    >(
                                        &mut __map
                                    )?);
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("args"),
                                    );
                                }
                                __field1 =
                                    _serde::__private::Some(_serde::de::MapAccess::next_value::<
                                        TestArgs,
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
                        _serde::__private::None => _serde::__private::de::missing_field("ctx")?,
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => _serde::__private::de::missing_field("args")?,
                    };
                    _serde::__private::Ok(ExampleToolParameters {
                        ctx: __field0,
                        args: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["ctx", "args"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "ExampleToolParameters",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<ExampleToolParameters>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
const _: () = {
    #[automatically_derived]
    #[allow(unused_braces)]
    impl schemars::JsonSchema for ExampleToolParameters {
        fn schema_name() -> std::string::String {
            "ExampleToolParameters".to_owned()
        }
        fn schema_id() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("mcp_tool::ExampleToolParameters")
        }
        fn json_schema(generator: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
            {
                let mut schema_object = schemars::schema::SchemaObject {
                    instance_type: Some(schemars::schema::InstanceType::Object.into()),
                    ..Default::default()
                };
                let object_validation = schema_object.object();
                {
                    schemars::_private::insert_object_property::<Arc<TestContext>>(
                        object_validation,
                        "ctx",
                        false,
                        false,
                        generator.subschema_for::<Arc<TestContext>>(),
                    );
                }
                {
                    schemars::_private::insert_object_property::<TestArgs>(
                        object_validation,
                        "args",
                        false,
                        false,
                        generator.subschema_for::<TestArgs>(),
                    );
                }
                schemars::schema::Schema::Object(schema_object)
            }
        }
    };
};
pub struct ExampleTool {
    context: Arc<TestContext>,
}
impl ExampleTool {
    pub fn new() -> Self {
        Self { context: () }
    }
    pub fn tool() -> mcp_core::types::Tool {
        mcp_core::types::Tool {
            name: "example_tool".to_string(),
            description: Some("Test tool".to_string()),
            input_schema: yart::derive_parameters::<TestArgs>(),
            annotations: Some(mcp_core::types::ToolAnnotations {
                title: Some("example_tool".to_string()),
                read_only_hint: Some(false),
                destructive_hint: Some(true),
                idempotent_hint: Some(false),
                open_world_hint: Some(true),
            }),
        }
    }
    pub fn handler() -> mcp_core::tools::ToolHandler {
        mcp_core::tools::ToolHandler {
            tool: Self::tool(),
            f: Box::new(
                |req: mcp_core::types::CallToolRequest| -> std::pin::Pin<
                    Box<dyn std::future::Future<Output = mcp_core::types::CallToolResponse> + Send>,
                > {
                    Box::pin(async move {
                        let tool = ExampleTool::new(());
                        let params = match req.arguments {
                            Some(args) => serde_json::to_value(args).unwrap_or_default(),
                            None => serde_json::Value::Null,
                        };
                        let params: ExampleToolParameters = match serde_json::from_value(params) {
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
                        let ctx = self.context.clone();
                        match ExampleTool::internal_call(params.ctx, params.args).await {
                            Ok(response) => {
                                let content = if let Ok(vec_content) = serde_json::from_value::<
                                    Vec<mcp_core::types::ToolResponseContent>,
                                >(
                                    serde_json::to_value(&response).unwrap_or_default(),
                                ) {
                                    vec_content
                                } else if let Ok(single_content) =
                                    serde_json::from_value::<mcp_core::types::ToolResponseContent>(
                                        serde_json::to_value(&response).unwrap_or_default(),
                                    )
                                {
                                    <[_]>::into_vec(::alloc::boxed::box_new([single_content]))
                                } else {
                                    <[_]>::into_vec(::alloc::boxed::box_new([
                                        mcp_core::types::ToolResponseContent::Text(
                                            mcp_core::types::TextContent {
                                                content_type: "text".to_string(),
                                                text: ::alloc::__export::must_use({
                                                    let res = ::alloc::fmt::format(format_args!(
                                                        "Invalid response type: {0:?}",
                                                        response
                                                    ));
                                                    res
                                                }),
                                                annotations: None,
                                            },
                                        ),
                                    ]))
                                };
                                mcp_core::types::CallToolResponse {
                                    content,
                                    is_error: None,
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
                        }
                    })
                },
            ),
        }
    }
    async fn internal_call(
        ctx: Arc<TestContext>,
        args: TestArgs,
    ) -> Result<TestOutput, anyhow::Error> {
        {
            Ok(TestOutput {
                value: ::alloc::__export::must_use({
                    let res =
                        ::alloc::fmt::format(format_args!("Echo: {0} {1}", ctx.value, args.param));
                    res
                }),
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
