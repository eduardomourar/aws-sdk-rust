// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_batch_delete_category_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::batch_delete_category::BatchDeleteCategoryOutput,
    crate::operation::batch_delete_category::BatchDeleteCategoryError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::batch_delete_category::BatchDeleteCategoryError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::batch_delete_category::BatchDeleteCategoryError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::operation::batch_delete_category::BatchDeleteCategoryError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                    .map_err(crate::operation::batch_delete_category::BatchDeleteCategoryError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::access_denied_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::batch_delete_category::BatchDeleteCategoryError::unhandled)?
            };
            tmp
        }),
        "ConflictException" => crate::operation::batch_delete_category::BatchDeleteCategoryError::ConflictException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ConflictExceptionBuilder::default();
                output = crate::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output)
                    .map_err(crate::operation::batch_delete_category::BatchDeleteCategoryError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::conflict_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::batch_delete_category::BatchDeleteCategoryError::unhandled)?
            };
            tmp
        }),
        "InternalServerException" => crate::operation::batch_delete_category::BatchDeleteCategoryError::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                    .map_err(crate::operation::batch_delete_category::BatchDeleteCategoryError::unhandled)?;
                output = output.set_retry_after_seconds(
                    crate::protocol_serde::shape_internal_server_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
                        crate::operation::batch_delete_category::BatchDeleteCategoryError::unhandled(
                            "Failed to parse retryAfterSeconds from header `Retry-After",
                        )
                    })?,
                );
                let output = output.meta(generic);
                crate::serde_util::internal_server_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::batch_delete_category::BatchDeleteCategoryError::unhandled)?
            };
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::batch_delete_category::BatchDeleteCategoryError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::batch_delete_category::BatchDeleteCategoryError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::resource_not_found_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::batch_delete_category::BatchDeleteCategoryError::unhandled)?
            };
            tmp
        }),
        "ThrottlingException" => crate::operation::batch_delete_category::BatchDeleteCategoryError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                    .map_err(crate::operation::batch_delete_category::BatchDeleteCategoryError::unhandled)?;
                output = output.set_retry_after_seconds(
                    crate::protocol_serde::shape_throttling_exception::de_retry_after_seconds_header(_response_headers).map_err(|_| {
                        crate::operation::batch_delete_category::BatchDeleteCategoryError::unhandled(
                            "Failed to parse retryAfterSeconds from header `Retry-After",
                        )
                    })?,
                );
                let output = output.meta(generic);
                crate::serde_util::throttling_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::batch_delete_category::BatchDeleteCategoryError::unhandled)?
            };
            tmp
        }),
        "UnauthorizedException" => crate::operation::batch_delete_category::BatchDeleteCategoryError::UnauthorizedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::UnauthorizedExceptionBuilder::default();
                output = crate::protocol_serde::shape_unauthorized_exception::de_unauthorized_exception_json_err(_response_body, output)
                    .map_err(crate::operation::batch_delete_category::BatchDeleteCategoryError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::unauthorized_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::batch_delete_category::BatchDeleteCategoryError::unhandled)?
            };
            tmp
        }),
        "ValidationException" => crate::operation::batch_delete_category::BatchDeleteCategoryError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                    .map_err(crate::operation::batch_delete_category::BatchDeleteCategoryError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::validation_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::batch_delete_category::BatchDeleteCategoryError::unhandled)?
            };
            tmp
        }),
        _ => crate::operation::batch_delete_category::BatchDeleteCategoryError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_batch_delete_category_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::batch_delete_category::BatchDeleteCategoryOutput,
    crate::operation::batch_delete_category::BatchDeleteCategoryError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::batch_delete_category::builders::BatchDeleteCategoryOutputBuilder::default();
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_batch_delete_category_headers(
    input: &crate::operation::batch_delete_category::BatchDeleteCategoryInput,
    mut builder: ::http::request::Builder,
) -> std::result::Result<::http::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
    if let ::std::option::Option::Some(inner_1) = &input.instance_id {
        let formatted_2 = inner_1.as_str();
        let header_value = formatted_2;
        let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
            ::aws_smithy_types::error::operation::BuildError::invalid_field(
                "instance_id",
                format!("`{}` cannot be used as a header value: {}", &header_value, err),
            )
        })?;
        builder = builder.header("instance-id", header_value);
    }
    Ok(builder)
}

pub fn ser_batch_delete_category_input(
    input: &crate::operation::batch_delete_category::BatchDeleteCategoryInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_batch_delete_category_input::ser_batch_delete_category_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
