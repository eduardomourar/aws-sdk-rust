// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_workflow_run_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::get_workflow_run::GetWorkflowRunOutput, crate::operation::get_workflow_run::GetWorkflowRunError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::get_workflow_run::GetWorkflowRunError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::get_workflow_run::GetWorkflowRunError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::operation::get_workflow_run::GetWorkflowRunError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_workflow_run::GetWorkflowRunError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::access_denied_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_workflow_run::GetWorkflowRunError::unhandled)?
            };
            tmp
        }),
        "ConflictException" => crate::operation::get_workflow_run::GetWorkflowRunError::ConflictException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ConflictExceptionBuilder::default();
                output = crate::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_workflow_run::GetWorkflowRunError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::conflict_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_workflow_run::GetWorkflowRunError::unhandled)?
            };
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::get_workflow_run::GetWorkflowRunError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_workflow_run::GetWorkflowRunError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::resource_not_found_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_workflow_run::GetWorkflowRunError::unhandled)?
            };
            tmp
        }),
        "ServiceQuotaExceededException" => crate::operation::get_workflow_run::GetWorkflowRunError::ServiceQuotaExceededException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ServiceQuotaExceededExceptionBuilder::default();
                output = crate::protocol_serde::shape_service_quota_exceeded_exception::de_service_quota_exceeded_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::get_workflow_run::GetWorkflowRunError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::service_quota_exceeded_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_workflow_run::GetWorkflowRunError::unhandled)?
            };
            tmp
        }),
        "ThrottlingException" => crate::operation::get_workflow_run::GetWorkflowRunError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_workflow_run::GetWorkflowRunError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::throttling_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_workflow_run::GetWorkflowRunError::unhandled)?
            };
            tmp
        }),
        "ValidationException" => crate::operation::get_workflow_run::GetWorkflowRunError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_workflow_run::GetWorkflowRunError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::validation_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_workflow_run::GetWorkflowRunError::unhandled)?
            };
            tmp
        }),
        _ => crate::operation::get_workflow_run::GetWorkflowRunError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_workflow_run_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::get_workflow_run::GetWorkflowRunOutput, crate::operation::get_workflow_run::GetWorkflowRunError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_workflow_run::builders::GetWorkflowRunOutputBuilder::default();
        output = crate::protocol_serde::shape_get_workflow_run::de_get_workflow_run(_response_body, output)
            .map_err(crate::operation::get_workflow_run::GetWorkflowRunError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::get_workflow_run_output_output_correct_errors(output)
            .build()
            .map_err(crate::operation::get_workflow_run::GetWorkflowRunError::unhandled)?
    })
}

pub(crate) fn de_get_workflow_run(
    value: &[u8],
    mut builder: crate::operation::get_workflow_run::builders::GetWorkflowRunOutputBuilder,
) -> ::std::result::Result<
    crate::operation::get_workflow_run::builders::GetWorkflowRunOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "endTime" => {
                    builder = builder.set_end_time(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                        tokens.next(),
                        ::aws_smithy_types::date_time::Format::DateTimeWithOffset,
                    )?);
                }
                "id" => {
                    builder = builder.set_id(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "lastUpdatedTime" => {
                    builder = builder.set_last_updated_time(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                        tokens.next(),
                        ::aws_smithy_types::date_time::Format::DateTimeWithOffset,
                    )?);
                }
                "projectName" => {
                    builder = builder.set_project_name(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "spaceName" => {
                    builder = builder.set_space_name(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "startTime" => {
                    builder = builder.set_start_time(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                        tokens.next(),
                        ::aws_smithy_types::date_time::Format::DateTimeWithOffset,
                    )?);
                }
                "status" => {
                    builder = builder.set_status(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| crate::types::WorkflowRunStatus::from(u.as_ref())))
                            .transpose()?,
                    );
                }
                "statusReasons" => {
                    builder = builder.set_status_reasons(crate::protocol_serde::shape_workflow_run_status_reasons::de_workflow_run_status_reasons(
                        tokens,
                    )?);
                }
                "workflowId" => {
                    builder = builder.set_workflow_id(
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
