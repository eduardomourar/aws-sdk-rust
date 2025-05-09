// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_virtual_gateway_tls_validation_context_trust(
    object_2: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::VirtualGatewayTlsValidationContextTrust,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    match input {
        crate::types::VirtualGatewayTlsValidationContextTrust::Acm(inner) => {
            #[allow(unused_mut)]
            let mut object_1 = object_2.key("acm").start_object();
            crate::protocol_serde::shape_virtual_gateway_tls_validation_context_acm_trust::ser_virtual_gateway_tls_validation_context_acm_trust(
                &mut object_1,
                inner,
            )?;
            object_1.finish();
        }
        crate::types::VirtualGatewayTlsValidationContextTrust::File(inner) => {
            #[allow(unused_mut)]
            let mut object_2 = object_2.key("file").start_object();
            crate::protocol_serde::shape_virtual_gateway_tls_validation_context_file_trust::ser_virtual_gateway_tls_validation_context_file_trust(
                &mut object_2,
                inner,
            )?;
            object_2.finish();
        }
        crate::types::VirtualGatewayTlsValidationContextTrust::Sds(inner) => {
            #[allow(unused_mut)]
            let mut object_3 = object_2.key("sds").start_object();
            crate::protocol_serde::shape_virtual_gateway_tls_validation_context_sds_trust::ser_virtual_gateway_tls_validation_context_sds_trust(
                &mut object_3,
                inner,
            )?;
            object_3.finish();
        }
        crate::types::VirtualGatewayTlsValidationContextTrust::Unknown => {
            return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
                "VirtualGatewayTlsValidationContextTrust",
            ))
        }
    }
    Ok(())
}

pub(crate) fn de_virtual_gateway_tls_validation_context_trust<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::VirtualGatewayTlsValidationContextTrust>, ::aws_smithy_json::deserialize::error::DeserializeError>
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
                            "acm" => {
                                Some(crate::types::VirtualGatewayTlsValidationContextTrust::Acm(
                                    crate::protocol_serde::shape_virtual_gateway_tls_validation_context_acm_trust::de_virtual_gateway_tls_validation_context_acm_trust(tokens)?
                                    .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'acm' cannot be null"))?
                                ))
                            }
                            "file" => {
                                Some(crate::types::VirtualGatewayTlsValidationContextTrust::File(
                                    crate::protocol_serde::shape_virtual_gateway_tls_validation_context_file_trust::de_virtual_gateway_tls_validation_context_file_trust(tokens)?
                                    .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'file' cannot be null"))?
                                ))
                            }
                            "sds" => {
                                Some(crate::types::VirtualGatewayTlsValidationContextTrust::Sds(
                                    crate::protocol_serde::shape_virtual_gateway_tls_validation_context_sds_trust::de_virtual_gateway_tls_validation_context_sds_trust(tokens)?
                                    .ok_or_else(|| ::aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'sds' cannot be null"))?
                                ))
                            }
                            _ => {
                                                                              ::aws_smithy_json::deserialize::token::skip_value(tokens)?;
                                                                              Some(crate::types::VirtualGatewayTlsValidationContextTrust::Unknown)
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
