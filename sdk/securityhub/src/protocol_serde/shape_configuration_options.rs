// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_configuration_options<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::ConfigurationOptions>, ::aws_smithy_json::deserialize::error::DeserializeError>
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
                        "Integer" => Some(crate::types::ConfigurationOptions::Integer(
                            crate::protocol_serde::shape_integer_configuration_options::de_integer_configuration_options(tokens)?.ok_or_else(
                                || ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'Integer' cannot be null"),
                            )?,
                        )),
                        "IntegerList" => Some(crate::types::ConfigurationOptions::IntegerList(
                            crate::protocol_serde::shape_integer_list_configuration_options::de_integer_list_configuration_options(tokens)?
                                .ok_or_else(|| {
                                    ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'IntegerList' cannot be null")
                                })?,
                        )),
                        "Double" => Some(crate::types::ConfigurationOptions::Double(
                            crate::protocol_serde::shape_double_configuration_options::de_double_configuration_options(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'Double' cannot be null")
                            })?,
                        )),
                        "String" => Some(crate::types::ConfigurationOptions::String(
                            crate::protocol_serde::shape_string_configuration_options::de_string_configuration_options(tokens)?.ok_or_else(|| {
                                ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'String' cannot be null")
                            })?,
                        )),
                        "StringList" => Some(crate::types::ConfigurationOptions::StringList(
                            crate::protocol_serde::shape_string_list_configuration_options::de_string_list_configuration_options(tokens)?
                                .ok_or_else(|| {
                                    ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'StringList' cannot be null")
                                })?,
                        )),
                        "Boolean" => Some(crate::types::ConfigurationOptions::Boolean(
                            crate::protocol_serde::shape_boolean_configuration_options::de_boolean_configuration_options(tokens)?.ok_or_else(
                                || ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'Boolean' cannot be null"),
                            )?,
                        )),
                        "Enum" => Some(crate::types::ConfigurationOptions::Enum(
                            crate::protocol_serde::shape_enum_configuration_options::de_enum_configuration_options(tokens)?
                                .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'Enum' cannot be null"))?,
                        )),
                        "EnumList" => Some(crate::types::ConfigurationOptions::EnumList(
                            crate::protocol_serde::shape_enum_list_configuration_options::de_enum_list_configuration_options(tokens)?.ok_or_else(
                                || ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'EnumList' cannot be null"),
                            )?,
                        )),
                        _ => {
                            ::aws_smithy_json::deserialize::token::skip_value(tokens)?;
                            Some(crate::types::ConfigurationOptions::Unknown)
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
