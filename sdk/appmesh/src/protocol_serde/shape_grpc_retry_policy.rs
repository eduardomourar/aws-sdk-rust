// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_grpc_retry_policy(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::GrpcRetryPolicy,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.per_retry_timeout {
        #[allow(unused_mut)]
        let mut object_2 = object.key("perRetryTimeout").start_object();
        crate::protocol_serde::shape_duration::ser_duration(&mut object_2, var_1)?;
        object_2.finish();
    }
    {
        object.key("maxRetries").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.max_retries).into()),
        );
    }
    if let Some(var_3) = &input.http_retry_events {
        let mut array_4 = object.key("httpRetryEvents").start_array();
        for item_5 in var_3 {
            {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    if let Some(var_6) = &input.tcp_retry_events {
        let mut array_7 = object.key("tcpRetryEvents").start_array();
        for item_8 in var_6 {
            {
                array_7.value().string(item_8.as_str());
            }
        }
        array_7.finish();
    }
    if let Some(var_9) = &input.grpc_retry_events {
        let mut array_10 = object.key("grpcRetryEvents").start_array();
        for item_11 in var_9 {
            {
                array_10.value().string(item_11.as_str());
            }
        }
        array_10.finish();
    }
    Ok(())
}

pub(crate) fn de_grpc_retry_policy<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::GrpcRetryPolicy>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::GrpcRetryPolicyBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "perRetryTimeout" => {
                            builder = builder.set_per_retry_timeout(crate::protocol_serde::shape_duration::de_duration(tokens)?);
                        }
                        "maxRetries" => {
                            builder = builder.set_max_retries(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i64::try_from)
                                    .transpose()?,
                            );
                        }
                        "httpRetryEvents" => {
                            builder = builder.set_http_retry_events(
                                crate::protocol_serde::shape_http_retry_policy_events::de_http_retry_policy_events(tokens)?,
                            );
                        }
                        "tcpRetryEvents" => {
                            builder = builder
                                .set_tcp_retry_events(crate::protocol_serde::shape_tcp_retry_policy_events::de_tcp_retry_policy_events(tokens)?);
                        }
                        "grpcRetryEvents" => {
                            builder = builder.set_grpc_retry_events(
                                crate::protocol_serde::shape_grpc_retry_policy_events::de_grpc_retry_policy_events(tokens)?,
                            );
                        }
                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                    },
                    other => {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                            "expected object key or end object, found: {:?}",
                            other
                        )))
                    }
                }
            }
            Ok(Some(crate::serde_util::grpc_retry_policy_correct_errors(builder).build().map_err(
                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
            )?))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
