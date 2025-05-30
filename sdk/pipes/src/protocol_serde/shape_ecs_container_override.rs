// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_ecs_container_override(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::EcsContainerOverride,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.command {
        let mut array_2 = object.key("Command").start_array();
        for item_3 in var_1 {
            {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.cpu {
        object.key("Cpu").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    if let Some(var_5) = &input.environment {
        let mut array_6 = object.key("Environment").start_array();
        for item_7 in var_5 {
            {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_ecs_environment_variable::ser_ecs_environment_variable(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    if let Some(var_9) = &input.environment_files {
        let mut array_10 = object.key("EnvironmentFiles").start_array();
        for item_11 in var_9 {
            {
                #[allow(unused_mut)]
                let mut object_12 = array_10.value().start_object();
                crate::protocol_serde::shape_ecs_environment_file::ser_ecs_environment_file(&mut object_12, item_11)?;
                object_12.finish();
            }
        }
        array_10.finish();
    }
    if let Some(var_13) = &input.memory {
        object.key("Memory").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_13).into()),
        );
    }
    if let Some(var_14) = &input.memory_reservation {
        object.key("MemoryReservation").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_14).into()),
        );
    }
    if let Some(var_15) = &input.name {
        object.key("Name").string(var_15.as_str());
    }
    if let Some(var_16) = &input.resource_requirements {
        let mut array_17 = object.key("ResourceRequirements").start_array();
        for item_18 in var_16 {
            {
                #[allow(unused_mut)]
                let mut object_19 = array_17.value().start_object();
                crate::protocol_serde::shape_ecs_resource_requirement::ser_ecs_resource_requirement(&mut object_19, item_18)?;
                object_19.finish();
            }
        }
        array_17.finish();
    }
    Ok(())
}

pub(crate) fn de_ecs_container_override<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::EcsContainerOverride>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::EcsContainerOverrideBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "Command" => {
                            builder = builder.set_command(crate::protocol_serde::shape_string_list::de_string_list(tokens)?);
                        }
                        "Cpu" => {
                            builder = builder.set_cpu(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "Environment" => {
                            builder = builder.set_environment(
                                crate::protocol_serde::shape_ecs_environment_variable_list::de_ecs_environment_variable_list(tokens)?,
                            );
                        }
                        "EnvironmentFiles" => {
                            builder = builder.set_environment_files(
                                crate::protocol_serde::shape_ecs_environment_file_list::de_ecs_environment_file_list(tokens)?,
                            );
                        }
                        "Memory" => {
                            builder = builder.set_memory(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "MemoryReservation" => {
                            builder = builder.set_memory_reservation(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "Name" => {
                            builder = builder.set_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "ResourceRequirements" => {
                            builder = builder.set_resource_requirements(
                                crate::protocol_serde::shape_ecs_resource_requirements_list::de_ecs_resource_requirements_list(tokens)?,
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
