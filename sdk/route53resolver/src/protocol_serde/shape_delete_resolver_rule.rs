// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_resolver_rule_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_resolver_rule::DeleteResolverRuleOutput,
    crate::operation::delete_resolver_rule::DeleteResolverRuleError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::delete_resolver_rule::DeleteResolverRuleError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::delete_resolver_rule::DeleteResolverRuleError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServiceErrorException" => crate::operation::delete_resolver_rule::DeleteResolverRuleError::InternalServiceErrorException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalServiceErrorExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_service_error_exception::de_internal_service_error_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::delete_resolver_rule::DeleteResolverRuleError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidParameterException" => crate::operation::delete_resolver_rule::DeleteResolverRuleError::InvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidParameterExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(_response_body, output)
                    .map_err(crate::operation::delete_resolver_rule::DeleteResolverRuleError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::invalid_parameter_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::delete_resolver_rule::DeleteResolverRuleError::unhandled)?
            };
            tmp
        }),
        "ResourceInUseException" => crate::operation::delete_resolver_rule::DeleteResolverRuleError::ResourceInUseException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceInUseExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_in_use_exception::de_resource_in_use_exception_json_err(_response_body, output)
                    .map_err(crate::operation::delete_resolver_rule::DeleteResolverRuleError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::delete_resolver_rule::DeleteResolverRuleError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::delete_resolver_rule::DeleteResolverRuleError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ThrottlingException" => crate::operation::delete_resolver_rule::DeleteResolverRuleError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                    .map_err(crate::operation::delete_resolver_rule::DeleteResolverRuleError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::delete_resolver_rule::DeleteResolverRuleError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_resolver_rule_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_resolver_rule::DeleteResolverRuleOutput,
    crate::operation::delete_resolver_rule::DeleteResolverRuleError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_resolver_rule::builders::DeleteResolverRuleOutputBuilder::default();
        output = crate::protocol_serde::shape_delete_resolver_rule::de_delete_resolver_rule(_response_body, output)
            .map_err(crate::operation::delete_resolver_rule::DeleteResolverRuleError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_delete_resolver_rule_input(
    input: &crate::operation::delete_resolver_rule::DeleteResolverRuleInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_delete_resolver_rule_input::ser_delete_resolver_rule_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_delete_resolver_rule(
    value: &[u8],
    mut builder: crate::operation::delete_resolver_rule::builders::DeleteResolverRuleOutputBuilder,
) -> ::std::result::Result<
    crate::operation::delete_resolver_rule::builders::DeleteResolverRuleOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "ResolverRule" => {
                    builder = builder.set_resolver_rule(crate::protocol_serde::shape_resolver_rule::de_resolver_rule(tokens)?);
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
