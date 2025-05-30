// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_send_serial_console_ssh_public_key_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::send_serial_console_ssh_public_key::SendSerialConsoleSshPublicKeyOutput,
    crate::operation::send_serial_console_ssh_public_key::SendSerialConsoleSSHPublicKeyError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::send_serial_console_ssh_public_key::SendSerialConsoleSSHPublicKeyError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::send_serial_console_ssh_public_key::SendSerialConsoleSSHPublicKeyError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AuthException" => crate::operation::send_serial_console_ssh_public_key::SendSerialConsoleSSHPublicKeyError::AuthException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::AuthExceptionBuilder::default();
                output = crate::protocol_serde::shape_auth_exception::de_auth_exception_json_err(_response_body, output)
                    .map_err(crate::operation::send_serial_console_ssh_public_key::SendSerialConsoleSSHPublicKeyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "EC2InstanceNotFoundException" => {
            crate::operation::send_serial_console_ssh_public_key::SendSerialConsoleSSHPublicKeyError::Ec2InstanceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::Ec2InstanceNotFoundExceptionBuilder::default();
                    output = crate::protocol_serde::shape_ec2_instance_not_found_exception::de_ec2_instance_not_found_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::send_serial_console_ssh_public_key::SendSerialConsoleSSHPublicKeyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "EC2InstanceStateInvalidException" => {
            crate::operation::send_serial_console_ssh_public_key::SendSerialConsoleSSHPublicKeyError::Ec2InstanceStateInvalidException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::Ec2InstanceStateInvalidExceptionBuilder::default();
                    output = crate::protocol_serde::shape_ec2_instance_state_invalid_exception::de_ec2_instance_state_invalid_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::send_serial_console_ssh_public_key::SendSerialConsoleSSHPublicKeyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "EC2InstanceTypeInvalidException" => {
            crate::operation::send_serial_console_ssh_public_key::SendSerialConsoleSSHPublicKeyError::Ec2InstanceTypeInvalidException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::Ec2InstanceTypeInvalidExceptionBuilder::default();
                    output = crate::protocol_serde::shape_ec2_instance_type_invalid_exception::de_ec2_instance_type_invalid_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::send_serial_console_ssh_public_key::SendSerialConsoleSSHPublicKeyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "EC2InstanceUnavailableException" => {
            crate::operation::send_serial_console_ssh_public_key::SendSerialConsoleSSHPublicKeyError::Ec2InstanceUnavailableException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::Ec2InstanceUnavailableExceptionBuilder::default();
                    output = crate::protocol_serde::shape_ec2_instance_unavailable_exception::de_ec2_instance_unavailable_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::send_serial_console_ssh_public_key::SendSerialConsoleSSHPublicKeyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidArgsException" => crate::operation::send_serial_console_ssh_public_key::SendSerialConsoleSSHPublicKeyError::InvalidArgsException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidArgsExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_args_exception::de_invalid_args_exception_json_err(_response_body, output)
                    .map_err(crate::operation::send_serial_console_ssh_public_key::SendSerialConsoleSSHPublicKeyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "SerialConsoleAccessDisabledException" => {
            crate::operation::send_serial_console_ssh_public_key::SendSerialConsoleSSHPublicKeyError::SerialConsoleAccessDisabledException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::SerialConsoleAccessDisabledExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_serial_console_access_disabled_exception::de_serial_console_access_disabled_exception_json_err(
                            _response_body,
                            output,
                        )
                        .map_err(crate::operation::send_serial_console_ssh_public_key::SendSerialConsoleSSHPublicKeyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "SerialConsoleSessionLimitExceededException" => {
            crate::operation::send_serial_console_ssh_public_key::SendSerialConsoleSSHPublicKeyError::SerialConsoleSessionLimitExceededException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::SerialConsoleSessionLimitExceededExceptionBuilder::default();
                    output = crate::protocol_serde::shape_serial_console_session_limit_exceeded_exception::de_serial_console_session_limit_exceeded_exception_json_err(_response_body, output).map_err(crate::operation::send_serial_console_ssh_public_key::SendSerialConsoleSSHPublicKeyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "SerialConsoleSessionUnavailableException" => {
            crate::operation::send_serial_console_ssh_public_key::SendSerialConsoleSSHPublicKeyError::SerialConsoleSessionUnavailableException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::SerialConsoleSessionUnavailableExceptionBuilder::default();
                    output = crate::protocol_serde::shape_serial_console_session_unavailable_exception::de_serial_console_session_unavailable_exception_json_err(_response_body, output).map_err(crate::operation::send_serial_console_ssh_public_key::SendSerialConsoleSSHPublicKeyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "SerialConsoleSessionUnsupportedException" => {
            crate::operation::send_serial_console_ssh_public_key::SendSerialConsoleSSHPublicKeyError::SerialConsoleSessionUnsupportedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::SerialConsoleSessionUnsupportedExceptionBuilder::default();
                    output = crate::protocol_serde::shape_serial_console_session_unsupported_exception::de_serial_console_session_unsupported_exception_json_err(_response_body, output).map_err(crate::operation::send_serial_console_ssh_public_key::SendSerialConsoleSSHPublicKeyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ServiceException" => crate::operation::send_serial_console_ssh_public_key::SendSerialConsoleSSHPublicKeyError::ServiceException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ServiceExceptionBuilder::default();
                output = crate::protocol_serde::shape_service_exception::de_service_exception_json_err(_response_body, output)
                    .map_err(crate::operation::send_serial_console_ssh_public_key::SendSerialConsoleSSHPublicKeyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ThrottlingException" => crate::operation::send_serial_console_ssh_public_key::SendSerialConsoleSSHPublicKeyError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                    .map_err(crate::operation::send_serial_console_ssh_public_key::SendSerialConsoleSSHPublicKeyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::send_serial_console_ssh_public_key::SendSerialConsoleSSHPublicKeyError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_send_serial_console_ssh_public_key_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::send_serial_console_ssh_public_key::SendSerialConsoleSshPublicKeyOutput,
    crate::operation::send_serial_console_ssh_public_key::SendSerialConsoleSSHPublicKeyError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::send_serial_console_ssh_public_key::builders::SendSerialConsoleSshPublicKeyOutputBuilder::default();
        output = crate::protocol_serde::shape_send_serial_console_ssh_public_key::de_send_serial_console_ssh_public_key(_response_body, output)
            .map_err(crate::operation::send_serial_console_ssh_public_key::SendSerialConsoleSSHPublicKeyError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_send_serial_console_ssh_public_key_input(
    input: &crate::operation::send_serial_console_ssh_public_key::SendSerialConsoleSshPublicKeyInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_send_serial_console_ssh_public_key_input::ser_send_serial_console_ssh_public_key_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_send_serial_console_ssh_public_key(
    value: &[u8],
    mut builder: crate::operation::send_serial_console_ssh_public_key::builders::SendSerialConsoleSshPublicKeyOutputBuilder,
) -> ::std::result::Result<
    crate::operation::send_serial_console_ssh_public_key::builders::SendSerialConsoleSshPublicKeyOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "RequestId" => {
                    builder = builder.set_request_id(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "Success" => {
                    builder = builder.set_success(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
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
