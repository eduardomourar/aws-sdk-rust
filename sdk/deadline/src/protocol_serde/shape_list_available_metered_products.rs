// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_list_available_metered_products_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::list_available_metered_products::ListAvailableMeteredProductsOutput,
    crate::operation::list_available_metered_products::ListAvailableMeteredProductsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::list_available_metered_products::ListAvailableMeteredProductsError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::list_available_metered_products::ListAvailableMeteredProductsError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServerErrorException" => {
            crate::operation::list_available_metered_products::ListAvailableMeteredProductsError::InternalServerErrorException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServerErrorExceptionBuilder::default();
                    output = crate::protocol_serde::shape_internal_server_error_exception::de_internal_server_error_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::list_available_metered_products::ListAvailableMeteredProductsError::unhandled)?;
                    output = output.set_retry_after_seconds(
                        crate::protocol_serde::shape_internal_server_error_exception::de_retry_after_seconds_header(_response_headers).map_err(
                            |_| {
                                crate::operation::list_available_metered_products::ListAvailableMeteredProductsError::unhandled(
                                    "Failed to parse retryAfterSeconds from header `Retry-After",
                                )
                            },
                        )?,
                    );
                    let output = output.meta(generic);
                    crate::serde_util::internal_server_error_exception_correct_errors(output)
                        .build()
                        .map_err(crate::operation::list_available_metered_products::ListAvailableMeteredProductsError::unhandled)?
                };
                tmp
            })
        }
        "ThrottlingException" => crate::operation::list_available_metered_products::ListAvailableMeteredProductsError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                    .map_err(crate::operation::list_available_metered_products::ListAvailableMeteredProductsError::unhandled)?;
                output = output.set_retry_after_seconds(
                    crate::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
                        crate::operation::list_available_metered_products::ListAvailableMeteredProductsError::unhandled(
                            "Failed to parse retryAfterSeconds from header `Retry-After",
                        )
                    })?,
                );
                let output = output.meta(generic);
                crate::serde_util::throttling_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::list_available_metered_products::ListAvailableMeteredProductsError::unhandled)?
            };
            tmp
        }),
        _ => crate::operation::list_available_metered_products::ListAvailableMeteredProductsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_available_metered_products_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::list_available_metered_products::ListAvailableMeteredProductsOutput,
    crate::operation::list_available_metered_products::ListAvailableMeteredProductsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::list_available_metered_products::builders::ListAvailableMeteredProductsOutputBuilder::default();
        output = crate::protocol_serde::shape_list_available_metered_products::de_list_available_metered_products(_response_body, output)
            .map_err(crate::operation::list_available_metered_products::ListAvailableMeteredProductsError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::list_available_metered_products_output_output_correct_errors(output)
            .build()
            .map_err(crate::operation::list_available_metered_products::ListAvailableMeteredProductsError::unhandled)?
    })
}

pub(crate) fn de_list_available_metered_products(
    value: &[u8],
    mut builder: crate::operation::list_available_metered_products::builders::ListAvailableMeteredProductsOutputBuilder,
) -> ::std::result::Result<
    crate::operation::list_available_metered_products::builders::ListAvailableMeteredProductsOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "meteredProducts" => {
                    builder = builder
                        .set_metered_products(crate::protocol_serde::shape_metered_product_summary_list::de_metered_product_summary_list(tokens)?);
                }
                "nextToken" => {
                    builder = builder.set_next_token(
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
