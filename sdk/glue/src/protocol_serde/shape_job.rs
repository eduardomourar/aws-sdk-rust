// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_job<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::Job>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::JobBuilder::default();
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
                        "JobMode" => {
                            builder = builder.set_job_mode(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::JobMode::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "JobRunQueuingEnabled" => {
                            builder = builder.set_job_run_queuing_enabled(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "Description" => {
                            builder = builder.set_description(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "LogUri" => {
                            builder = builder.set_log_uri(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Role" => {
                            builder = builder.set_role(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "CreatedOn" => {
                            builder = builder.set_created_on(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "LastModifiedOn" => {
                            builder = builder.set_last_modified_on(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "ExecutionProperty" => {
                            builder = builder.set_execution_property(crate::protocol_serde::shape_execution_property::de_execution_property(tokens)?);
                        }
                        "Command" => {
                            builder = builder.set_command(crate::protocol_serde::shape_job_command::de_job_command(tokens)?);
                        }
                        "DefaultArguments" => {
                            builder = builder.set_default_arguments(crate::protocol_serde::shape_generic_map::de_generic_map(tokens)?);
                        }
                        "NonOverridableArguments" => {
                            builder = builder.set_non_overridable_arguments(crate::protocol_serde::shape_generic_map::de_generic_map(tokens)?);
                        }
                        "Connections" => {
                            builder = builder.set_connections(crate::protocol_serde::shape_connections_list::de_connections_list(tokens)?);
                        }
                        "MaxRetries" => {
                            builder = builder.set_max_retries(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "AllocatedCapacity" => {
                            builder = builder.set_allocated_capacity(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "Timeout" => {
                            builder = builder.set_timeout(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "MaxCapacity" => {
                            builder = builder.set_max_capacity(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?.map(|v| v.to_f64_lossy()),
                            );
                        }
                        "WorkerType" => {
                            builder = builder.set_worker_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::WorkerType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "NumberOfWorkers" => {
                            builder = builder.set_number_of_workers(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "SecurityConfiguration" => {
                            builder = builder.set_security_configuration(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "NotificationProperty" => {
                            builder = builder
                                .set_notification_property(crate::protocol_serde::shape_notification_property::de_notification_property(tokens)?);
                        }
                        "GlueVersion" => {
                            builder = builder.set_glue_version(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "CodeGenConfigurationNodes" => {
                            builder = builder.set_code_gen_configuration_nodes(
                                crate::protocol_serde::shape_code_gen_configuration_nodes::de_code_gen_configuration_nodes(tokens)?,
                            );
                        }
                        "ExecutionClass" => {
                            builder = builder.set_execution_class(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::ExecutionClass::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "SourceControlDetails" => {
                            builder = builder
                                .set_source_control_details(crate::protocol_serde::shape_source_control_details::de_source_control_details(tokens)?);
                        }
                        "MaintenanceWindow" => {
                            builder = builder.set_maintenance_window(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "ProfileName" => {
                            builder = builder.set_profile_name(
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
            Ok(Some(builder.build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
