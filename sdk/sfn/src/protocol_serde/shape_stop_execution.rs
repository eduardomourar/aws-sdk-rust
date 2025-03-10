// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_stop_execution_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::stop_execution::StopExecutionOutput, crate::operation::stop_execution::StopExecutionError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::stop_execution::StopExecutionError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::stop_execution::StopExecutionError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ExecutionDoesNotExist" => crate::operation::stop_execution::StopExecutionError::ExecutionDoesNotExist({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ExecutionDoesNotExistBuilder::default();
                output = crate::protocol_serde::shape_execution_does_not_exist::de_execution_does_not_exist_json_err(_response_body, output)
                    .map_err(crate::operation::stop_execution::StopExecutionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidArn" => crate::operation::stop_execution::StopExecutionError::InvalidArn({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidArnBuilder::default();
                output = crate::protocol_serde::shape_invalid_arn::de_invalid_arn_json_err(_response_body, output)
                    .map_err(crate::operation::stop_execution::StopExecutionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "KmsAccessDeniedException" => crate::operation::stop_execution::StopExecutionError::KmsAccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::KmsAccessDeniedExceptionBuilder::default();
                output = crate::protocol_serde::shape_kms_access_denied_exception::de_kms_access_denied_exception_json_err(_response_body, output)
                    .map_err(crate::operation::stop_execution::StopExecutionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "KmsInvalidStateException" => crate::operation::stop_execution::StopExecutionError::KmsInvalidStateException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::KmsInvalidStateExceptionBuilder::default();
                output = crate::protocol_serde::shape_kms_invalid_state_exception::de_kms_invalid_state_exception_json_err(_response_body, output)
                    .map_err(crate::operation::stop_execution::StopExecutionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "KmsThrottlingException" => crate::operation::stop_execution::StopExecutionError::KmsThrottlingException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::KmsThrottlingExceptionBuilder::default();
                output = crate::protocol_serde::shape_kms_throttling_exception::de_kms_throttling_exception_json_err(_response_body, output)
                    .map_err(crate::operation::stop_execution::StopExecutionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ValidationException" => crate::operation::stop_execution::StopExecutionError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                    .map_err(crate::operation::stop_execution::StopExecutionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::stop_execution::StopExecutionError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_stop_execution_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::stop_execution::StopExecutionOutput, crate::operation::stop_execution::StopExecutionError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::stop_execution::builders::StopExecutionOutputBuilder::default();
        output = crate::protocol_serde::shape_stop_execution::de_stop_execution(_response_body, output)
            .map_err(crate::operation::stop_execution::StopExecutionError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::stop_execution_output_output_correct_errors(output)
            .build()
            .map_err(crate::operation::stop_execution::StopExecutionError::unhandled)?
    })
}

pub fn ser_stop_execution_input(
    input: &crate::operation::stop_execution::StopExecutionInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_stop_execution_input::ser_stop_execution_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_stop_execution(
    value: &[u8],
    mut builder: crate::operation::stop_execution::builders::StopExecutionOutputBuilder,
) -> ::std::result::Result<
    crate::operation::stop_execution::builders::StopExecutionOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "stopDate" => {
                    builder = builder.set_stop_date(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                        tokens.next(),
                        ::aws_smithy_types::date_time::Format::EpochSeconds,
                    )?);
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
