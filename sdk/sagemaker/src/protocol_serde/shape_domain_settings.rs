// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_domain_settings(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::DomainSettings,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.security_group_ids {
        let mut array_2 = object.key("SecurityGroupIds").start_array();
        for item_3 in var_1 {
            {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.r_studio_server_pro_domain_settings {
        #[allow(unused_mut)]
        let mut object_5 = object.key("RStudioServerProDomainSettings").start_object();
        crate::protocol_serde::shape_r_studio_server_pro_domain_settings::ser_r_studio_server_pro_domain_settings(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.execution_role_identity_config {
        object.key("ExecutionRoleIdentityConfig").string(var_6.as_str());
    }
    if let Some(var_7) = &input.docker_settings {
        #[allow(unused_mut)]
        let mut object_8 = object.key("DockerSettings").start_object();
        crate::protocol_serde::shape_docker_settings::ser_docker_settings(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.amazon_q_settings {
        #[allow(unused_mut)]
        let mut object_10 = object.key("AmazonQSettings").start_object();
        crate::protocol_serde::shape_amazon_q_settings::ser_amazon_q_settings(&mut object_10, var_9)?;
        object_10.finish();
    }
    if let Some(var_11) = &input.unified_studio_settings {
        #[allow(unused_mut)]
        let mut object_12 = object.key("UnifiedStudioSettings").start_object();
        crate::protocol_serde::shape_unified_studio_settings::ser_unified_studio_settings(&mut object_12, var_11)?;
        object_12.finish();
    }
    Ok(())
}

pub(crate) fn de_domain_settings<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::DomainSettings>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::DomainSettingsBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "SecurityGroupIds" => {
                            builder = builder.set_security_group_ids(
                                crate::protocol_serde::shape_domain_security_group_ids::de_domain_security_group_ids(tokens)?,
                            );
                        }
                        "RStudioServerProDomainSettings" => {
                            builder = builder.set_r_studio_server_pro_domain_settings(
                                crate::protocol_serde::shape_r_studio_server_pro_domain_settings::de_r_studio_server_pro_domain_settings(tokens)?,
                            );
                        }
                        "ExecutionRoleIdentityConfig" => {
                            builder = builder.set_execution_role_identity_config(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::ExecutionRoleIdentityConfig::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "DockerSettings" => {
                            builder = builder.set_docker_settings(crate::protocol_serde::shape_docker_settings::de_docker_settings(tokens)?);
                        }
                        "AmazonQSettings" => {
                            builder = builder.set_amazon_q_settings(crate::protocol_serde::shape_amazon_q_settings::de_amazon_q_settings(tokens)?);
                        }
                        "UnifiedStudioSettings" => {
                            builder = builder.set_unified_studio_settings(
                                crate::protocol_serde::shape_unified_studio_settings::de_unified_studio_settings(tokens)?,
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
