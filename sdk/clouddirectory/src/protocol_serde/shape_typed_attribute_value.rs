// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_typed_attribute_value(
    object_4: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::TypedAttributeValue,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    match input {
        crate::types::TypedAttributeValue::StringValue(inner) => {
            object_4.key("StringValue").string(inner.as_str());
        }
        crate::types::TypedAttributeValue::BinaryValue(inner) => {
            object_4.key("BinaryValue").string_unchecked(&::aws_smithy_types::base64::encode(inner));
        }
        crate::types::TypedAttributeValue::BooleanValue(inner) => {
            object_4.key("BooleanValue").boolean(*inner);
        }
        crate::types::TypedAttributeValue::NumberValue(inner) => {
            object_4.key("NumberValue").string(inner.as_str());
        }
        crate::types::TypedAttributeValue::DatetimeValue(inner) => {
            object_4
                .key("DatetimeValue")
                .date_time(inner, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
        }
        crate::types::TypedAttributeValue::Unknown => {
            return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
                "TypedAttributeValue",
            ))
        }
    }
    Ok(())
}

pub(crate) fn de_typed_attribute_value<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::TypedAttributeValue>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    let mut variant = None;
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => return Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => loop {
            match tokens.next().transpose()? {
                Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                    if let ::std::option::Option::Some(::std::result::Result::Ok(::aws_smithy_json::deserialize::Token::ValueNull { .. })) =
                        tokens.peek()
                    {
                        let _ = tokens.next().expect("peek returned a token")?;
                        continue;
                    }
                    let key = key.to_unescaped()?;
                    if key == "__type" {
                        ::aws_smithy_json::deserialize::token::skip_value(tokens)?;
                        continue;
                    }
                    if variant.is_some() {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
                            "encountered mixed variants in union",
                        ));
                    }
                    variant = match key.as_ref() {
                        "StringValue" => Some(crate::types::TypedAttributeValue::StringValue(
                            ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?
                                .ok_or_else(|| {
                                    ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'StringValue' cannot be null")
                                })?,
                        )),
                        "BinaryValue" => Some(crate::types::TypedAttributeValue::BinaryValue(
                            ::aws_smithy_json::deserialize::token::expect_blob_or_null(tokens.next())?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'BinaryValue' cannot be null")
                            })?,
                        )),
                        "BooleanValue" => Some(crate::types::TypedAttributeValue::BooleanValue(
                            ::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'BooleanValue' cannot be null")
                            })?,
                        )),
                        "NumberValue" => Some(crate::types::TypedAttributeValue::NumberValue(
                            ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?
                                .ok_or_else(|| {
                                    ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'NumberValue' cannot be null")
                                })?,
                        )),
                        "DatetimeValue" => Some(crate::types::TypedAttributeValue::DatetimeValue(
                            ::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?
                            .ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'DatetimeValue' cannot be null")
                            })?,
                        )),
                        _ => {
                            ::aws_smithy_json::deserialize::token::skip_value(tokens)?;
                            Some(crate::types::TypedAttributeValue::Unknown)
                        }
                    };
                }
                other => {
                    return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                        "expected object key or end object, found: {:?}",
                        other
                    )))
                }
            }
        },
        _ => {
            return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
                "expected start object or null",
            ))
        }
    }
    if variant.is_none() {
        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "Union did not contain a valid variant.",
        ));
    }
    Ok(variant)
}
