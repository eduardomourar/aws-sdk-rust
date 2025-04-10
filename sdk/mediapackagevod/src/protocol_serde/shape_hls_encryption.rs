// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_hls_encryption<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::HlsEncryption>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::HlsEncryptionBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "constantInitializationVector" => {
                            builder = builder.set_constant_initialization_vector(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "encryptionMethod" => {
                            builder = builder.set_encryption_method(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::EncryptionMethod::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "spekeKeyProvider" => {
                            builder = builder.set_speke_key_provider(crate::protocol_serde::shape_speke_key_provider::de_speke_key_provider(tokens)?);
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
            Ok(Some(crate::serde_util::hls_encryption_correct_errors(builder).build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}

pub fn ser_hls_encryption(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::HlsEncryption,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.constant_initialization_vector {
        object.key("constantInitializationVector").string(var_1.as_str());
    }
    if let Some(var_2) = &input.encryption_method {
        object.key("encryptionMethod").string(var_2.as_str());
    }
    if let Some(var_3) = &input.speke_key_provider {
        #[allow(unused_mut)]
        let mut object_4 = object.key("spekeKeyProvider").start_object();
        crate::protocol_serde::shape_speke_key_provider::ser_speke_key_provider(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}
