// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_invocation_logs_config(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::InvocationLogsConfig,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if input.use_prompt_response {
        object.key("usePromptResponse").boolean(input.use_prompt_response);
    }
    if let Some(var_1) = &input.invocation_log_source {
        #[allow(unused_mut)]
        let mut object_2 = object.key("invocationLogSource").start_object();
        crate::protocol_serde::shape_invocation_log_source::ser_invocation_log_source(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.request_metadata_filters {
        #[allow(unused_mut)]
        let mut object_4 = object.key("requestMetadataFilters").start_object();
        crate::protocol_serde::shape_request_metadata_filters::ser_request_metadata_filters(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

pub(crate) fn de_invocation_logs_config<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::InvocationLogsConfig>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::InvocationLogsConfigBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "usePromptResponse" => {
                            builder = builder.set_use_prompt_response(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "invocationLogSource" => {
                            builder = builder
                                .set_invocation_log_source(crate::protocol_serde::shape_invocation_log_source::de_invocation_log_source(tokens)?);
                        }
                        "requestMetadataFilters" => {
                            builder = builder.set_request_metadata_filters(
                                crate::protocol_serde::shape_request_metadata_filters::de_request_metadata_filters(tokens)?,
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
            Ok(Some(crate::serde_util::invocation_logs_config_correct_errors(builder).build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
