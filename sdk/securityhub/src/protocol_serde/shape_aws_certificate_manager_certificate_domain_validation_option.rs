// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aws_certificate_manager_certificate_domain_validation_option(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AwsCertificateManagerCertificateDomainValidationOption,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.domain_name {
        object.key("DomainName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.resource_record {
        #[allow(unused_mut)]
        let mut object_3 = object.key("ResourceRecord").start_object();
        crate::protocol_serde::shape_aws_certificate_manager_certificate_resource_record::ser_aws_certificate_manager_certificate_resource_record(
            &mut object_3,
            var_2,
        )?;
        object_3.finish();
    }
    if let Some(var_4) = &input.validation_domain {
        object.key("ValidationDomain").string(var_4.as_str());
    }
    if let Some(var_5) = &input.validation_emails {
        let mut array_6 = object.key("ValidationEmails").start_array();
        for item_7 in var_5 {
            {
                array_6.value().string(item_7.as_str());
            }
        }
        array_6.finish();
    }
    if let Some(var_8) = &input.validation_method {
        object.key("ValidationMethod").string(var_8.as_str());
    }
    if let Some(var_9) = &input.validation_status {
        object.key("ValidationStatus").string(var_9.as_str());
    }
    Ok(())
}

pub(crate) fn de_aws_certificate_manager_certificate_domain_validation_option<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<
    Option<crate::types::AwsCertificateManagerCertificateDomainValidationOption>,
    ::aws_smithy_json::deserialize::error::DeserializeError,
>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::AwsCertificateManagerCertificateDomainValidationOptionBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "DomainName" => {
                            builder = builder.set_domain_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "ResourceRecord" => {
                            builder = builder.set_resource_record(
                                    crate::protocol_serde::shape_aws_certificate_manager_certificate_resource_record::de_aws_certificate_manager_certificate_resource_record(tokens)?
                                );
                        }
                        "ValidationDomain" => {
                            builder = builder.set_validation_domain(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "ValidationEmails" => {
                            builder = builder.set_validation_emails(crate::protocol_serde::shape_string_list::de_string_list(tokens)?);
                        }
                        "ValidationMethod" => {
                            builder = builder.set_validation_method(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "ValidationStatus" => {
                            builder = builder.set_validation_status(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
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
