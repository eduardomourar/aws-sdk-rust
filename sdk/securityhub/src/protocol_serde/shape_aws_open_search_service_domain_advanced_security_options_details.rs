// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aws_open_search_service_domain_advanced_security_options_details(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AwsOpenSearchServiceDomainAdvancedSecurityOptionsDetails,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.enabled {
        object.key("Enabled").boolean(*var_1);
    }
    if let Some(var_2) = &input.internal_user_database_enabled {
        object.key("InternalUserDatabaseEnabled").boolean(*var_2);
    }
    if let Some(var_3) = &input.master_user_options {
        #[allow(unused_mut)]
        let mut object_4 = object.key("MasterUserOptions").start_object();
        crate::protocol_serde::shape_aws_open_search_service_domain_master_user_options_details::ser_aws_open_search_service_domain_master_user_options_details(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

pub(crate) fn de_aws_open_search_service_domain_advanced_security_options_details<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<
    Option<crate::types::AwsOpenSearchServiceDomainAdvancedSecurityOptionsDetails>,
    ::aws_smithy_json::deserialize::error::DeserializeError,
>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::AwsOpenSearchServiceDomainAdvancedSecurityOptionsDetailsBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "Enabled" => {
                            builder = builder.set_enabled(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "InternalUserDatabaseEnabled" => {
                            builder = builder
                                .set_internal_user_database_enabled(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "MasterUserOptions" => {
                            builder = builder.set_master_user_options(
                                    crate::protocol_serde::shape_aws_open_search_service_domain_master_user_options_details::de_aws_open_search_service_domain_master_user_options_details(tokens)?
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
