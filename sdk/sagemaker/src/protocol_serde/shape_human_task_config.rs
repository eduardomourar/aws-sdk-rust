// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_human_task_config(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::HumanTaskConfig,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.workteam_arn {
        object.key("WorkteamArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.ui_config {
        #[allow(unused_mut)]
        let mut object_3 = object.key("UiConfig").start_object();
        crate::protocol_serde::shape_ui_config::ser_ui_config(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.pre_human_task_lambda_arn {
        object.key("PreHumanTaskLambdaArn").string(var_4.as_str());
    }
    if let Some(var_5) = &input.task_keywords {
        let mut array_6 = object.key("TaskKeywords").start_array();
        for item_7 in var_5 {
            {
                array_6.value().string(item_7.as_str());
            }
        }
        array_6.finish();
    }
    if let Some(var_8) = &input.task_title {
        object.key("TaskTitle").string(var_8.as_str());
    }
    if let Some(var_9) = &input.task_description {
        object.key("TaskDescription").string(var_9.as_str());
    }
    if let Some(var_10) = &input.number_of_human_workers_per_data_object {
        object.key("NumberOfHumanWorkersPerDataObject").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_10).into()),
        );
    }
    if let Some(var_11) = &input.task_time_limit_in_seconds {
        object.key("TaskTimeLimitInSeconds").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_11).into()),
        );
    }
    if let Some(var_12) = &input.task_availability_lifetime_in_seconds {
        object.key("TaskAvailabilityLifetimeInSeconds").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_12).into()),
        );
    }
    if let Some(var_13) = &input.max_concurrent_task_count {
        object.key("MaxConcurrentTaskCount").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_13).into()),
        );
    }
    if let Some(var_14) = &input.annotation_consolidation_config {
        #[allow(unused_mut)]
        let mut object_15 = object.key("AnnotationConsolidationConfig").start_object();
        crate::protocol_serde::shape_annotation_consolidation_config::ser_annotation_consolidation_config(&mut object_15, var_14)?;
        object_15.finish();
    }
    if let Some(var_16) = &input.public_workforce_task_price {
        #[allow(unused_mut)]
        let mut object_17 = object.key("PublicWorkforceTaskPrice").start_object();
        crate::protocol_serde::shape_public_workforce_task_price::ser_public_workforce_task_price(&mut object_17, var_16)?;
        object_17.finish();
    }
    Ok(())
}

pub(crate) fn de_human_task_config<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::HumanTaskConfig>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::HumanTaskConfigBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "WorkteamArn" => {
                            builder = builder.set_workteam_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "UiConfig" => {
                            builder = builder.set_ui_config(crate::protocol_serde::shape_ui_config::de_ui_config(tokens)?);
                        }
                        "PreHumanTaskLambdaArn" => {
                            builder = builder.set_pre_human_task_lambda_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "TaskKeywords" => {
                            builder = builder.set_task_keywords(crate::protocol_serde::shape_task_keywords::de_task_keywords(tokens)?);
                        }
                        "TaskTitle" => {
                            builder = builder.set_task_title(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "TaskDescription" => {
                            builder = builder.set_task_description(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "NumberOfHumanWorkersPerDataObject" => {
                            builder = builder.set_number_of_human_workers_per_data_object(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "TaskTimeLimitInSeconds" => {
                            builder = builder.set_task_time_limit_in_seconds(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "TaskAvailabilityLifetimeInSeconds" => {
                            builder = builder.set_task_availability_lifetime_in_seconds(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "MaxConcurrentTaskCount" => {
                            builder = builder.set_max_concurrent_task_count(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "AnnotationConsolidationConfig" => {
                            builder = builder.set_annotation_consolidation_config(
                                crate::protocol_serde::shape_annotation_consolidation_config::de_annotation_consolidation_config(tokens)?,
                            );
                        }
                        "PublicWorkforceTaskPrice" => {
                            builder = builder.set_public_workforce_task_price(
                                crate::protocol_serde::shape_public_workforce_task_price::de_public_workforce_task_price(tokens)?,
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
            Ok(Some(crate::serde_util::human_task_config_correct_errors(builder).build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
