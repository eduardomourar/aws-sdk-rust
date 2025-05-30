// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_batch_get_calculated_attribute_for_profile_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::batch_get_calculated_attribute_for_profile::BatchGetCalculatedAttributeForProfileOutput,
    crate::operation::batch_get_calculated_attribute_for_profile::BatchGetCalculatedAttributeForProfileError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::batch_get_calculated_attribute_for_profile::BatchGetCalculatedAttributeForProfileError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::batch_get_calculated_attribute_for_profile::BatchGetCalculatedAttributeForProfileError::unhandled(generic))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => {
            crate::operation::batch_get_calculated_attribute_for_profile::BatchGetCalculatedAttributeForProfileError::AccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                        .map_err(
                            crate::operation::batch_get_calculated_attribute_for_profile::BatchGetCalculatedAttributeForProfileError::unhandled,
                        )?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "BadRequestException" => {
            crate::operation::batch_get_calculated_attribute_for_profile::BatchGetCalculatedAttributeForProfileError::BadRequestException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::BadRequestExceptionBuilder::default();
                    output = crate::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(_response_body, output).map_err(
                        crate::operation::batch_get_calculated_attribute_for_profile::BatchGetCalculatedAttributeForProfileError::unhandled,
                    )?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InternalServerException" => {
            crate::operation::batch_get_calculated_attribute_for_profile::BatchGetCalculatedAttributeForProfileError::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
                    output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                        .map_err(
                            crate::operation::batch_get_calculated_attribute_for_profile::BatchGetCalculatedAttributeForProfileError::unhandled,
                        )?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ResourceNotFoundException" => {
            crate::operation::batch_get_calculated_attribute_for_profile::BatchGetCalculatedAttributeForProfileError::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                            .map_err(
                            crate::operation::batch_get_calculated_attribute_for_profile::BatchGetCalculatedAttributeForProfileError::unhandled,
                        )?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ThrottlingException" => {
            crate::operation::batch_get_calculated_attribute_for_profile::BatchGetCalculatedAttributeForProfileError::ThrottlingException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                    output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output).map_err(
                        crate::operation::batch_get_calculated_attribute_for_profile::BatchGetCalculatedAttributeForProfileError::unhandled,
                    )?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::batch_get_calculated_attribute_for_profile::BatchGetCalculatedAttributeForProfileError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_batch_get_calculated_attribute_for_profile_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::batch_get_calculated_attribute_for_profile::BatchGetCalculatedAttributeForProfileOutput,
    crate::operation::batch_get_calculated_attribute_for_profile::BatchGetCalculatedAttributeForProfileError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::batch_get_calculated_attribute_for_profile::builders::BatchGetCalculatedAttributeForProfileOutputBuilder::default();
        output = crate::protocol_serde::shape_batch_get_calculated_attribute_for_profile::de_batch_get_calculated_attribute_for_profile(
            _response_body,
            output,
        )
        .map_err(crate::operation::batch_get_calculated_attribute_for_profile::BatchGetCalculatedAttributeForProfileError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_batch_get_calculated_attribute_for_profile_input(
    input: &crate::operation::batch_get_calculated_attribute_for_profile::BatchGetCalculatedAttributeForProfileInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_batch_get_calculated_attribute_for_profile_input::ser_batch_get_calculated_attribute_for_profile_input_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_batch_get_calculated_attribute_for_profile(
    value: &[u8],
    mut builder: crate::operation::batch_get_calculated_attribute_for_profile::builders::BatchGetCalculatedAttributeForProfileOutputBuilder,
) -> ::std::result::Result<
    crate::operation::batch_get_calculated_attribute_for_profile::builders::BatchGetCalculatedAttributeForProfileOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "CalculatedAttributeValues" => {
                    builder = builder.set_calculated_attribute_values(
                        crate::protocol_serde::shape_calculated_attribute_value_list::de_calculated_attribute_value_list(tokens)?,
                    );
                }
                "ConditionOverrides" => {
                    builder = builder.set_condition_overrides(crate::protocol_serde::shape_condition_overrides::de_condition_overrides(tokens)?);
                }
                "Errors" => {
                    builder = builder.set_errors(
                            crate::protocol_serde::shape_batch_get_calculated_attribute_for_profile_error_list::de_batch_get_calculated_attribute_for_profile_error_list(tokens)?
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
