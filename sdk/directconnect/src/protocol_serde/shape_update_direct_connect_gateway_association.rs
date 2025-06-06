// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_update_direct_connect_gateway_association_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::update_direct_connect_gateway_association::UpdateDirectConnectGatewayAssociationOutput,
    crate::operation::update_direct_connect_gateway_association::UpdateDirectConnectGatewayAssociationError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::update_direct_connect_gateway_association::UpdateDirectConnectGatewayAssociationError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::update_direct_connect_gateway_association::UpdateDirectConnectGatewayAssociationError::unhandled(generic))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "DirectConnectClientException" => {
            crate::operation::update_direct_connect_gateway_association::UpdateDirectConnectGatewayAssociationError::DirectConnectClientException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::DirectConnectClientExceptionBuilder::default();
                    output = crate::protocol_serde::shape_direct_connect_client_exception::de_direct_connect_client_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::update_direct_connect_gateway_association::UpdateDirectConnectGatewayAssociationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "DirectConnectServerException" => {
            crate::operation::update_direct_connect_gateway_association::UpdateDirectConnectGatewayAssociationError::DirectConnectServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::DirectConnectServerExceptionBuilder::default();
                    output = crate::protocol_serde::shape_direct_connect_server_exception::de_direct_connect_server_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::update_direct_connect_gateway_association::UpdateDirectConnectGatewayAssociationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::update_direct_connect_gateway_association::UpdateDirectConnectGatewayAssociationError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_direct_connect_gateway_association_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::update_direct_connect_gateway_association::UpdateDirectConnectGatewayAssociationOutput,
    crate::operation::update_direct_connect_gateway_association::UpdateDirectConnectGatewayAssociationError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::update_direct_connect_gateway_association::builders::UpdateDirectConnectGatewayAssociationOutputBuilder::default();
        output = crate::protocol_serde::shape_update_direct_connect_gateway_association::de_update_direct_connect_gateway_association(
            _response_body,
            output,
        )
        .map_err(crate::operation::update_direct_connect_gateway_association::UpdateDirectConnectGatewayAssociationError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_update_direct_connect_gateway_association_input(
    input: &crate::operation::update_direct_connect_gateway_association::UpdateDirectConnectGatewayAssociationInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_update_direct_connect_gateway_association_input::ser_update_direct_connect_gateway_association_input_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_update_direct_connect_gateway_association(
    value: &[u8],
    mut builder: crate::operation::update_direct_connect_gateway_association::builders::UpdateDirectConnectGatewayAssociationOutputBuilder,
) -> ::std::result::Result<
    crate::operation::update_direct_connect_gateway_association::builders::UpdateDirectConnectGatewayAssociationOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "directConnectGatewayAssociation" => {
                    builder = builder.set_direct_connect_gateway_association(
                        crate::protocol_serde::shape_direct_connect_gateway_association::de_direct_connect_gateway_association(tokens)?,
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
