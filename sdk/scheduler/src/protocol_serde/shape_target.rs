// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_target(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::Target,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("Arn").string(input.arn.as_str());
    }
    {
        object.key("RoleArn").string(input.role_arn.as_str());
    }
    if let Some(var_1) = &input.dead_letter_config {
        #[allow(unused_mut)]
        let mut object_2 = object.key("DeadLetterConfig").start_object();
        crate::protocol_serde::shape_dead_letter_config::ser_dead_letter_config(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.retry_policy {
        #[allow(unused_mut)]
        let mut object_4 = object.key("RetryPolicy").start_object();
        crate::protocol_serde::shape_retry_policy::ser_retry_policy(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.input {
        object.key("Input").string(var_5.as_str());
    }
    if let Some(var_6) = &input.ecs_parameters {
        #[allow(unused_mut)]
        let mut object_7 = object.key("EcsParameters").start_object();
        crate::protocol_serde::shape_ecs_parameters::ser_ecs_parameters(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.event_bridge_parameters {
        #[allow(unused_mut)]
        let mut object_9 = object.key("EventBridgeParameters").start_object();
        crate::protocol_serde::shape_event_bridge_parameters::ser_event_bridge_parameters(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.kinesis_parameters {
        #[allow(unused_mut)]
        let mut object_11 = object.key("KinesisParameters").start_object();
        crate::protocol_serde::shape_kinesis_parameters::ser_kinesis_parameters(&mut object_11, var_10)?;
        object_11.finish();
    }
    if let Some(var_12) = &input.sage_maker_pipeline_parameters {
        #[allow(unused_mut)]
        let mut object_13 = object.key("SageMakerPipelineParameters").start_object();
        crate::protocol_serde::shape_sage_maker_pipeline_parameters::ser_sage_maker_pipeline_parameters(&mut object_13, var_12)?;
        object_13.finish();
    }
    if let Some(var_14) = &input.sqs_parameters {
        #[allow(unused_mut)]
        let mut object_15 = object.key("SqsParameters").start_object();
        crate::protocol_serde::shape_sqs_parameters::ser_sqs_parameters(&mut object_15, var_14)?;
        object_15.finish();
    }
    Ok(())
}

pub(crate) fn de_target<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::Target>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::TargetBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "Arn" => {
                            builder = builder.set_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "RoleArn" => {
                            builder = builder.set_role_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "DeadLetterConfig" => {
                            builder = builder.set_dead_letter_config(crate::protocol_serde::shape_dead_letter_config::de_dead_letter_config(tokens)?);
                        }
                        "RetryPolicy" => {
                            builder = builder.set_retry_policy(crate::protocol_serde::shape_retry_policy::de_retry_policy(tokens)?);
                        }
                        "Input" => {
                            builder = builder.set_input(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "EcsParameters" => {
                            builder = builder.set_ecs_parameters(crate::protocol_serde::shape_ecs_parameters::de_ecs_parameters(tokens)?);
                        }
                        "EventBridgeParameters" => {
                            builder = builder.set_event_bridge_parameters(
                                crate::protocol_serde::shape_event_bridge_parameters::de_event_bridge_parameters(tokens)?,
                            );
                        }
                        "KinesisParameters" => {
                            builder = builder.set_kinesis_parameters(crate::protocol_serde::shape_kinesis_parameters::de_kinesis_parameters(tokens)?);
                        }
                        "SageMakerPipelineParameters" => {
                            builder = builder.set_sage_maker_pipeline_parameters(
                                crate::protocol_serde::shape_sage_maker_pipeline_parameters::de_sage_maker_pipeline_parameters(tokens)?,
                            );
                        }
                        "SqsParameters" => {
                            builder = builder.set_sqs_parameters(crate::protocol_serde::shape_sqs_parameters::de_sqs_parameters(tokens)?);
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
            Ok(Some(crate::serde_util::target_correct_errors(builder).build().map_err(|err| {
                ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err)
            })?))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
