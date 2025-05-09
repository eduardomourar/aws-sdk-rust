// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_model_dashboard_monitoring_schedule<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::ModelDashboardMonitoringSchedule>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ModelDashboardMonitoringScheduleBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "MonitoringScheduleArn" => {
                            builder = builder.set_monitoring_schedule_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "MonitoringScheduleName" => {
                            builder = builder.set_monitoring_schedule_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "MonitoringScheduleStatus" => {
                            builder = builder.set_monitoring_schedule_status(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::ScheduleStatus::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "MonitoringType" => {
                            builder = builder.set_monitoring_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::MonitoringType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "FailureReason" => {
                            builder = builder.set_failure_reason(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "CreationTime" => {
                            builder = builder.set_creation_time(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
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
                        "MonitoringScheduleConfig" => {
                            builder = builder.set_monitoring_schedule_config(
                                crate::protocol_serde::shape_monitoring_schedule_config::de_monitoring_schedule_config(tokens)?,
                            );
                        }
                        "EndpointName" => {
                            builder = builder.set_endpoint_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "MonitoringAlertSummaries" => {
                            builder = builder.set_monitoring_alert_summaries(
                                crate::protocol_serde::shape_monitoring_alert_summary_list::de_monitoring_alert_summary_list(tokens)?,
                            );
                        }
                        "LastMonitoringExecutionSummary" => {
                            builder = builder.set_last_monitoring_execution_summary(
                                crate::protocol_serde::shape_monitoring_execution_summary::de_monitoring_execution_summary(tokens)?,
                            );
                        }
                        "BatchTransformInput" => {
                            builder = builder
                                .set_batch_transform_input(crate::protocol_serde::shape_batch_transform_input::de_batch_transform_input(tokens)?);
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
