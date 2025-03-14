// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_list_server_neighbors_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::list_server_neighbors::ListServerNeighborsOutput,
    crate::operation::list_server_neighbors::ListServerNeighborsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::list_server_neighbors::ListServerNeighborsError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::list_server_neighbors::ListServerNeighborsError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AuthorizationErrorException" => crate::operation::list_server_neighbors::ListServerNeighborsError::AuthorizationErrorException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::AuthorizationErrorExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_authorization_error_exception::de_authorization_error_exception_json_err(_response_body, output)
                        .map_err(crate::operation::list_server_neighbors::ListServerNeighborsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "HomeRegionNotSetException" => crate::operation::list_server_neighbors::ListServerNeighborsError::HomeRegionNotSetException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::HomeRegionNotSetExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_home_region_not_set_exception::de_home_region_not_set_exception_json_err(_response_body, output)
                        .map_err(crate::operation::list_server_neighbors::ListServerNeighborsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidParameterException" => crate::operation::list_server_neighbors::ListServerNeighborsError::InvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidParameterExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(_response_body, output)
                    .map_err(crate::operation::list_server_neighbors::ListServerNeighborsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidParameterValueException" => crate::operation::list_server_neighbors::ListServerNeighborsError::InvalidParameterValueException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidParameterValueExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::list_server_neighbors::ListServerNeighborsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ServerInternalErrorException" => crate::operation::list_server_neighbors::ListServerNeighborsError::ServerInternalErrorException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ServerInternalErrorExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_server_internal_error_exception::de_server_internal_error_exception_json_err(_response_body, output)
                        .map_err(crate::operation::list_server_neighbors::ListServerNeighborsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::list_server_neighbors::ListServerNeighborsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_server_neighbors_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::list_server_neighbors::ListServerNeighborsOutput,
    crate::operation::list_server_neighbors::ListServerNeighborsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::list_server_neighbors::builders::ListServerNeighborsOutputBuilder::default();
        output = crate::protocol_serde::shape_list_server_neighbors::de_list_server_neighbors(_response_body, output)
            .map_err(crate::operation::list_server_neighbors::ListServerNeighborsError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::list_server_neighbors_output_output_correct_errors(output)
            .build()
            .map_err(crate::operation::list_server_neighbors::ListServerNeighborsError::unhandled)?
    })
}

pub fn ser_list_server_neighbors_input(
    input: &crate::operation::list_server_neighbors::ListServerNeighborsInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_list_server_neighbors_input::ser_list_server_neighbors_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_list_server_neighbors(
    value: &[u8],
    mut builder: crate::operation::list_server_neighbors::builders::ListServerNeighborsOutputBuilder,
) -> ::std::result::Result<
    crate::operation::list_server_neighbors::builders::ListServerNeighborsOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "neighbors" => {
                    builder = builder.set_neighbors(crate::protocol_serde::shape_neighbor_details_list::de_neighbor_details_list(tokens)?);
                }
                "nextToken" => {
                    builder = builder.set_next_token(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "knownDependencyCount" => {
                    builder = builder.set_known_dependency_count(
                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                            .map(i64::try_from)
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
