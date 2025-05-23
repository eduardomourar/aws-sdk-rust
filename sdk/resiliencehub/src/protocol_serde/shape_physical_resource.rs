// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_physical_resource<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::PhysicalResource>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::PhysicalResourceBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "resourceName" => {
                            builder = builder.set_resource_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "logicalResourceId" => {
                            builder =
                                builder.set_logical_resource_id(crate::protocol_serde::shape_logical_resource_id::de_logical_resource_id(tokens)?);
                        }
                        "physicalResourceId" => {
                            builder =
                                builder.set_physical_resource_id(crate::protocol_serde::shape_physical_resource_id::de_physical_resource_id(tokens)?);
                        }
                        "resourceType" => {
                            builder = builder.set_resource_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "appComponents" => {
                            builder = builder.set_app_components(crate::protocol_serde::shape_app_component_list::de_app_component_list(tokens)?);
                        }
                        "additionalInfo" => {
                            builder = builder.set_additional_info(crate::protocol_serde::shape_additional_info_map::de_additional_info_map(tokens)?);
                        }
                        "excluded" => {
                            builder = builder.set_excluded(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "sourceType" => {
                            builder = builder.set_source_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::ResourceSourceType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "parentResourceName" => {
                            builder = builder.set_parent_resource_name(
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
            Ok(Some(crate::serde_util::physical_resource_correct_errors(builder).build().map_err(
                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
            )?))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
