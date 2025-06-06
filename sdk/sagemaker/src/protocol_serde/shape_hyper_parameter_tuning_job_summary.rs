// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_hyper_parameter_tuning_job_summary<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::HyperParameterTuningJobSummary>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::HyperParameterTuningJobSummaryBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "HyperParameterTuningJobName" => {
                            builder = builder.set_hyper_parameter_tuning_job_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "HyperParameterTuningJobArn" => {
                            builder = builder.set_hyper_parameter_tuning_job_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "HyperParameterTuningJobStatus" => {
                            builder = builder.set_hyper_parameter_tuning_job_status(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::HyperParameterTuningJobStatus::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "Strategy" => {
                            builder = builder.set_strategy(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| {
                                        s.to_unescaped()
                                            .map(|u| crate::types::HyperParameterTuningJobStrategyType::from(u.as_ref()))
                                    })
                                    .transpose()?,
                            );
                        }
                        "CreationTime" => {
                            builder = builder.set_creation_time(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "HyperParameterTuningEndTime" => {
                            builder = builder.set_hyper_parameter_tuning_end_time(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "LastModifiedTime" => {
                            builder = builder.set_last_modified_time(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "TrainingJobStatusCounters" => {
                            builder = builder.set_training_job_status_counters(
                                crate::protocol_serde::shape_training_job_status_counters::de_training_job_status_counters(tokens)?,
                            );
                        }
                        "ObjectiveStatusCounters" => {
                            builder = builder.set_objective_status_counters(
                                crate::protocol_serde::shape_objective_status_counters::de_objective_status_counters(tokens)?,
                            );
                        }
                        "ResourceLimits" => {
                            builder = builder.set_resource_limits(crate::protocol_serde::shape_resource_limits::de_resource_limits(tokens)?);
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
            Ok(Some(
                crate::serde_util::hyper_parameter_tuning_job_summary_correct_errors(builder).build(),
            ))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
