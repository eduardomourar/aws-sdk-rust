// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_snowflake_parameters(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::SnowflakeParameters,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("Host").string(input.host.as_str());
    }
    {
        object.key("Database").string(input.database.as_str());
    }
    {
        object.key("Warehouse").string(input.warehouse.as_str());
    }
    if let Some(var_1) = &input.authentication_type {
        object.key("AuthenticationType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.database_access_control_role {
        object.key("DatabaseAccessControlRole").string(var_2.as_str());
    }
    if let Some(var_3) = &input.o_auth_parameters {
        #[allow(unused_mut)]
        let mut object_4 = object.key("OAuthParameters").start_object();
        crate::protocol_serde::shape_o_auth_parameters::ser_o_auth_parameters(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

pub(crate) fn de_snowflake_parameters<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::SnowflakeParameters>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::SnowflakeParametersBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "Host" => {
                            builder = builder.set_host(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Database" => {
                            builder = builder.set_database(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Warehouse" => {
                            builder = builder.set_warehouse(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "AuthenticationType" => {
                            builder = builder.set_authentication_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::AuthenticationType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "DatabaseAccessControlRole" => {
                            builder = builder.set_database_access_control_role(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "OAuthParameters" => {
                            builder = builder.set_o_auth_parameters(crate::protocol_serde::shape_o_auth_parameters::de_o_auth_parameters(tokens)?);
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
            Ok(Some(crate::serde_util::snowflake_parameters_correct_errors(builder).build().map_err(
                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
            )?))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
