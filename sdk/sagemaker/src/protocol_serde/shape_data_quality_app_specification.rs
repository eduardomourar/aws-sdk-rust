// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_data_quality_app_specification(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::DataQualityAppSpecification,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.image_uri {
        object.key("ImageUri").string(var_1.as_str());
    }
    if let Some(var_2) = &input.container_entrypoint {
        let mut array_3 = object.key("ContainerEntrypoint").start_array();
        for item_4 in var_2 {
            {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    if let Some(var_5) = &input.container_arguments {
        let mut array_6 = object.key("ContainerArguments").start_array();
        for item_7 in var_5 {
            {
                array_6.value().string(item_7.as_str());
            }
        }
        array_6.finish();
    }
    if let Some(var_8) = &input.record_preprocessor_source_uri {
        object.key("RecordPreprocessorSourceUri").string(var_8.as_str());
    }
    if let Some(var_9) = &input.post_analytics_processor_source_uri {
        object.key("PostAnalyticsProcessorSourceUri").string(var_9.as_str());
    }
    if let Some(var_10) = &input.environment {
        #[allow(unused_mut)]
        let mut object_11 = object.key("Environment").start_object();
        for (key_12, value_13) in var_10 {
            {
                object_11.key(key_12.as_str()).string(value_13.as_str());
            }
        }
        object_11.finish();
    }
    Ok(())
}

pub(crate) fn de_data_quality_app_specification<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::DataQualityAppSpecification>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::DataQualityAppSpecificationBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "ImageUri" => {
                            builder = builder.set_image_uri(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "ContainerEntrypoint" => {
                            builder =
                                builder.set_container_entrypoint(crate::protocol_serde::shape_container_entrypoint::de_container_entrypoint(tokens)?);
                        }
                        "ContainerArguments" => {
                            builder = builder.set_container_arguments(
                                crate::protocol_serde::shape_monitoring_container_arguments::de_monitoring_container_arguments(tokens)?,
                            );
                        }
                        "RecordPreprocessorSourceUri" => {
                            builder = builder.set_record_preprocessor_source_uri(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "PostAnalyticsProcessorSourceUri" => {
                            builder = builder.set_post_analytics_processor_source_uri(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Environment" => {
                            builder = builder.set_environment(
                                crate::protocol_serde::shape_monitoring_environment_map::de_monitoring_environment_map(tokens)?,
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
            Ok(Some(crate::serde_util::data_quality_app_specification_correct_errors(builder).build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
