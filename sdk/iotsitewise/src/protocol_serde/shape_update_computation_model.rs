// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_update_computation_model_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::update_computation_model::UpdateComputationModelOutput,
    crate::operation::update_computation_model::UpdateComputationModelError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::update_computation_model::UpdateComputationModelError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::update_computation_model::UpdateComputationModelError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ConflictingOperationException" => crate::operation::update_computation_model::UpdateComputationModelError::ConflictingOperationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ConflictingOperationExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_conflicting_operation_exception::de_conflicting_operation_exception_json_err(_response_body, output)
                        .map_err(crate::operation::update_computation_model::UpdateComputationModelError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::conflicting_operation_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::update_computation_model::UpdateComputationModelError::unhandled)?
            };
            tmp
        }),
        "InternalFailureException" => crate::operation::update_computation_model::UpdateComputationModelError::InternalFailureException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalFailureExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_failure_exception::de_internal_failure_exception_json_err(_response_body, output)
                    .map_err(crate::operation::update_computation_model::UpdateComputationModelError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::internal_failure_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::update_computation_model::UpdateComputationModelError::unhandled)?
            };
            tmp
        }),
        "InvalidRequestException" => crate::operation::update_computation_model::UpdateComputationModelError::InvalidRequestException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output)
                    .map_err(crate::operation::update_computation_model::UpdateComputationModelError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::invalid_request_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::update_computation_model::UpdateComputationModelError::unhandled)?
            };
            tmp
        }),
        "LimitExceededException" => crate::operation::update_computation_model::UpdateComputationModelError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::LimitExceededExceptionBuilder::default();
                output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(_response_body, output)
                    .map_err(crate::operation::update_computation_model::UpdateComputationModelError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::limit_exceeded_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::update_computation_model::UpdateComputationModelError::unhandled)?
            };
            tmp
        }),
        "ResourceAlreadyExistsException" => {
            crate::operation::update_computation_model::UpdateComputationModelError::ResourceAlreadyExistsException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceAlreadyExistsExceptionBuilder::default();
                    output = crate::protocol_serde::shape_resource_already_exists_exception::de_resource_already_exists_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::update_computation_model::UpdateComputationModelError::unhandled)?;
                    let output = output.meta(generic);
                    crate::serde_util::resource_already_exists_exception_correct_errors(output)
                        .build()
                        .map_err(crate::operation::update_computation_model::UpdateComputationModelError::unhandled)?
                };
                tmp
            })
        }
        "ResourceNotFoundException" => crate::operation::update_computation_model::UpdateComputationModelError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::update_computation_model::UpdateComputationModelError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::resource_not_found_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::update_computation_model::UpdateComputationModelError::unhandled)?
            };
            tmp
        }),
        "ThrottlingException" => crate::operation::update_computation_model::UpdateComputationModelError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                    .map_err(crate::operation::update_computation_model::UpdateComputationModelError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::throttling_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::update_computation_model::UpdateComputationModelError::unhandled)?
            };
            tmp
        }),
        _ => crate::operation::update_computation_model::UpdateComputationModelError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_computation_model_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::update_computation_model::UpdateComputationModelOutput,
    crate::operation::update_computation_model::UpdateComputationModelError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::update_computation_model::builders::UpdateComputationModelOutputBuilder::default();
        output = crate::protocol_serde::shape_update_computation_model::de_update_computation_model(_response_body, output)
            .map_err(crate::operation::update_computation_model::UpdateComputationModelError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::update_computation_model_output_output_correct_errors(output).build()
    })
}

pub fn ser_update_computation_model_input(
    input: &crate::operation::update_computation_model::UpdateComputationModelInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_update_computation_model_input::ser_update_computation_model_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_update_computation_model(
    value: &[u8],
    mut builder: crate::operation::update_computation_model::builders::UpdateComputationModelOutputBuilder,
) -> ::std::result::Result<
    crate::operation::update_computation_model::builders::UpdateComputationModelOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "computationModelStatus" => {
                    builder = builder.set_computation_model_status(
                        crate::protocol_serde::shape_computation_model_status::de_computation_model_status(tokens)?,
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
