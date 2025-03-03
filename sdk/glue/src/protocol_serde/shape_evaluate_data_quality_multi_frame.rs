// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_evaluate_data_quality_multi_frame(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::EvaluateDataQualityMultiFrame,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("Name").string(input.name.as_str());
    }
    {
        let mut array_1 = object.key("Inputs").start_array();
        for item_2 in &input.inputs {
            {
                array_1.value().string(item_2.as_str());
            }
        }
        array_1.finish();
    }
    if let Some(var_3) = &input.additional_data_sources {
        #[allow(unused_mut)]
        let mut object_4 = object.key("AdditionalDataSources").start_object();
        for (key_5, value_6) in var_3 {
            {
                object_4.key(key_5.as_str()).string(value_6.as_str());
            }
        }
        object_4.finish();
    }
    {
        object.key("Ruleset").string(input.ruleset.as_str());
    }
    if let Some(var_7) = &input.publishing_options {
        #[allow(unused_mut)]
        let mut object_8 = object.key("PublishingOptions").start_object();
        crate::protocol_serde::shape_dq_results_publishing_options::ser_dq_results_publishing_options(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.additional_options {
        #[allow(unused_mut)]
        let mut object_10 = object.key("AdditionalOptions").start_object();
        for (key_11, value_12) in var_9 {
            {
                object_10.key(key_11.as_str()).string(value_12.as_str());
            }
        }
        object_10.finish();
    }
    if let Some(var_13) = &input.stop_job_on_failure_options {
        #[allow(unused_mut)]
        let mut object_14 = object.key("StopJobOnFailureOptions").start_object();
        crate::protocol_serde::shape_dq_stop_job_on_failure_options::ser_dq_stop_job_on_failure_options(&mut object_14, var_13)?;
        object_14.finish();
    }
    Ok(())
}

pub(crate) fn de_evaluate_data_quality_multi_frame<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::EvaluateDataQualityMultiFrame>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::EvaluateDataQualityMultiFrameBuilder::default();
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
                        "Inputs" => {
                            builder = builder.set_inputs(crate::protocol_serde::shape_many_inputs::de_many_inputs(tokens)?);
                        }
                        "AdditionalDataSources" => {
                            builder = builder.set_additional_data_sources(crate::protocol_serde::shape_dqdl_aliases::de_dqdl_aliases(tokens)?);
                        }
                        "Ruleset" => {
                            builder = builder.set_ruleset(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "PublishingOptions" => {
                            builder = builder.set_publishing_options(
                                crate::protocol_serde::shape_dq_results_publishing_options::de_dq_results_publishing_options(tokens)?,
                            );
                        }
                        "AdditionalOptions" => {
                            builder =
                                builder.set_additional_options(crate::protocol_serde::shape_dq_additional_options::de_dq_additional_options(tokens)?);
                        }
                        "StopJobOnFailureOptions" => {
                            builder = builder.set_stop_job_on_failure_options(
                                crate::protocol_serde::shape_dq_stop_job_on_failure_options::de_dq_stop_job_on_failure_options(tokens)?,
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
                crate::serde_util::evaluate_data_quality_multi_frame_correct_errors(builder)
                    .build()
                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
            ))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
