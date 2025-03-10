// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_set_default_permission_version_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::set_default_permission_version::SetDefaultPermissionVersionOutput,
    crate::operation::set_default_permission_version::SetDefaultPermissionVersionError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::set_default_permission_version::SetDefaultPermissionVersionError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::set_default_permission_version::SetDefaultPermissionVersionError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "IdempotentParameterMismatchException" => {
            crate::operation::set_default_permission_version::SetDefaultPermissionVersionError::IdempotentParameterMismatchException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::IdempotentParameterMismatchExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_idempotent_parameter_mismatch_exception::de_idempotent_parameter_mismatch_exception_json_err(
                            _response_body,
                            output,
                        )
                        .map_err(crate::operation::set_default_permission_version::SetDefaultPermissionVersionError::unhandled)?;
                    let output = output.meta(generic);
                    crate::serde_util::idempotent_parameter_mismatch_exception_correct_errors(output)
                        .build()
                        .map_err(crate::operation::set_default_permission_version::SetDefaultPermissionVersionError::unhandled)?
                };
                tmp
            })
        }
        "InvalidClientTokenException" => {
            crate::operation::set_default_permission_version::SetDefaultPermissionVersionError::InvalidClientTokenException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidClientTokenExceptionBuilder::default();
                    output = crate::protocol_serde::shape_invalid_client_token_exception::de_invalid_client_token_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::set_default_permission_version::SetDefaultPermissionVersionError::unhandled)?;
                    let output = output.meta(generic);
                    crate::serde_util::invalid_client_token_exception_correct_errors(output)
                        .build()
                        .map_err(crate::operation::set_default_permission_version::SetDefaultPermissionVersionError::unhandled)?
                };
                tmp
            })
        }
        "InvalidParameterException" => {
            crate::operation::set_default_permission_version::SetDefaultPermissionVersionError::InvalidParameterException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidParameterExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(_response_body, output)
                            .map_err(crate::operation::set_default_permission_version::SetDefaultPermissionVersionError::unhandled)?;
                    let output = output.meta(generic);
                    crate::serde_util::invalid_parameter_exception_correct_errors(output)
                        .build()
                        .map_err(crate::operation::set_default_permission_version::SetDefaultPermissionVersionError::unhandled)?
                };
                tmp
            })
        }
        "MalformedArnException" => crate::operation::set_default_permission_version::SetDefaultPermissionVersionError::MalformedArnException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::MalformedArnExceptionBuilder::default();
                output = crate::protocol_serde::shape_malformed_arn_exception::de_malformed_arn_exception_json_err(_response_body, output)
                    .map_err(crate::operation::set_default_permission_version::SetDefaultPermissionVersionError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::malformed_arn_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::set_default_permission_version::SetDefaultPermissionVersionError::unhandled)?
            };
            tmp
        }),
        "ServerInternalException" => crate::operation::set_default_permission_version::SetDefaultPermissionVersionError::ServerInternalException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ServerInternalExceptionBuilder::default();
                output = crate::protocol_serde::shape_server_internal_exception::de_server_internal_exception_json_err(_response_body, output)
                    .map_err(crate::operation::set_default_permission_version::SetDefaultPermissionVersionError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::server_internal_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::set_default_permission_version::SetDefaultPermissionVersionError::unhandled)?
            };
            tmp
        }),
        "ServiceUnavailableException" => {
            crate::operation::set_default_permission_version::SetDefaultPermissionVersionError::ServiceUnavailableException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ServiceUnavailableExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_service_unavailable_exception::de_service_unavailable_exception_json_err(_response_body, output)
                            .map_err(crate::operation::set_default_permission_version::SetDefaultPermissionVersionError::unhandled)?;
                    let output = output.meta(generic);
                    crate::serde_util::service_unavailable_exception_correct_errors(output)
                        .build()
                        .map_err(crate::operation::set_default_permission_version::SetDefaultPermissionVersionError::unhandled)?
                };
                tmp
            })
        }
        "UnknownResourceException" => crate::operation::set_default_permission_version::SetDefaultPermissionVersionError::UnknownResourceException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::UnknownResourceExceptionBuilder::default();
                output = crate::protocol_serde::shape_unknown_resource_exception::de_unknown_resource_exception_json_err(_response_body, output)
                    .map_err(crate::operation::set_default_permission_version::SetDefaultPermissionVersionError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::unknown_resource_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::set_default_permission_version::SetDefaultPermissionVersionError::unhandled)?
            };
            tmp
        }),
        _ => crate::operation::set_default_permission_version::SetDefaultPermissionVersionError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_set_default_permission_version_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::set_default_permission_version::SetDefaultPermissionVersionOutput,
    crate::operation::set_default_permission_version::SetDefaultPermissionVersionError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::set_default_permission_version::builders::SetDefaultPermissionVersionOutputBuilder::default();
        output = crate::protocol_serde::shape_set_default_permission_version::de_set_default_permission_version(_response_body, output)
            .map_err(crate::operation::set_default_permission_version::SetDefaultPermissionVersionError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_set_default_permission_version_input(
    input: &crate::operation::set_default_permission_version::SetDefaultPermissionVersionInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_set_default_permission_version_input::ser_set_default_permission_version_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_set_default_permission_version(
    value: &[u8],
    mut builder: crate::operation::set_default_permission_version::builders::SetDefaultPermissionVersionOutputBuilder,
) -> ::std::result::Result<
    crate::operation::set_default_permission_version::builders::SetDefaultPermissionVersionOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "clientToken" => {
                    builder = builder.set_client_token(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "returnValue" => {
                    builder = builder.set_return_value(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
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
