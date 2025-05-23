// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_workflow_execution_info<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::WorkflowExecutionInfo>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::WorkflowExecutionInfoBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "execution" => {
                            builder = builder.set_execution(crate::protocol_serde::shape_workflow_execution::de_workflow_execution(tokens)?);
                        }
                        "workflowType" => {
                            builder = builder.set_workflow_type(crate::protocol_serde::shape_workflow_type::de_workflow_type(tokens)?);
                        }
                        "startTimestamp" => {
                            builder = builder.set_start_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "closeTimestamp" => {
                            builder = builder.set_close_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "executionStatus" => {
                            builder = builder.set_execution_status(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::ExecutionStatus::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "closeStatus" => {
                            builder = builder.set_close_status(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::CloseStatus::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "parent" => {
                            builder = builder.set_parent(crate::protocol_serde::shape_workflow_execution::de_workflow_execution(tokens)?);
                        }
                        "tagList" => {
                            builder = builder.set_tag_list(crate::protocol_serde::shape_tag_list::de_tag_list(tokens)?);
                        }
                        "cancelRequested" => {
                            builder = builder.set_cancel_requested(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
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
            Ok(Some(crate::serde_util::workflow_execution_info_correct_errors(builder).build().map_err(
                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
            )?))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
