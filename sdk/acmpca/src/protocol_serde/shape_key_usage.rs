// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_key_usage(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::KeyUsage,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if input.digital_signature {
        object.key("DigitalSignature").boolean(input.digital_signature);
    }
    if input.non_repudiation {
        object.key("NonRepudiation").boolean(input.non_repudiation);
    }
    if input.key_encipherment {
        object.key("KeyEncipherment").boolean(input.key_encipherment);
    }
    if input.data_encipherment {
        object.key("DataEncipherment").boolean(input.data_encipherment);
    }
    if input.key_agreement {
        object.key("KeyAgreement").boolean(input.key_agreement);
    }
    if input.key_cert_sign {
        object.key("KeyCertSign").boolean(input.key_cert_sign);
    }
    if input.crl_sign {
        object.key("CRLSign").boolean(input.crl_sign);
    }
    if input.encipher_only {
        object.key("EncipherOnly").boolean(input.encipher_only);
    }
    if input.decipher_only {
        object.key("DecipherOnly").boolean(input.decipher_only);
    }
    Ok(())
}

pub(crate) fn de_key_usage<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::KeyUsage>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::KeyUsageBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "DigitalSignature" => {
                            builder = builder.set_digital_signature(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "NonRepudiation" => {
                            builder = builder.set_non_repudiation(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "KeyEncipherment" => {
                            builder = builder.set_key_encipherment(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "DataEncipherment" => {
                            builder = builder.set_data_encipherment(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "KeyAgreement" => {
                            builder = builder.set_key_agreement(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "KeyCertSign" => {
                            builder = builder.set_key_cert_sign(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "CRLSign" => {
                            builder = builder.set_crl_sign(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "EncipherOnly" => {
                            builder = builder.set_encipher_only(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "DecipherOnly" => {
                            builder = builder.set_decipher_only(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
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
