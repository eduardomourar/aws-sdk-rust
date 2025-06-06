// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_rate_based_statement_custom_key(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::RateBasedStatementCustomKey,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.header {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Header").start_object();
        crate::protocol_serde::shape_rate_limit_header::ser_rate_limit_header(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.cookie {
        #[allow(unused_mut)]
        let mut object_4 = object.key("Cookie").start_object();
        crate::protocol_serde::shape_rate_limit_cookie::ser_rate_limit_cookie(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.query_argument {
        #[allow(unused_mut)]
        let mut object_6 = object.key("QueryArgument").start_object();
        crate::protocol_serde::shape_rate_limit_query_argument::ser_rate_limit_query_argument(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.query_string {
        #[allow(unused_mut)]
        let mut object_8 = object.key("QueryString").start_object();
        crate::protocol_serde::shape_rate_limit_query_string::ser_rate_limit_query_string(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.http_method {
        #[allow(unused_mut)]
        let mut object_10 = object.key("HTTPMethod").start_object();
        crate::protocol_serde::shape_rate_limit_http_method::ser_rate_limit_http_method(&mut object_10, var_9)?;
        object_10.finish();
    }
    if let Some(var_11) = &input.forwarded_ip {
        #[allow(unused_mut)]
        let mut object_12 = object.key("ForwardedIP").start_object();
        crate::protocol_serde::shape_rate_limit_forwarded_ip::ser_rate_limit_forwarded_ip(&mut object_12, var_11)?;
        object_12.finish();
    }
    if let Some(var_13) = &input.ip {
        #[allow(unused_mut)]
        let mut object_14 = object.key("IP").start_object();
        crate::protocol_serde::shape_rate_limit_ip::ser_rate_limit_ip(&mut object_14, var_13)?;
        object_14.finish();
    }
    if let Some(var_15) = &input.label_namespace {
        #[allow(unused_mut)]
        let mut object_16 = object.key("LabelNamespace").start_object();
        crate::protocol_serde::shape_rate_limit_label_namespace::ser_rate_limit_label_namespace(&mut object_16, var_15)?;
        object_16.finish();
    }
    if let Some(var_17) = &input.uri_path {
        #[allow(unused_mut)]
        let mut object_18 = object.key("UriPath").start_object();
        crate::protocol_serde::shape_rate_limit_uri_path::ser_rate_limit_uri_path(&mut object_18, var_17)?;
        object_18.finish();
    }
    if let Some(var_19) = &input.ja3_fingerprint {
        #[allow(unused_mut)]
        let mut object_20 = object.key("JA3Fingerprint").start_object();
        crate::protocol_serde::shape_rate_limit_ja3_fingerprint::ser_rate_limit_ja3_fingerprint(&mut object_20, var_19)?;
        object_20.finish();
    }
    if let Some(var_21) = &input.ja4_fingerprint {
        #[allow(unused_mut)]
        let mut object_22 = object.key("JA4Fingerprint").start_object();
        crate::protocol_serde::shape_rate_limit_ja4_fingerprint::ser_rate_limit_ja4_fingerprint(&mut object_22, var_21)?;
        object_22.finish();
    }
    if let Some(var_23) = &input.asn {
        #[allow(unused_mut)]
        let mut object_24 = object.key("ASN").start_object();
        crate::protocol_serde::shape_rate_limit_asn::ser_rate_limit_asn(&mut object_24, var_23)?;
        object_24.finish();
    }
    Ok(())
}

pub(crate) fn de_rate_based_statement_custom_key<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::RateBasedStatementCustomKey>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::RateBasedStatementCustomKeyBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "Header" => {
                            builder = builder.set_header(crate::protocol_serde::shape_rate_limit_header::de_rate_limit_header(tokens)?);
                        }
                        "Cookie" => {
                            builder = builder.set_cookie(crate::protocol_serde::shape_rate_limit_cookie::de_rate_limit_cookie(tokens)?);
                        }
                        "QueryArgument" => {
                            builder = builder.set_query_argument(
                                crate::protocol_serde::shape_rate_limit_query_argument::de_rate_limit_query_argument(tokens)?,
                            );
                        }
                        "QueryString" => {
                            builder =
                                builder.set_query_string(crate::protocol_serde::shape_rate_limit_query_string::de_rate_limit_query_string(tokens)?);
                        }
                        "HTTPMethod" => {
                            builder =
                                builder.set_http_method(crate::protocol_serde::shape_rate_limit_http_method::de_rate_limit_http_method(tokens)?);
                        }
                        "ForwardedIP" => {
                            builder =
                                builder.set_forwarded_ip(crate::protocol_serde::shape_rate_limit_forwarded_ip::de_rate_limit_forwarded_ip(tokens)?);
                        }
                        "IP" => {
                            builder = builder.set_ip(crate::protocol_serde::shape_rate_limit_ip::de_rate_limit_ip(tokens)?);
                        }
                        "LabelNamespace" => {
                            builder = builder.set_label_namespace(
                                crate::protocol_serde::shape_rate_limit_label_namespace::de_rate_limit_label_namespace(tokens)?,
                            );
                        }
                        "UriPath" => {
                            builder = builder.set_uri_path(crate::protocol_serde::shape_rate_limit_uri_path::de_rate_limit_uri_path(tokens)?);
                        }
                        "JA3Fingerprint" => {
                            builder = builder.set_ja3_fingerprint(
                                crate::protocol_serde::shape_rate_limit_ja3_fingerprint::de_rate_limit_ja3_fingerprint(tokens)?,
                            );
                        }
                        "JA4Fingerprint" => {
                            builder = builder.set_ja4_fingerprint(
                                crate::protocol_serde::shape_rate_limit_ja4_fingerprint::de_rate_limit_ja4_fingerprint(tokens)?,
                            );
                        }
                        "ASN" => {
                            builder = builder.set_asn(crate::protocol_serde::shape_rate_limit_asn::de_rate_limit_asn(tokens)?);
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
