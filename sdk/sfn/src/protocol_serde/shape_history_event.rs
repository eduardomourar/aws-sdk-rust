// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_history_event<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::HistoryEvent>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::HistoryEventBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "timestamp" => {
                            builder = builder.set_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "type" => {
                            builder = builder.set_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::HistoryEventType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "id" => {
                            builder = builder.set_id(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i64::try_from)
                                    .transpose()?,
                            );
                        }
                        "previousEventId" => {
                            builder = builder.set_previous_event_id(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i64::try_from)
                                    .transpose()?,
                            );
                        }
                        "activityFailedEventDetails" => {
                            builder = builder.set_activity_failed_event_details(
                                crate::protocol_serde::shape_activity_failed_event_details::de_activity_failed_event_details(tokens)?,
                            );
                        }
                        "activityScheduleFailedEventDetails" => {
                            builder = builder.set_activity_schedule_failed_event_details(
                                crate::protocol_serde::shape_activity_schedule_failed_event_details::de_activity_schedule_failed_event_details(
                                    tokens,
                                )?,
                            );
                        }
                        "activityScheduledEventDetails" => {
                            builder = builder.set_activity_scheduled_event_details(
                                crate::protocol_serde::shape_activity_scheduled_event_details::de_activity_scheduled_event_details(tokens)?,
                            );
                        }
                        "activityStartedEventDetails" => {
                            builder = builder.set_activity_started_event_details(
                                crate::protocol_serde::shape_activity_started_event_details::de_activity_started_event_details(tokens)?,
                            );
                        }
                        "activitySucceededEventDetails" => {
                            builder = builder.set_activity_succeeded_event_details(
                                crate::protocol_serde::shape_activity_succeeded_event_details::de_activity_succeeded_event_details(tokens)?,
                            );
                        }
                        "activityTimedOutEventDetails" => {
                            builder = builder.set_activity_timed_out_event_details(
                                crate::protocol_serde::shape_activity_timed_out_event_details::de_activity_timed_out_event_details(tokens)?,
                            );
                        }
                        "taskFailedEventDetails" => {
                            builder = builder.set_task_failed_event_details(
                                crate::protocol_serde::shape_task_failed_event_details::de_task_failed_event_details(tokens)?,
                            );
                        }
                        "taskScheduledEventDetails" => {
                            builder = builder.set_task_scheduled_event_details(
                                crate::protocol_serde::shape_task_scheduled_event_details::de_task_scheduled_event_details(tokens)?,
                            );
                        }
                        "taskStartFailedEventDetails" => {
                            builder = builder.set_task_start_failed_event_details(
                                crate::protocol_serde::shape_task_start_failed_event_details::de_task_start_failed_event_details(tokens)?,
                            );
                        }
                        "taskStartedEventDetails" => {
                            builder = builder.set_task_started_event_details(
                                crate::protocol_serde::shape_task_started_event_details::de_task_started_event_details(tokens)?,
                            );
                        }
                        "taskSubmitFailedEventDetails" => {
                            builder = builder.set_task_submit_failed_event_details(
                                crate::protocol_serde::shape_task_submit_failed_event_details::de_task_submit_failed_event_details(tokens)?,
                            );
                        }
                        "taskSubmittedEventDetails" => {
                            builder = builder.set_task_submitted_event_details(
                                crate::protocol_serde::shape_task_submitted_event_details::de_task_submitted_event_details(tokens)?,
                            );
                        }
                        "taskSucceededEventDetails" => {
                            builder = builder.set_task_succeeded_event_details(
                                crate::protocol_serde::shape_task_succeeded_event_details::de_task_succeeded_event_details(tokens)?,
                            );
                        }
                        "taskTimedOutEventDetails" => {
                            builder = builder.set_task_timed_out_event_details(
                                crate::protocol_serde::shape_task_timed_out_event_details::de_task_timed_out_event_details(tokens)?,
                            );
                        }
                        "executionFailedEventDetails" => {
                            builder = builder.set_execution_failed_event_details(
                                crate::protocol_serde::shape_execution_failed_event_details::de_execution_failed_event_details(tokens)?,
                            );
                        }
                        "executionStartedEventDetails" => {
                            builder = builder.set_execution_started_event_details(
                                crate::protocol_serde::shape_execution_started_event_details::de_execution_started_event_details(tokens)?,
                            );
                        }
                        "executionSucceededEventDetails" => {
                            builder = builder.set_execution_succeeded_event_details(
                                crate::protocol_serde::shape_execution_succeeded_event_details::de_execution_succeeded_event_details(tokens)?,
                            );
                        }
                        "executionAbortedEventDetails" => {
                            builder = builder.set_execution_aborted_event_details(
                                crate::protocol_serde::shape_execution_aborted_event_details::de_execution_aborted_event_details(tokens)?,
                            );
                        }
                        "executionTimedOutEventDetails" => {
                            builder = builder.set_execution_timed_out_event_details(
                                crate::protocol_serde::shape_execution_timed_out_event_details::de_execution_timed_out_event_details(tokens)?,
                            );
                        }
                        "executionRedrivenEventDetails" => {
                            builder = builder.set_execution_redriven_event_details(
                                crate::protocol_serde::shape_execution_redriven_event_details::de_execution_redriven_event_details(tokens)?,
                            );
                        }
                        "mapStateStartedEventDetails" => {
                            builder = builder.set_map_state_started_event_details(
                                crate::protocol_serde::shape_map_state_started_event_details::de_map_state_started_event_details(tokens)?,
                            );
                        }
                        "mapIterationStartedEventDetails" => {
                            builder = builder.set_map_iteration_started_event_details(
                                crate::protocol_serde::shape_map_iteration_event_details::de_map_iteration_event_details(tokens)?,
                            );
                        }
                        "mapIterationSucceededEventDetails" => {
                            builder = builder.set_map_iteration_succeeded_event_details(
                                crate::protocol_serde::shape_map_iteration_event_details::de_map_iteration_event_details(tokens)?,
                            );
                        }
                        "mapIterationFailedEventDetails" => {
                            builder = builder.set_map_iteration_failed_event_details(
                                crate::protocol_serde::shape_map_iteration_event_details::de_map_iteration_event_details(tokens)?,
                            );
                        }
                        "mapIterationAbortedEventDetails" => {
                            builder = builder.set_map_iteration_aborted_event_details(
                                crate::protocol_serde::shape_map_iteration_event_details::de_map_iteration_event_details(tokens)?,
                            );
                        }
                        "lambdaFunctionFailedEventDetails" => {
                            builder = builder.set_lambda_function_failed_event_details(
                                crate::protocol_serde::shape_lambda_function_failed_event_details::de_lambda_function_failed_event_details(tokens)?,
                            );
                        }
                        "lambdaFunctionScheduleFailedEventDetails" => {
                            builder = builder.set_lambda_function_schedule_failed_event_details(
                                    crate::protocol_serde::shape_lambda_function_schedule_failed_event_details::de_lambda_function_schedule_failed_event_details(tokens)?
                                );
                        }
                        "lambdaFunctionScheduledEventDetails" => {
                            builder = builder.set_lambda_function_scheduled_event_details(
                                crate::protocol_serde::shape_lambda_function_scheduled_event_details::de_lambda_function_scheduled_event_details(
                                    tokens,
                                )?,
                            );
                        }
                        "lambdaFunctionStartFailedEventDetails" => {
                            builder = builder.set_lambda_function_start_failed_event_details(
                                    crate::protocol_serde::shape_lambda_function_start_failed_event_details::de_lambda_function_start_failed_event_details(tokens)?
                                );
                        }
                        "lambdaFunctionSucceededEventDetails" => {
                            builder = builder.set_lambda_function_succeeded_event_details(
                                crate::protocol_serde::shape_lambda_function_succeeded_event_details::de_lambda_function_succeeded_event_details(
                                    tokens,
                                )?,
                            );
                        }
                        "lambdaFunctionTimedOutEventDetails" => {
                            builder = builder.set_lambda_function_timed_out_event_details(
                                crate::protocol_serde::shape_lambda_function_timed_out_event_details::de_lambda_function_timed_out_event_details(
                                    tokens,
                                )?,
                            );
                        }
                        "stateEnteredEventDetails" => {
                            builder = builder.set_state_entered_event_details(
                                crate::protocol_serde::shape_state_entered_event_details::de_state_entered_event_details(tokens)?,
                            );
                        }
                        "stateExitedEventDetails" => {
                            builder = builder.set_state_exited_event_details(
                                crate::protocol_serde::shape_state_exited_event_details::de_state_exited_event_details(tokens)?,
                            );
                        }
                        "mapRunStartedEventDetails" => {
                            builder = builder.set_map_run_started_event_details(
                                crate::protocol_serde::shape_map_run_started_event_details::de_map_run_started_event_details(tokens)?,
                            );
                        }
                        "mapRunFailedEventDetails" => {
                            builder = builder.set_map_run_failed_event_details(
                                crate::protocol_serde::shape_map_run_failed_event_details::de_map_run_failed_event_details(tokens)?,
                            );
                        }
                        "mapRunRedrivenEventDetails" => {
                            builder = builder.set_map_run_redriven_event_details(
                                crate::protocol_serde::shape_map_run_redriven_event_details::de_map_run_redriven_event_details(tokens)?,
                            );
                        }
                        "evaluationFailedEventDetails" => {
                            builder = builder.set_evaluation_failed_event_details(
                                crate::protocol_serde::shape_evaluation_failed_event_details::de_evaluation_failed_event_details(tokens)?,
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
            Ok(Some(crate::serde_util::history_event_correct_errors(builder).build().map_err(|err| {
                ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err)
            })?))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
