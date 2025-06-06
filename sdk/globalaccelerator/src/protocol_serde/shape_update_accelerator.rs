// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_update_accelerator_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::update_accelerator::UpdateAcceleratorOutput, crate::operation::update_accelerator::UpdateAcceleratorError>
{
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::update_accelerator::UpdateAcceleratorError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::update_accelerator::UpdateAcceleratorError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AcceleratorNotFoundException" => crate::operation::update_accelerator::UpdateAcceleratorError::AcceleratorNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::AcceleratorNotFoundExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_accelerator_not_found_exception::de_accelerator_not_found_exception_json_err(_response_body, output)
                        .map_err(crate::operation::update_accelerator::UpdateAcceleratorError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "AccessDeniedException" => crate::operation::update_accelerator::UpdateAcceleratorError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                    .map_err(crate::operation::update_accelerator::UpdateAcceleratorError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ConflictException" => crate::operation::update_accelerator::UpdateAcceleratorError::ConflictException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ConflictExceptionBuilder::default();
                output = crate::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output)
                    .map_err(crate::operation::update_accelerator::UpdateAcceleratorError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InternalServiceErrorException" => crate::operation::update_accelerator::UpdateAcceleratorError::InternalServiceErrorException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalServiceErrorExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_service_error_exception::de_internal_service_error_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::update_accelerator::UpdateAcceleratorError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidArgumentException" => crate::operation::update_accelerator::UpdateAcceleratorError::InvalidArgumentException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidArgumentExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_argument_exception::de_invalid_argument_exception_json_err(_response_body, output)
                    .map_err(crate::operation::update_accelerator::UpdateAcceleratorError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "TransactionInProgressException" => crate::operation::update_accelerator::UpdateAcceleratorError::TransactionInProgressException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::TransactionInProgressExceptionBuilder::default();
                output = crate::protocol_serde::shape_transaction_in_progress_exception::de_transaction_in_progress_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::update_accelerator::UpdateAcceleratorError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::update_accelerator::UpdateAcceleratorError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_accelerator_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::update_accelerator::UpdateAcceleratorOutput, crate::operation::update_accelerator::UpdateAcceleratorError>
{
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::update_accelerator::builders::UpdateAcceleratorOutputBuilder::default();
        output = crate::protocol_serde::shape_update_accelerator::de_update_accelerator(_response_body, output)
            .map_err(crate::operation::update_accelerator::UpdateAcceleratorError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_update_accelerator_input(
    input: &crate::operation::update_accelerator::UpdateAcceleratorInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_update_accelerator_input::ser_update_accelerator_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_update_accelerator(
    value: &[u8],
    mut builder: crate::operation::update_accelerator::builders::UpdateAcceleratorOutputBuilder,
) -> ::std::result::Result<
    crate::operation::update_accelerator::builders::UpdateAcceleratorOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "Accelerator" => {
                    builder = builder.set_accelerator(crate::protocol_serde::shape_accelerator::de_accelerator(tokens)?);
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
