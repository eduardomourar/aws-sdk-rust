// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_attach_load_balancer_tls_certificate_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::attach_load_balancer_tls_certificate::AttachLoadBalancerTlsCertificateOutput,
    crate::operation::attach_load_balancer_tls_certificate::AttachLoadBalancerTlsCertificateError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::attach_load_balancer_tls_certificate::AttachLoadBalancerTlsCertificateError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::attach_load_balancer_tls_certificate::AttachLoadBalancerTlsCertificateError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => {
            crate::operation::attach_load_balancer_tls_certificate::AttachLoadBalancerTlsCertificateError::AccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                        .map_err(crate::operation::attach_load_balancer_tls_certificate::AttachLoadBalancerTlsCertificateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "AccountSetupInProgressException" => {
            crate::operation::attach_load_balancer_tls_certificate::AttachLoadBalancerTlsCertificateError::AccountSetupInProgressException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccountSetupInProgressExceptionBuilder::default();
                    output = crate::protocol_serde::shape_account_setup_in_progress_exception::de_account_setup_in_progress_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::attach_load_balancer_tls_certificate::AttachLoadBalancerTlsCertificateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidInputException" => {
            crate::operation::attach_load_balancer_tls_certificate::AttachLoadBalancerTlsCertificateError::InvalidInputException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidInputExceptionBuilder::default();
                    output = crate::protocol_serde::shape_invalid_input_exception::de_invalid_input_exception_json_err(_response_body, output)
                        .map_err(crate::operation::attach_load_balancer_tls_certificate::AttachLoadBalancerTlsCertificateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "NotFoundException" => crate::operation::attach_load_balancer_tls_certificate::AttachLoadBalancerTlsCertificateError::NotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::NotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::attach_load_balancer_tls_certificate::AttachLoadBalancerTlsCertificateError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "OperationFailureException" => {
            crate::operation::attach_load_balancer_tls_certificate::AttachLoadBalancerTlsCertificateError::OperationFailureException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::OperationFailureExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_operation_failure_exception::de_operation_failure_exception_json_err(_response_body, output)
                            .map_err(crate::operation::attach_load_balancer_tls_certificate::AttachLoadBalancerTlsCertificateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "RegionSetupInProgressException" => {
            crate::operation::attach_load_balancer_tls_certificate::AttachLoadBalancerTlsCertificateError::RegionSetupInProgressException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::RegionSetupInProgressExceptionBuilder::default();
                    output = crate::protocol_serde::shape_region_setup_in_progress_exception::de_region_setup_in_progress_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::attach_load_balancer_tls_certificate::AttachLoadBalancerTlsCertificateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ServiceException" => crate::operation::attach_load_balancer_tls_certificate::AttachLoadBalancerTlsCertificateError::ServiceException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ServiceExceptionBuilder::default();
                output = crate::protocol_serde::shape_service_exception::de_service_exception_json_err(_response_body, output)
                    .map_err(crate::operation::attach_load_balancer_tls_certificate::AttachLoadBalancerTlsCertificateError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "UnauthenticatedException" => {
            crate::operation::attach_load_balancer_tls_certificate::AttachLoadBalancerTlsCertificateError::UnauthenticatedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::UnauthenticatedExceptionBuilder::default();
                    output = crate::protocol_serde::shape_unauthenticated_exception::de_unauthenticated_exception_json_err(_response_body, output)
                        .map_err(crate::operation::attach_load_balancer_tls_certificate::AttachLoadBalancerTlsCertificateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::attach_load_balancer_tls_certificate::AttachLoadBalancerTlsCertificateError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_attach_load_balancer_tls_certificate_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::attach_load_balancer_tls_certificate::AttachLoadBalancerTlsCertificateOutput,
    crate::operation::attach_load_balancer_tls_certificate::AttachLoadBalancerTlsCertificateError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::attach_load_balancer_tls_certificate::builders::AttachLoadBalancerTlsCertificateOutputBuilder::default();
        output = crate::protocol_serde::shape_attach_load_balancer_tls_certificate::de_attach_load_balancer_tls_certificate(_response_body, output)
            .map_err(crate::operation::attach_load_balancer_tls_certificate::AttachLoadBalancerTlsCertificateError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_attach_load_balancer_tls_certificate_input(
    input: &crate::operation::attach_load_balancer_tls_certificate::AttachLoadBalancerTlsCertificateInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_attach_load_balancer_tls_certificate_input::ser_attach_load_balancer_tls_certificate_input_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_attach_load_balancer_tls_certificate(
    value: &[u8],
    mut builder: crate::operation::attach_load_balancer_tls_certificate::builders::AttachLoadBalancerTlsCertificateOutputBuilder,
) -> ::std::result::Result<
    crate::operation::attach_load_balancer_tls_certificate::builders::AttachLoadBalancerTlsCertificateOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "operations" => {
                    builder = builder.set_operations(crate::protocol_serde::shape_operation_list::de_operation_list(tokens)?);
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
