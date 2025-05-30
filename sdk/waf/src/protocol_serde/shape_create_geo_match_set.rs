// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_create_geo_match_set_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::create_geo_match_set::CreateGeoMatchSetOutput,
    crate::operation::create_geo_match_set::CreateGeoMatchSetError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::create_geo_match_set::CreateGeoMatchSetError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::create_geo_match_set::CreateGeoMatchSetError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "WAFDisallowedNameException" => crate::operation::create_geo_match_set::CreateGeoMatchSetError::WafDisallowedNameException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::WafDisallowedNameExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_waf_disallowed_name_exception::de_waf_disallowed_name_exception_json_err(_response_body, output)
                        .map_err(crate::operation::create_geo_match_set::CreateGeoMatchSetError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "WAFInternalErrorException" => crate::operation::create_geo_match_set::CreateGeoMatchSetError::WafInternalErrorException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::WafInternalErrorExceptionBuilder::default();
                output = crate::protocol_serde::shape_waf_internal_error_exception::de_waf_internal_error_exception_json_err(_response_body, output)
                    .map_err(crate::operation::create_geo_match_set::CreateGeoMatchSetError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "WAFInvalidAccountException" => crate::operation::create_geo_match_set::CreateGeoMatchSetError::WafInvalidAccountException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::WafInvalidAccountExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_waf_invalid_account_exception::de_waf_invalid_account_exception_json_err(_response_body, output)
                        .map_err(crate::operation::create_geo_match_set::CreateGeoMatchSetError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "WAFInvalidParameterException" => crate::operation::create_geo_match_set::CreateGeoMatchSetError::WafInvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::WafInvalidParameterExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_waf_invalid_parameter_exception::de_waf_invalid_parameter_exception_json_err(_response_body, output)
                        .map_err(crate::operation::create_geo_match_set::CreateGeoMatchSetError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "WAFLimitsExceededException" => crate::operation::create_geo_match_set::CreateGeoMatchSetError::WafLimitsExceededException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::WafLimitsExceededExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_waf_limits_exceeded_exception::de_waf_limits_exceeded_exception_json_err(_response_body, output)
                        .map_err(crate::operation::create_geo_match_set::CreateGeoMatchSetError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "WAFStaleDataException" => crate::operation::create_geo_match_set::CreateGeoMatchSetError::WafStaleDataException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::WafStaleDataExceptionBuilder::default();
                output = crate::protocol_serde::shape_waf_stale_data_exception::de_waf_stale_data_exception_json_err(_response_body, output)
                    .map_err(crate::operation::create_geo_match_set::CreateGeoMatchSetError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::create_geo_match_set::CreateGeoMatchSetError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_geo_match_set_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::create_geo_match_set::CreateGeoMatchSetOutput,
    crate::operation::create_geo_match_set::CreateGeoMatchSetError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::create_geo_match_set::builders::CreateGeoMatchSetOutputBuilder::default();
        output = crate::protocol_serde::shape_create_geo_match_set::de_create_geo_match_set(_response_body, output)
            .map_err(crate::operation::create_geo_match_set::CreateGeoMatchSetError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_create_geo_match_set_input(
    input: &crate::operation::create_geo_match_set::CreateGeoMatchSetInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_create_geo_match_set_input::ser_create_geo_match_set_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_create_geo_match_set(
    value: &[u8],
    mut builder: crate::operation::create_geo_match_set::builders::CreateGeoMatchSetOutputBuilder,
) -> ::std::result::Result<
    crate::operation::create_geo_match_set::builders::CreateGeoMatchSetOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "GeoMatchSet" => {
                    builder = builder.set_geo_match_set(crate::protocol_serde::shape_geo_match_set::de_geo_match_set(tokens)?);
                }
                "ChangeToken" => {
                    builder = builder.set_change_token(
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
