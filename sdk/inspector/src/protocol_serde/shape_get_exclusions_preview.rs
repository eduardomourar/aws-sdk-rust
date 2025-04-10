// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_exclusions_preview_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_exclusions_preview::GetExclusionsPreviewOutput,
    crate::operation::get_exclusions_preview::GetExclusionsPreviewError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::get_exclusions_preview::GetExclusionsPreviewError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::get_exclusions_preview::GetExclusionsPreviewError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::operation::get_exclusions_preview::GetExclusionsPreviewError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_exclusions_preview::GetExclusionsPreviewError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::access_denied_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_exclusions_preview::GetExclusionsPreviewError::unhandled)?
            };
            tmp
        }),
        "InternalException" => crate::operation::get_exclusions_preview::GetExclusionsPreviewError::InternalException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_exception::de_internal_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_exclusions_preview::GetExclusionsPreviewError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::internal_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_exclusions_preview::GetExclusionsPreviewError::unhandled)?
            };
            tmp
        }),
        "InvalidInputException" => crate::operation::get_exclusions_preview::GetExclusionsPreviewError::InvalidInputException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidInputExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_input_exception::de_invalid_input_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_exclusions_preview::GetExclusionsPreviewError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::invalid_input_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_exclusions_preview::GetExclusionsPreviewError::unhandled)?
            };
            tmp
        }),
        "NoSuchEntityException" => crate::operation::get_exclusions_preview::GetExclusionsPreviewError::NoSuchEntityException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::NoSuchEntityExceptionBuilder::default();
                output = crate::protocol_serde::shape_no_such_entity_exception::de_no_such_entity_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_exclusions_preview::GetExclusionsPreviewError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::no_such_entity_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_exclusions_preview::GetExclusionsPreviewError::unhandled)?
            };
            tmp
        }),
        _ => crate::operation::get_exclusions_preview::GetExclusionsPreviewError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_exclusions_preview_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_exclusions_preview::GetExclusionsPreviewOutput,
    crate::operation::get_exclusions_preview::GetExclusionsPreviewError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_exclusions_preview::builders::GetExclusionsPreviewOutputBuilder::default();
        output = crate::protocol_serde::shape_get_exclusions_preview::de_get_exclusions_preview(_response_body, output)
            .map_err(crate::operation::get_exclusions_preview::GetExclusionsPreviewError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::get_exclusions_preview_output_output_correct_errors(output)
            .build()
            .map_err(crate::operation::get_exclusions_preview::GetExclusionsPreviewError::unhandled)?
    })
}

pub fn ser_get_exclusions_preview_input(
    input: &crate::operation::get_exclusions_preview::GetExclusionsPreviewInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_get_exclusions_preview_input::ser_get_exclusions_preview_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_get_exclusions_preview(
    value: &[u8],
    mut builder: crate::operation::get_exclusions_preview::builders::GetExclusionsPreviewOutputBuilder,
) -> ::std::result::Result<
    crate::operation::get_exclusions_preview::builders::GetExclusionsPreviewOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "previewStatus" => {
                    builder = builder.set_preview_status(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| crate::types::PreviewStatus::from(u.as_ref())))
                            .transpose()?,
                    );
                }
                "exclusionPreviews" => {
                    builder = builder.set_exclusion_previews(crate::protocol_serde::shape_exclusion_preview_list::de_exclusion_preview_list(tokens)?);
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
