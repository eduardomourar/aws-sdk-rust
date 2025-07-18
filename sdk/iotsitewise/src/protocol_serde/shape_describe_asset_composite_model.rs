// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_asset_composite_model_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_asset_composite_model::DescribeAssetCompositeModelOutput,
    crate::operation::describe_asset_composite_model::DescribeAssetCompositeModelError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::describe_asset_composite_model::DescribeAssetCompositeModelError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::describe_asset_composite_model::DescribeAssetCompositeModelError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalFailureException" => crate::operation::describe_asset_composite_model::DescribeAssetCompositeModelError::InternalFailureException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalFailureExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_failure_exception::de_internal_failure_exception_json_err(_response_body, output)
                    .map_err(crate::operation::describe_asset_composite_model::DescribeAssetCompositeModelError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::internal_failure_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::describe_asset_composite_model::DescribeAssetCompositeModelError::unhandled)?
            };
            tmp
        }),
        "InvalidRequestException" => crate::operation::describe_asset_composite_model::DescribeAssetCompositeModelError::InvalidRequestException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output)
                    .map_err(crate::operation::describe_asset_composite_model::DescribeAssetCompositeModelError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::invalid_request_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::describe_asset_composite_model::DescribeAssetCompositeModelError::unhandled)?
            };
            tmp
        }),
        "ResourceNotFoundException" => {
            crate::operation::describe_asset_composite_model::DescribeAssetCompositeModelError::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                            .map_err(crate::operation::describe_asset_composite_model::DescribeAssetCompositeModelError::unhandled)?;
                    let output = output.meta(generic);
                    crate::serde_util::resource_not_found_exception_correct_errors(output)
                        .build()
                        .map_err(crate::operation::describe_asset_composite_model::DescribeAssetCompositeModelError::unhandled)?
                };
                tmp
            })
        }
        "ThrottlingException" => crate::operation::describe_asset_composite_model::DescribeAssetCompositeModelError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                    .map_err(crate::operation::describe_asset_composite_model::DescribeAssetCompositeModelError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::throttling_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::describe_asset_composite_model::DescribeAssetCompositeModelError::unhandled)?
            };
            tmp
        }),
        _ => crate::operation::describe_asset_composite_model::DescribeAssetCompositeModelError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_asset_composite_model_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_asset_composite_model::DescribeAssetCompositeModelOutput,
    crate::operation::describe_asset_composite_model::DescribeAssetCompositeModelError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_asset_composite_model::builders::DescribeAssetCompositeModelOutputBuilder::default();
        output = crate::protocol_serde::shape_describe_asset_composite_model::de_describe_asset_composite_model(_response_body, output)
            .map_err(crate::operation::describe_asset_composite_model::DescribeAssetCompositeModelError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::describe_asset_composite_model_output_output_correct_errors(output)
            .build()
            .map_err(crate::operation::describe_asset_composite_model::DescribeAssetCompositeModelError::unhandled)?
    })
}

pub(crate) fn de_describe_asset_composite_model(
    value: &[u8],
    mut builder: crate::operation::describe_asset_composite_model::builders::DescribeAssetCompositeModelOutputBuilder,
) -> ::std::result::Result<
    crate::operation::describe_asset_composite_model::builders::DescribeAssetCompositeModelOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "actionDefinitions" => {
                    builder = builder.set_action_definitions(crate::protocol_serde::shape_action_definitions::de_action_definitions(tokens)?);
                }
                "assetCompositeModelDescription" => {
                    builder = builder.set_asset_composite_model_description(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "assetCompositeModelExternalId" => {
                    builder = builder.set_asset_composite_model_external_id(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "assetCompositeModelId" => {
                    builder = builder.set_asset_composite_model_id(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "assetCompositeModelName" => {
                    builder = builder.set_asset_composite_model_name(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "assetCompositeModelPath" => {
                    builder = builder.set_asset_composite_model_path(
                        crate::protocol_serde::shape_asset_composite_model_path::de_asset_composite_model_path(tokens)?,
                    );
                }
                "assetCompositeModelProperties" => {
                    builder =
                        builder.set_asset_composite_model_properties(crate::protocol_serde::shape_asset_properties::de_asset_properties(tokens)?);
                }
                "assetCompositeModelSummaries" => {
                    builder = builder.set_asset_composite_model_summaries(
                        crate::protocol_serde::shape_asset_composite_model_summaries::de_asset_composite_model_summaries(tokens)?,
                    );
                }
                "assetCompositeModelType" => {
                    builder = builder.set_asset_composite_model_type(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "assetId" => {
                    builder = builder.set_asset_id(
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
