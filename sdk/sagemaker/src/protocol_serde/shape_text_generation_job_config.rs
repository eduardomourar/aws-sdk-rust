// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_text_generation_job_config(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::TextGenerationJobConfig,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.completion_criteria {
        #[allow(unused_mut)]
        let mut object_2 = object.key("CompletionCriteria").start_object();
        crate::protocol_serde::shape_auto_ml_job_completion_criteria::ser_auto_ml_job_completion_criteria(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.base_model_name {
        object.key("BaseModelName").string(var_3.as_str());
    }
    if let Some(var_4) = &input.text_generation_hyper_parameters {
        #[allow(unused_mut)]
        let mut object_5 = object.key("TextGenerationHyperParameters").start_object();
        for (key_6, value_7) in var_4 {
            {
                object_5.key(key_6.as_str()).string(value_7.as_str());
            }
        }
        object_5.finish();
    }
    if let Some(var_8) = &input.model_access_config {
        #[allow(unused_mut)]
        let mut object_9 = object.key("ModelAccessConfig").start_object();
        crate::protocol_serde::shape_model_access_config::ser_model_access_config(&mut object_9, var_8)?;
        object_9.finish();
    }
    Ok(())
}

pub(crate) fn de_text_generation_job_config<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::TextGenerationJobConfig>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::TextGenerationJobConfigBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "CompletionCriteria" => {
                            builder = builder.set_completion_criteria(
                                crate::protocol_serde::shape_auto_ml_job_completion_criteria::de_auto_ml_job_completion_criteria(tokens)?,
                            );
                        }
                        "BaseModelName" => {
                            builder = builder.set_base_model_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "TextGenerationHyperParameters" => {
                            builder = builder.set_text_generation_hyper_parameters(
                                crate::protocol_serde::shape_text_generation_hyper_parameters::de_text_generation_hyper_parameters(tokens)?,
                            );
                        }
                        "ModelAccessConfig" => {
                            builder =
                                builder.set_model_access_config(crate::protocol_serde::shape_model_access_config::de_model_access_config(tokens)?);
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
