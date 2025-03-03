// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aws_cloud_front_distribution_origin_custom_origin_config(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AwsCloudFrontDistributionOriginCustomOriginConfig,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.http_port {
        object.key("HttpPort").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_1).into()),
        );
    }
    if let Some(var_2) = &input.https_port {
        object.key("HttpsPort").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    if let Some(var_3) = &input.origin_keepalive_timeout {
        object.key("OriginKeepaliveTimeout").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_3).into()),
        );
    }
    if let Some(var_4) = &input.origin_protocol_policy {
        object.key("OriginProtocolPolicy").string(var_4.as_str());
    }
    if let Some(var_5) = &input.origin_read_timeout {
        object.key("OriginReadTimeout").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_5).into()),
        );
    }
    if let Some(var_6) = &input.origin_ssl_protocols {
        #[allow(unused_mut)]
        let mut object_7 = object.key("OriginSslProtocols").start_object();
        crate::protocol_serde::shape_aws_cloud_front_distribution_origin_ssl_protocols::ser_aws_cloud_front_distribution_origin_ssl_protocols(
            &mut object_7,
            var_6,
        )?;
        object_7.finish();
    }
    Ok(())
}

pub(crate) fn de_aws_cloud_front_distribution_origin_custom_origin_config<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<
    Option<crate::types::AwsCloudFrontDistributionOriginCustomOriginConfig>,
    ::aws_smithy_json::deserialize::error::DeserializeError,
>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::AwsCloudFrontDistributionOriginCustomOriginConfigBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "HttpPort" => {
                            builder = builder.set_http_port(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "HttpsPort" => {
                            builder = builder.set_https_port(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "OriginKeepaliveTimeout" => {
                            builder = builder.set_origin_keepalive_timeout(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "OriginProtocolPolicy" => {
                            builder = builder.set_origin_protocol_policy(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "OriginReadTimeout" => {
                            builder = builder.set_origin_read_timeout(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "OriginSslProtocols" => {
                            builder = builder.set_origin_ssl_protocols(
                                    crate::protocol_serde::shape_aws_cloud_front_distribution_origin_ssl_protocols::de_aws_cloud_front_distribution_origin_ssl_protocols(tokens)?
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
            Ok(Some(builder.build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
