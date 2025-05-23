// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_associate_wireless_gateway_with_thing_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::associate_wireless_gateway_with_thing::AssociateWirelessGatewayWithThingOutput,
    crate::operation::associate_wireless_gateway_with_thing::AssociateWirelessGatewayWithThingError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::associate_wireless_gateway_with_thing::AssociateWirelessGatewayWithThingError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::associate_wireless_gateway_with_thing::AssociateWirelessGatewayWithThingError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => {
            crate::operation::associate_wireless_gateway_with_thing::AssociateWirelessGatewayWithThingError::AccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                        .map_err(crate::operation::associate_wireless_gateway_with_thing::AssociateWirelessGatewayWithThingError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ConflictException" => crate::operation::associate_wireless_gateway_with_thing::AssociateWirelessGatewayWithThingError::ConflictException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ConflictExceptionBuilder::default();
                output = crate::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output)
                    .map_err(crate::operation::associate_wireless_gateway_with_thing::AssociateWirelessGatewayWithThingError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InternalServerException" => {
            crate::operation::associate_wireless_gateway_with_thing::AssociateWirelessGatewayWithThingError::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
                    output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                        .map_err(crate::operation::associate_wireless_gateway_with_thing::AssociateWirelessGatewayWithThingError::unhandled)?;
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
            crate::operation::associate_wireless_gateway_with_thing::AssociateWirelessGatewayWithThingError::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                            .map_err(crate::operation::associate_wireless_gateway_with_thing::AssociateWirelessGatewayWithThingError::unhandled)?;
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
            crate::operation::associate_wireless_gateway_with_thing::AssociateWirelessGatewayWithThingError::ThrottlingException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                    output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                        .map_err(crate::operation::associate_wireless_gateway_with_thing::AssociateWirelessGatewayWithThingError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ValidationException" => {
            crate::operation::associate_wireless_gateway_with_thing::AssociateWirelessGatewayWithThingError::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                        .map_err(crate::operation::associate_wireless_gateway_with_thing::AssociateWirelessGatewayWithThingError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::associate_wireless_gateway_with_thing::AssociateWirelessGatewayWithThingError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_associate_wireless_gateway_with_thing_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::associate_wireless_gateway_with_thing::AssociateWirelessGatewayWithThingOutput,
    crate::operation::associate_wireless_gateway_with_thing::AssociateWirelessGatewayWithThingError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::associate_wireless_gateway_with_thing::builders::AssociateWirelessGatewayWithThingOutputBuilder::default();
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_associate_wireless_gateway_with_thing_input(
    input: &crate::operation::associate_wireless_gateway_with_thing::AssociateWirelessGatewayWithThingInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_associate_wireless_gateway_with_thing_input::ser_associate_wireless_gateway_with_thing_input_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
