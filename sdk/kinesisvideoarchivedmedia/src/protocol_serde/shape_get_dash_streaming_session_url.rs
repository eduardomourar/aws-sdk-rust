// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_dash_streaming_session_url_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_dash_streaming_session_url::GetDashStreamingSessionUrlOutput,
    crate::operation::get_dash_streaming_session_url::GetDASHStreamingSessionURLError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::get_dash_streaming_session_url::GetDASHStreamingSessionURLError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::get_dash_streaming_session_url::GetDASHStreamingSessionURLError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ClientLimitExceededException" => {
            crate::operation::get_dash_streaming_session_url::GetDASHStreamingSessionURLError::ClientLimitExceededException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ClientLimitExceededExceptionBuilder::default();
                    output = crate::protocol_serde::shape_client_limit_exceeded_exception::de_client_limit_exceeded_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::get_dash_streaming_session_url::GetDASHStreamingSessionURLError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidArgumentException" => crate::operation::get_dash_streaming_session_url::GetDASHStreamingSessionURLError::InvalidArgumentException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidArgumentExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_argument_exception::de_invalid_argument_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_dash_streaming_session_url::GetDASHStreamingSessionURLError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidCodecPrivateDataException" => {
            crate::operation::get_dash_streaming_session_url::GetDASHStreamingSessionURLError::InvalidCodecPrivateDataException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidCodecPrivateDataExceptionBuilder::default();
                    output = crate::protocol_serde::shape_invalid_codec_private_data_exception::de_invalid_codec_private_data_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::get_dash_streaming_session_url::GetDASHStreamingSessionURLError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "MissingCodecPrivateDataException" => {
            crate::operation::get_dash_streaming_session_url::GetDASHStreamingSessionURLError::MissingCodecPrivateDataException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::MissingCodecPrivateDataExceptionBuilder::default();
                    output = crate::protocol_serde::shape_missing_codec_private_data_exception::de_missing_codec_private_data_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::get_dash_streaming_session_url::GetDASHStreamingSessionURLError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "NoDataRetentionException" => crate::operation::get_dash_streaming_session_url::GetDASHStreamingSessionURLError::NoDataRetentionException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::NoDataRetentionExceptionBuilder::default();
                output = crate::protocol_serde::shape_no_data_retention_exception::de_no_data_retention_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_dash_streaming_session_url::GetDASHStreamingSessionURLError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "NotAuthorizedException" => crate::operation::get_dash_streaming_session_url::GetDASHStreamingSessionURLError::NotAuthorizedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::NotAuthorizedExceptionBuilder::default();
                output = crate::protocol_serde::shape_not_authorized_exception::de_not_authorized_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_dash_streaming_session_url::GetDASHStreamingSessionURLError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ResourceNotFoundException" => {
            crate::operation::get_dash_streaming_session_url::GetDASHStreamingSessionURLError::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                            .map_err(crate::operation::get_dash_streaming_session_url::GetDASHStreamingSessionURLError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "UnsupportedStreamMediaTypeException" => {
            crate::operation::get_dash_streaming_session_url::GetDASHStreamingSessionURLError::UnsupportedStreamMediaTypeException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::UnsupportedStreamMediaTypeExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_unsupported_stream_media_type_exception::de_unsupported_stream_media_type_exception_json_err(
                            _response_body,
                            output,
                        )
                        .map_err(crate::operation::get_dash_streaming_session_url::GetDASHStreamingSessionURLError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::get_dash_streaming_session_url::GetDASHStreamingSessionURLError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_dash_streaming_session_url_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_dash_streaming_session_url::GetDashStreamingSessionUrlOutput,
    crate::operation::get_dash_streaming_session_url::GetDASHStreamingSessionURLError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_dash_streaming_session_url::builders::GetDashStreamingSessionUrlOutputBuilder::default();
        output = crate::protocol_serde::shape_get_dash_streaming_session_url::de_get_dash_streaming_session_url(_response_body, output)
            .map_err(crate::operation::get_dash_streaming_session_url::GetDASHStreamingSessionURLError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_get_dash_streaming_session_url_input(
    input: &crate::operation::get_dash_streaming_session_url::GetDashStreamingSessionUrlInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_get_dash_streaming_session_url_input::ser_get_dash_streaming_session_url_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_get_dash_streaming_session_url(
    value: &[u8],
    mut builder: crate::operation::get_dash_streaming_session_url::builders::GetDashStreamingSessionUrlOutputBuilder,
) -> ::std::result::Result<
    crate::operation::get_dash_streaming_session_url::builders::GetDashStreamingSessionUrlOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "DASHStreamingSessionURL" => {
                    builder = builder.set_dash_streaming_session_url(
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
