// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_cost_and_usage_comparisons_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_cost_and_usage_comparisons::GetCostAndUsageComparisonsOutput,
    crate::operation::get_cost_and_usage_comparisons::GetCostAndUsageComparisonsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::get_cost_and_usage_comparisons::GetCostAndUsageComparisonsError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::get_cost_and_usage_comparisons::GetCostAndUsageComparisonsError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "DataUnavailableException" => crate::operation::get_cost_and_usage_comparisons::GetCostAndUsageComparisonsError::DataUnavailableException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::DataUnavailableExceptionBuilder::default();
                output = crate::protocol_serde::shape_data_unavailable_exception::de_data_unavailable_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_cost_and_usage_comparisons::GetCostAndUsageComparisonsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidNextTokenException" => {
            crate::operation::get_cost_and_usage_comparisons::GetCostAndUsageComparisonsError::InvalidNextTokenException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidNextTokenExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_invalid_next_token_exception::de_invalid_next_token_exception_json_err(_response_body, output)
                            .map_err(crate::operation::get_cost_and_usage_comparisons::GetCostAndUsageComparisonsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "LimitExceededException" => crate::operation::get_cost_and_usage_comparisons::GetCostAndUsageComparisonsError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::LimitExceededExceptionBuilder::default();
                output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_cost_and_usage_comparisons::GetCostAndUsageComparisonsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ResourceNotFoundException" => {
            crate::operation::get_cost_and_usage_comparisons::GetCostAndUsageComparisonsError::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                            .map_err(crate::operation::get_cost_and_usage_comparisons::GetCostAndUsageComparisonsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::get_cost_and_usage_comparisons::GetCostAndUsageComparisonsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_cost_and_usage_comparisons_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_cost_and_usage_comparisons::GetCostAndUsageComparisonsOutput,
    crate::operation::get_cost_and_usage_comparisons::GetCostAndUsageComparisonsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_cost_and_usage_comparisons::builders::GetCostAndUsageComparisonsOutputBuilder::default();
        output = crate::protocol_serde::shape_get_cost_and_usage_comparisons::de_get_cost_and_usage_comparisons(_response_body, output)
            .map_err(crate::operation::get_cost_and_usage_comparisons::GetCostAndUsageComparisonsError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_get_cost_and_usage_comparisons_input(
    input: &crate::operation::get_cost_and_usage_comparisons::GetCostAndUsageComparisonsInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_get_cost_and_usage_comparisons_input::ser_get_cost_and_usage_comparisons_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_get_cost_and_usage_comparisons(
    value: &[u8],
    mut builder: crate::operation::get_cost_and_usage_comparisons::builders::GetCostAndUsageComparisonsOutputBuilder,
) -> ::std::result::Result<
    crate::operation::get_cost_and_usage_comparisons::builders::GetCostAndUsageComparisonsOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "CostAndUsageComparisons" => {
                    builder = builder.set_cost_and_usage_comparisons(
                        crate::protocol_serde::shape_cost_and_usage_comparisons::de_cost_and_usage_comparisons(tokens)?,
                    );
                }
                "TotalCostAndUsage" => {
                    builder = builder.set_total_cost_and_usage(crate::protocol_serde::shape_comparison_metrics::de_comparison_metrics(tokens)?);
                }
                "NextPageToken" => {
                    builder = builder.set_next_page_token(
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
