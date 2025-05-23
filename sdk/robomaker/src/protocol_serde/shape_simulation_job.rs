// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_simulation_job<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::SimulationJob>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::SimulationJobBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "arn" => {
                            builder = builder.set_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "name" => {
                            builder = builder.set_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "status" => {
                            builder = builder.set_status(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::SimulationJobStatus::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "lastStartedAt" => {
                            builder = builder.set_last_started_at(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "lastUpdatedAt" => {
                            builder = builder.set_last_updated_at(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "failureBehavior" => {
                            builder = builder.set_failure_behavior(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::FailureBehavior::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "failureCode" => {
                            builder = builder.set_failure_code(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::SimulationJobErrorCode::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "failureReason" => {
                            builder = builder.set_failure_reason(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "clientRequestToken" => {
                            builder = builder.set_client_request_token(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "outputLocation" => {
                            builder = builder.set_output_location(crate::protocol_serde::shape_output_location::de_output_location(tokens)?);
                        }
                        "loggingConfig" => {
                            builder = builder.set_logging_config(crate::protocol_serde::shape_logging_config::de_logging_config(tokens)?);
                        }
                        "maxJobDurationInSeconds" => {
                            builder = builder.set_max_job_duration_in_seconds(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i64::try_from)
                                    .transpose()?,
                            );
                        }
                        "simulationTimeMillis" => {
                            builder = builder.set_simulation_time_millis(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i64::try_from)
                                    .transpose()?,
                            );
                        }
                        "iamRole" => {
                            builder = builder.set_iam_role(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "robotApplications" => {
                            builder = builder.set_robot_applications(
                                crate::protocol_serde::shape_robot_application_configs::de_robot_application_configs(tokens)?,
                            );
                        }
                        "simulationApplications" => {
                            builder = builder.set_simulation_applications(
                                crate::protocol_serde::shape_simulation_application_configs::de_simulation_application_configs(tokens)?,
                            );
                        }
                        "dataSources" => {
                            builder = builder.set_data_sources(crate::protocol_serde::shape_data_sources::de_data_sources(tokens)?);
                        }
                        "tags" => {
                            builder = builder.set_tags(crate::protocol_serde::shape_tag_map::de_tag_map(tokens)?);
                        }
                        "vpcConfig" => {
                            builder = builder.set_vpc_config(crate::protocol_serde::shape_vpc_config_response::de_vpc_config_response(tokens)?);
                        }
                        "networkInterface" => {
                            builder = builder.set_network_interface(crate::protocol_serde::shape_network_interface::de_network_interface(tokens)?);
                        }
                        "compute" => {
                            builder = builder.set_compute(crate::protocol_serde::shape_compute_response::de_compute_response(tokens)?);
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
