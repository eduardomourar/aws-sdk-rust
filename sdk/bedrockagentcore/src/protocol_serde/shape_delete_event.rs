// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_event_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::delete_event::DeleteEventOutput, crate::operation::delete_event::DeleteEventError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::delete_event::DeleteEventError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::delete_event::DeleteEventError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::operation::delete_event::DeleteEventError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                    .map_err(crate::operation::delete_event::DeleteEventError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidInputException" => crate::operation::delete_event::DeleteEventError::InvalidInputException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidInputExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_input_exception::de_invalid_input_exception_json_err(_response_body, output)
                    .map_err(crate::operation::delete_event::DeleteEventError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::invalid_input_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::delete_event::DeleteEventError::unhandled)?
            };
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::delete_event::DeleteEventError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::delete_event::DeleteEventError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ServiceException" => crate::operation::delete_event::DeleteEventError::ServiceException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ServiceExceptionBuilder::default();
                output = crate::protocol_serde::shape_service_exception::de_service_exception_json_err(_response_body, output)
                    .map_err(crate::operation::delete_event::DeleteEventError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::service_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::delete_event::DeleteEventError::unhandled)?
            };
            tmp
        }),
        "ServiceQuotaExceededException" => crate::operation::delete_event::DeleteEventError::ServiceQuotaExceededException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ServiceQuotaExceededExceptionBuilder::default();
                output = crate::protocol_serde::shape_service_quota_exceeded_exception::de_service_quota_exceeded_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::delete_event::DeleteEventError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ThrottledException" => crate::operation::delete_event::DeleteEventError::ThrottledException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ThrottledExceptionBuilder::default();
                output = crate::protocol_serde::shape_throttled_exception::de_throttled_exception_json_err(_response_body, output)
                    .map_err(crate::operation::delete_event::DeleteEventError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::throttled_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::delete_event::DeleteEventError::unhandled)?
            };
            tmp
        }),
        "ValidationException" => crate::operation::delete_event::DeleteEventError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                    .map_err(crate::operation::delete_event::DeleteEventError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::validation_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::delete_event::DeleteEventError::unhandled)?
            };
            tmp
        }),
        _ => crate::operation::delete_event::DeleteEventError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_event_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::delete_event::DeleteEventOutput, crate::operation::delete_event::DeleteEventError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_event::builders::DeleteEventOutputBuilder::default();
        output = crate::protocol_serde::shape_delete_event::de_delete_event(_response_body, output)
            .map_err(crate::operation::delete_event::DeleteEventError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::delete_event_output_output_correct_errors(output)
            .build()
            .map_err(crate::operation::delete_event::DeleteEventError::unhandled)?
    })
}

pub(crate) fn de_delete_event(
    value: &[u8],
    mut builder: crate::operation::delete_event::builders::DeleteEventOutputBuilder,
) -> ::std::result::Result<crate::operation::delete_event::builders::DeleteEventOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
{
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "eventId" => {
                    builder = builder.set_event_id(
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
