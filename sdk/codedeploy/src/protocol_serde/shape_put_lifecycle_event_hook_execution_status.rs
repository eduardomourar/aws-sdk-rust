// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_put_lifecycle_event_hook_execution_status_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::put_lifecycle_event_hook_execution_status::PutLifecycleEventHookExecutionStatusOutput,
    crate::operation::put_lifecycle_event_hook_execution_status::PutLifecycleEventHookExecutionStatusError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::put_lifecycle_event_hook_execution_status::PutLifecycleEventHookExecutionStatusError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::put_lifecycle_event_hook_execution_status::PutLifecycleEventHookExecutionStatusError::unhandled(generic))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "DeploymentDoesNotExistException" => crate::operation::put_lifecycle_event_hook_execution_status::PutLifecycleEventHookExecutionStatusError::DeploymentDoesNotExistException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::DeploymentDoesNotExistExceptionBuilder::default();
                    output = crate::protocol_serde::shape_deployment_does_not_exist_exception::de_deployment_does_not_exist_exception_json_err(_response_body, output).map_err(crate::operation::put_lifecycle_event_hook_execution_status::PutLifecycleEventHookExecutionStatusError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                            tmp.message = _error_message;
                                                        }
            tmp
        }),
        "DeploymentIdRequiredException" => crate::operation::put_lifecycle_event_hook_execution_status::PutLifecycleEventHookExecutionStatusError::DeploymentIdRequiredException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::DeploymentIdRequiredExceptionBuilder::default();
                    output = crate::protocol_serde::shape_deployment_id_required_exception::de_deployment_id_required_exception_json_err(_response_body, output).map_err(crate::operation::put_lifecycle_event_hook_execution_status::PutLifecycleEventHookExecutionStatusError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                            tmp.message = _error_message;
                                                        }
            tmp
        }),
        "InvalidDeploymentIdException" => crate::operation::put_lifecycle_event_hook_execution_status::PutLifecycleEventHookExecutionStatusError::InvalidDeploymentIdException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidDeploymentIdExceptionBuilder::default();
                    output = crate::protocol_serde::shape_invalid_deployment_id_exception::de_invalid_deployment_id_exception_json_err(_response_body, output).map_err(crate::operation::put_lifecycle_event_hook_execution_status::PutLifecycleEventHookExecutionStatusError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                            tmp.message = _error_message;
                                                        }
            tmp
        }),
        "InvalidLifecycleEventHookExecutionIdException" => crate::operation::put_lifecycle_event_hook_execution_status::PutLifecycleEventHookExecutionStatusError::InvalidLifecycleEventHookExecutionIdException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidLifecycleEventHookExecutionIdExceptionBuilder::default();
                    output = crate::protocol_serde::shape_invalid_lifecycle_event_hook_execution_id_exception::de_invalid_lifecycle_event_hook_execution_id_exception_json_err(_response_body, output).map_err(crate::operation::put_lifecycle_event_hook_execution_status::PutLifecycleEventHookExecutionStatusError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                            tmp.message = _error_message;
                                                        }
            tmp
        }),
        "InvalidLifecycleEventHookExecutionStatusException" => crate::operation::put_lifecycle_event_hook_execution_status::PutLifecycleEventHookExecutionStatusError::InvalidLifecycleEventHookExecutionStatusException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidLifecycleEventHookExecutionStatusExceptionBuilder::default();
                    output = crate::protocol_serde::shape_invalid_lifecycle_event_hook_execution_status_exception::de_invalid_lifecycle_event_hook_execution_status_exception_json_err(_response_body, output).map_err(crate::operation::put_lifecycle_event_hook_execution_status::PutLifecycleEventHookExecutionStatusError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                            tmp.message = _error_message;
                                                        }
            tmp
        }),
        "LifecycleEventAlreadyCompletedException" => crate::operation::put_lifecycle_event_hook_execution_status::PutLifecycleEventHookExecutionStatusError::LifecycleEventAlreadyCompletedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::LifecycleEventAlreadyCompletedExceptionBuilder::default();
                    output = crate::protocol_serde::shape_lifecycle_event_already_completed_exception::de_lifecycle_event_already_completed_exception_json_err(_response_body, output).map_err(crate::operation::put_lifecycle_event_hook_execution_status::PutLifecycleEventHookExecutionStatusError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                            tmp.message = _error_message;
                                                        }
            tmp
        }),
        "UnsupportedActionForDeploymentTypeException" => crate::operation::put_lifecycle_event_hook_execution_status::PutLifecycleEventHookExecutionStatusError::UnsupportedActionForDeploymentTypeException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::UnsupportedActionForDeploymentTypeExceptionBuilder::default();
                    output = crate::protocol_serde::shape_unsupported_action_for_deployment_type_exception::de_unsupported_action_for_deployment_type_exception_json_err(_response_body, output).map_err(crate::operation::put_lifecycle_event_hook_execution_status::PutLifecycleEventHookExecutionStatusError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                            tmp.message = _error_message;
                                                        }
            tmp
        }),
        _ => crate::operation::put_lifecycle_event_hook_execution_status::PutLifecycleEventHookExecutionStatusError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_lifecycle_event_hook_execution_status_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::put_lifecycle_event_hook_execution_status::PutLifecycleEventHookExecutionStatusOutput,
    crate::operation::put_lifecycle_event_hook_execution_status::PutLifecycleEventHookExecutionStatusError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::put_lifecycle_event_hook_execution_status::builders::PutLifecycleEventHookExecutionStatusOutputBuilder::default();
        output = crate::protocol_serde::shape_put_lifecycle_event_hook_execution_status::de_put_lifecycle_event_hook_execution_status(
            _response_body,
            output,
        )
        .map_err(crate::operation::put_lifecycle_event_hook_execution_status::PutLifecycleEventHookExecutionStatusError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_put_lifecycle_event_hook_execution_status_input(
    input: &crate::operation::put_lifecycle_event_hook_execution_status::PutLifecycleEventHookExecutionStatusInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_put_lifecycle_event_hook_execution_status_input::ser_put_lifecycle_event_hook_execution_status_input_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_put_lifecycle_event_hook_execution_status(
    value: &[u8],
    mut builder: crate::operation::put_lifecycle_event_hook_execution_status::builders::PutLifecycleEventHookExecutionStatusOutputBuilder,
) -> ::std::result::Result<
    crate::operation::put_lifecycle_event_hook_execution_status::builders::PutLifecycleEventHookExecutionStatusOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "lifecycleEventHookExecutionId" => {
                    builder = builder.set_lifecycle_event_hook_execution_id(
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
    if tokens.next().is_some() {
        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}
