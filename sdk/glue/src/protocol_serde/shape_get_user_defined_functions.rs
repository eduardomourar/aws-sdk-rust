// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_user_defined_functions_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_user_defined_functions::GetUserDefinedFunctionsOutput,
    crate::operation::get_user_defined_functions::GetUserDefinedFunctionsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::get_user_defined_functions::GetUserDefinedFunctionsError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::get_user_defined_functions::GetUserDefinedFunctionsError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "EntityNotFoundException" => crate::operation::get_user_defined_functions::GetUserDefinedFunctionsError::EntityNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::EntityNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_entity_not_found_exception::de_entity_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_user_defined_functions::GetUserDefinedFunctionsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "GlueEncryptionException" => crate::operation::get_user_defined_functions::GetUserDefinedFunctionsError::GlueEncryptionException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::GlueEncryptionExceptionBuilder::default();
                output = crate::protocol_serde::shape_glue_encryption_exception::de_glue_encryption_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_user_defined_functions::GetUserDefinedFunctionsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InternalServiceException" => crate::operation::get_user_defined_functions::GetUserDefinedFunctionsError::InternalServiceException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalServiceExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_service_exception::de_internal_service_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_user_defined_functions::GetUserDefinedFunctionsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidInputException" => crate::operation::get_user_defined_functions::GetUserDefinedFunctionsError::InvalidInputException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidInputExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_input_exception::de_invalid_input_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_user_defined_functions::GetUserDefinedFunctionsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "OperationTimeoutException" => crate::operation::get_user_defined_functions::GetUserDefinedFunctionsError::OperationTimeoutException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::OperationTimeoutExceptionBuilder::default();
                output = crate::protocol_serde::shape_operation_timeout_exception::de_operation_timeout_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_user_defined_functions::GetUserDefinedFunctionsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::get_user_defined_functions::GetUserDefinedFunctionsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_user_defined_functions_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_user_defined_functions::GetUserDefinedFunctionsOutput,
    crate::operation::get_user_defined_functions::GetUserDefinedFunctionsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_user_defined_functions::builders::GetUserDefinedFunctionsOutputBuilder::default();
        output = crate::protocol_serde::shape_get_user_defined_functions::de_get_user_defined_functions(_response_body, output)
            .map_err(crate::operation::get_user_defined_functions::GetUserDefinedFunctionsError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_get_user_defined_functions_input(
    input: &crate::operation::get_user_defined_functions::GetUserDefinedFunctionsInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_get_user_defined_functions_input::ser_get_user_defined_functions_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_get_user_defined_functions(
    value: &[u8],
    mut builder: crate::operation::get_user_defined_functions::builders::GetUserDefinedFunctionsOutputBuilder,
) -> ::std::result::Result<
    crate::operation::get_user_defined_functions::builders::GetUserDefinedFunctionsOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "UserDefinedFunctions" => {
                    builder = builder.set_user_defined_functions(
                        crate::protocol_serde::shape_user_defined_function_list::de_user_defined_function_list(tokens)?,
                    );
                }
                "NextToken" => {
                    builder = builder.set_next_token(
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
