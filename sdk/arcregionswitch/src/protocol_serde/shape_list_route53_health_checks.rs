// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_list_route53_health_checks_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::list_route53_health_checks::ListRoute53HealthChecksOutput,
    crate::operation::list_route53_health_checks::ListRoute53HealthChecksError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::cbor_errors::parse_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::list_route53_health_checks::ListRoute53HealthChecksError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::list_route53_health_checks::ListRoute53HealthChecksError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::operation::list_route53_health_checks::ListRoute53HealthChecksError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_cbor_err(_response_body, output)
                    .map_err(crate::operation::list_route53_health_checks::ListRoute53HealthChecksError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::access_denied_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::list_route53_health_checks::ListRoute53HealthChecksError::unhandled)?
            };
            tmp
        }),
        "InternalServerException" => crate::operation::list_route53_health_checks::ListRoute53HealthChecksError::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_cbor_err(_response_body, output)
                    .map_err(crate::operation::list_route53_health_checks::ListRoute53HealthChecksError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::internal_server_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::list_route53_health_checks::ListRoute53HealthChecksError::unhandled)?
            };
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::list_route53_health_checks::ListRoute53HealthChecksError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_cbor_err(_response_body, output)
                    .map_err(crate::operation::list_route53_health_checks::ListRoute53HealthChecksError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::resource_not_found_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::list_route53_health_checks::ListRoute53HealthChecksError::unhandled)?
            };
            tmp
        }),
        _ => crate::operation::list_route53_health_checks::ListRoute53HealthChecksError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_route53_health_checks_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::list_route53_health_checks::ListRoute53HealthChecksOutput,
    crate::operation::list_route53_health_checks::ListRoute53HealthChecksError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::list_route53_health_checks::builders::ListRoute53HealthChecksOutputBuilder::default();
        output = crate::protocol_serde::shape_list_route53_health_checks::de_list_route53_health_checks(_response_body, output)
            .map_err(crate::operation::list_route53_health_checks::ListRoute53HealthChecksError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_list_route53_health_checks_input(
    input: &crate::operation::list_route53_health_checks::ListRoute53HealthChecksInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut encoder = ::aws_smithy_cbor::Encoder::new(Vec::new());
    {
        let encoder = &mut encoder;
        crate::protocol_serde::shape_list_route53_health_checks_input::ser_list_route53_health_checks_input_input(encoder, input)?;
    }
    Ok(::aws_smithy_types::body::SdkBody::from(encoder.into_writer()))
}

pub(crate) fn de_list_route53_health_checks(
    value: &[u8],
    mut builder: crate::operation::list_route53_health_checks::builders::ListRoute53HealthChecksOutputBuilder,
) -> ::std::result::Result<
    crate::operation::list_route53_health_checks::builders::ListRoute53HealthChecksOutputBuilder,
    ::aws_smithy_cbor::decode::DeserializeError,
> {
    #[allow(clippy::match_single_binding)]
    fn pair(
        mut builder: crate::operation::list_route53_health_checks::builders::ListRoute53HealthChecksOutputBuilder,
        decoder: &mut ::aws_smithy_cbor::Decoder,
    ) -> ::std::result::Result<
        crate::operation::list_route53_health_checks::builders::ListRoute53HealthChecksOutputBuilder,
        ::aws_smithy_cbor::decode::DeserializeError,
    > {
        builder = match decoder.str()?.as_ref() {
            "healthChecks" => ::aws_smithy_cbor::decode::set_optional(builder, decoder, |builder, decoder| {
                Ok(builder.set_health_checks(Some(
                    crate::protocol_serde::shape_route53_health_check_list::de_route53_health_check_list(decoder)?,
                )))
            })?,
            "nextToken" => {
                ::aws_smithy_cbor::decode::set_optional(builder, decoder, |builder, decoder| Ok(builder.set_next_token(Some(decoder.string()?))))?
            }
            _ => {
                decoder.skip()?;
                builder
            }
        };
        Ok(builder)
    }

    let decoder = &mut ::aws_smithy_cbor::Decoder::new(value);

    match decoder.map()? {
        None => loop {
            match decoder.datatype()? {
                ::aws_smithy_cbor::data::Type::Break => {
                    decoder.skip()?;
                    break;
                }
                _ => {
                    builder = pair(builder, decoder)?;
                }
            };
        },
        Some(n) => {
            for _ in 0..n {
                builder = pair(builder, decoder)?;
            }
        }
    };

    if decoder.position() != value.len() {
        return Err(::aws_smithy_cbor::decode::DeserializeError::expected_end_of_stream(decoder.position()));
    }

    Ok(builder)
}
