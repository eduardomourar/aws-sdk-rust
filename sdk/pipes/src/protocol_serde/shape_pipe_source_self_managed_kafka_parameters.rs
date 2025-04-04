// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_pipe_source_self_managed_kafka_parameters(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::PipeSourceSelfManagedKafkaParameters,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("TopicName").string(input.topic_name.as_str());
    }
    if let Some(var_1) = &input.starting_position {
        object.key("StartingPosition").string(var_1.as_str());
    }
    if let Some(var_2) = &input.additional_bootstrap_servers {
        let mut array_3 = object.key("AdditionalBootstrapServers").start_array();
        for item_4 in var_2 {
            {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    if let Some(var_5) = &input.batch_size {
        object.key("BatchSize").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_5).into()),
        );
    }
    if let Some(var_6) = &input.maximum_batching_window_in_seconds {
        object.key("MaximumBatchingWindowInSeconds").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_6).into()),
        );
    }
    if let Some(var_7) = &input.consumer_group_id {
        object.key("ConsumerGroupID").string(var_7.as_str());
    }
    if let Some(var_8) = &input.credentials {
        #[allow(unused_mut)]
        let mut object_9 = object.key("Credentials").start_object();
        crate::protocol_serde::shape_self_managed_kafka_access_configuration_credentials::ser_self_managed_kafka_access_configuration_credentials(
            &mut object_9,
            var_8,
        )?;
        object_9.finish();
    }
    if let Some(var_10) = &input.server_root_ca_certificate {
        object.key("ServerRootCaCertificate").string(var_10.as_str());
    }
    if let Some(var_11) = &input.vpc {
        #[allow(unused_mut)]
        let mut object_12 = object.key("Vpc").start_object();
        crate::protocol_serde::shape_self_managed_kafka_access_configuration_vpc::ser_self_managed_kafka_access_configuration_vpc(
            &mut object_12,
            var_11,
        )?;
        object_12.finish();
    }
    Ok(())
}

pub(crate) fn de_pipe_source_self_managed_kafka_parameters<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::PipeSourceSelfManagedKafkaParameters>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::PipeSourceSelfManagedKafkaParametersBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "TopicName" => {
                            builder = builder.set_topic_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "StartingPosition" => {
                            builder = builder.set_starting_position(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::SelfManagedKafkaStartPosition::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "AdditionalBootstrapServers" => {
                            builder = builder.set_additional_bootstrap_servers(
                                crate::protocol_serde::shape_kafka_bootstrap_servers::de_kafka_bootstrap_servers(tokens)?,
                            );
                        }
                        "BatchSize" => {
                            builder = builder.set_batch_size(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "MaximumBatchingWindowInSeconds" => {
                            builder = builder.set_maximum_batching_window_in_seconds(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "ConsumerGroupID" => {
                            builder = builder.set_consumer_group_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Credentials" => {
                            builder = builder.set_credentials(
                                    crate::protocol_serde::shape_self_managed_kafka_access_configuration_credentials::de_self_managed_kafka_access_configuration_credentials(tokens)?
                                );
                        }
                        "ServerRootCaCertificate" => {
                            builder = builder.set_server_root_ca_certificate(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Vpc" => {
                            builder = builder.set_vpc(
                                    crate::protocol_serde::shape_self_managed_kafka_access_configuration_vpc::de_self_managed_kafka_access_configuration_vpc(tokens)?
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
            Ok(Some(
                crate::serde_util::pipe_source_self_managed_kafka_parameters_correct_errors(builder)
                    .build()
                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
            ))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
