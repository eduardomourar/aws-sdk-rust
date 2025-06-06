// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_usage_tracking_resource(
    object_11: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::UsageTrackingResource,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    match input {
        crate::types::UsageTrackingResource::QueueId(inner) => {
            object_11.key("queueId").string(inner.as_str());
        }
        crate::types::UsageTrackingResource::Unknown => {
            return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
                "UsageTrackingResource",
            ))
        }
    }
    Ok(())
}

pub(crate) fn de_usage_tracking_resource<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::UsageTrackingResource>, ::aws_smithy_json::deserialize::error::DeserializeError>
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
                        "queueId" => Some(crate::types::UsageTrackingResource::QueueId(
                            ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                .transpose()?
                                .ok_or_else(|| {
                                    ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'queueId' cannot be null")
                                })?,
                        )),
                        _ => {
                            ::aws_smithy_json::deserialize::token::skip_value(tokens)?;
                            Some(crate::types::UsageTrackingResource::Unknown)
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
