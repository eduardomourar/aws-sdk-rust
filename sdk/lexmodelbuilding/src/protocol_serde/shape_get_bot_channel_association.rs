// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_bot_channel_association_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_bot_channel_association::GetBotChannelAssociationOutput,
    crate::operation::get_bot_channel_association::GetBotChannelAssociationError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::get_bot_channel_association::GetBotChannelAssociationError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::get_bot_channel_association::GetBotChannelAssociationError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "BadRequestException" => crate::operation::get_bot_channel_association::GetBotChannelAssociationError::BadRequestException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::BadRequestExceptionBuilder::default();
                output = crate::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_bot_channel_association::GetBotChannelAssociationError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InternalFailureException" => crate::operation::get_bot_channel_association::GetBotChannelAssociationError::InternalFailureException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalFailureExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_failure_exception::de_internal_failure_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_bot_channel_association::GetBotChannelAssociationError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "LimitExceededException" => crate::operation::get_bot_channel_association::GetBotChannelAssociationError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::LimitExceededExceptionBuilder::default();
                output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_bot_channel_association::GetBotChannelAssociationError::unhandled)?;
                output = output.set_retry_after_seconds(
                    crate::protocol_serde::shape_limit_exceeded_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
                        crate::operation::get_bot_channel_association::GetBotChannelAssociationError::unhandled(
                            "Failed to parse retryAfterSeconds from header `Retry-After",
                        )
                    })?,
                );
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "NotFoundException" => crate::operation::get_bot_channel_association::GetBotChannelAssociationError::NotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::NotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_bot_channel_association::GetBotChannelAssociationError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::get_bot_channel_association::GetBotChannelAssociationError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_bot_channel_association_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_bot_channel_association::GetBotChannelAssociationOutput,
    crate::operation::get_bot_channel_association::GetBotChannelAssociationError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_bot_channel_association::builders::GetBotChannelAssociationOutputBuilder::default();
        output = crate::protocol_serde::shape_get_bot_channel_association::de_get_bot_channel_association(_response_body, output)
            .map_err(crate::operation::get_bot_channel_association::GetBotChannelAssociationError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_get_bot_channel_association(
    value: &[u8],
    mut builder: crate::operation::get_bot_channel_association::builders::GetBotChannelAssociationOutputBuilder,
) -> ::std::result::Result<
    crate::operation::get_bot_channel_association::builders::GetBotChannelAssociationOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "botAlias" => {
                    builder = builder.set_bot_alias(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "botConfiguration" => {
                    builder = builder.set_bot_configuration(crate::protocol_serde::shape_channel_configuration_map::de_channel_configuration_map(
                        tokens,
                    )?);
                }
                "botName" => {
                    builder = builder.set_bot_name(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "createdDate" => {
                    builder = builder.set_created_date(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                        tokens.next(),
                        ::aws_smithy_types::date_time::Format::EpochSeconds,
                    )?);
                }
                "description" => {
                    builder = builder.set_description(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "failureReason" => {
                    builder = builder.set_failure_reason(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "name" => {
                    builder = builder.set_name(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "status" => {
                    builder = builder.set_status(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| crate::types::ChannelStatus::from(u.as_ref())))
                            .transpose()?,
                    );
                }
                "type" => {
                    builder = builder.set_type(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| crate::types::ChannelType::from(u.as_ref())))
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
