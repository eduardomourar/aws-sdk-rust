// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_association_description<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::AssociationDescription>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::AssociationDescriptionBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "Name" => {
                            builder = builder.set_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "InstanceId" => {
                            builder = builder.set_instance_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "AssociationVersion" => {
                            builder = builder.set_association_version(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Date" => {
                            builder = builder.set_date(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "LastUpdateAssociationDate" => {
                            builder = builder.set_last_update_association_date(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "Status" => {
                            builder = builder.set_status(crate::protocol_serde::shape_association_status::de_association_status(tokens)?);
                        }
                        "Overview" => {
                            builder = builder.set_overview(crate::protocol_serde::shape_association_overview::de_association_overview(tokens)?);
                        }
                        "DocumentVersion" => {
                            builder = builder.set_document_version(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "AutomationTargetParameterName" => {
                            builder = builder.set_automation_target_parameter_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Parameters" => {
                            builder = builder.set_parameters(crate::protocol_serde::shape_parameters::de_parameters(tokens)?);
                        }
                        "AssociationId" => {
                            builder = builder.set_association_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Targets" => {
                            builder = builder.set_targets(crate::protocol_serde::shape_targets::de_targets(tokens)?);
                        }
                        "ScheduleExpression" => {
                            builder = builder.set_schedule_expression(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "OutputLocation" => {
                            builder = builder.set_output_location(
                                crate::protocol_serde::shape_instance_association_output_location::de_instance_association_output_location(tokens)?,
                            );
                        }
                        "LastExecutionDate" => {
                            builder = builder.set_last_execution_date(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "LastSuccessfulExecutionDate" => {
                            builder = builder.set_last_successful_execution_date(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "AssociationName" => {
                            builder = builder.set_association_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "MaxErrors" => {
                            builder = builder.set_max_errors(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "MaxConcurrency" => {
                            builder = builder.set_max_concurrency(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "ComplianceSeverity" => {
                            builder = builder.set_compliance_severity(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::AssociationComplianceSeverity::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "SyncCompliance" => {
                            builder = builder.set_sync_compliance(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::AssociationSyncCompliance::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "ApplyOnlyAtCronInterval" => {
                            builder =
                                builder.set_apply_only_at_cron_interval(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "CalendarNames" => {
                            builder = builder.set_calendar_names(
                                crate::protocol_serde::shape_calendar_name_or_arn_list::de_calendar_name_or_arn_list(tokens)?,
                            );
                        }
                        "TargetLocations" => {
                            builder = builder.set_target_locations(crate::protocol_serde::shape_target_locations::de_target_locations(tokens)?);
                        }
                        "ScheduleOffset" => {
                            builder = builder.set_schedule_offset(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "Duration" => {
                            builder = builder.set_duration(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "TargetMaps" => {
                            builder = builder.set_target_maps(crate::protocol_serde::shape_target_maps::de_target_maps(tokens)?);
                        }
                        "AlarmConfiguration" => {
                            builder =
                                builder.set_alarm_configuration(crate::protocol_serde::shape_alarm_configuration::de_alarm_configuration(tokens)?);
                        }
                        "TriggeredAlarms" => {
                            builder = builder.set_triggered_alarms(
                                crate::protocol_serde::shape_alarm_state_information_list::de_alarm_state_information_list(tokens)?,
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
