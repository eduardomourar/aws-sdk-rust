// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_evaluate_pull_request_approval_rules_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::evaluate_pull_request_approval_rules::EvaluatePullRequestApprovalRulesOutput,
    crate::operation::evaluate_pull_request_approval_rules::EvaluatePullRequestApprovalRulesError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::evaluate_pull_request_approval_rules::EvaluatePullRequestApprovalRulesError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::evaluate_pull_request_approval_rules::EvaluatePullRequestApprovalRulesError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "EncryptionIntegrityChecksFailedException" => {
            crate::operation::evaluate_pull_request_approval_rules::EvaluatePullRequestApprovalRulesError::EncryptionIntegrityChecksFailedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::EncryptionIntegrityChecksFailedExceptionBuilder::default();
                    output = crate::protocol_serde::shape_encryption_integrity_checks_failed_exception::de_encryption_integrity_checks_failed_exception_json_err(_response_body, output).map_err(crate::operation::evaluate_pull_request_approval_rules::EvaluatePullRequestApprovalRulesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "EncryptionKeyAccessDeniedException" => {
            crate::operation::evaluate_pull_request_approval_rules::EvaluatePullRequestApprovalRulesError::EncryptionKeyAccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::EncryptionKeyAccessDeniedExceptionBuilder::default();
                    output = crate::protocol_serde::shape_encryption_key_access_denied_exception::de_encryption_key_access_denied_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::evaluate_pull_request_approval_rules::EvaluatePullRequestApprovalRulesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "EncryptionKeyDisabledException" => {
            crate::operation::evaluate_pull_request_approval_rules::EvaluatePullRequestApprovalRulesError::EncryptionKeyDisabledException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::EncryptionKeyDisabledExceptionBuilder::default();
                    output = crate::protocol_serde::shape_encryption_key_disabled_exception::de_encryption_key_disabled_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::evaluate_pull_request_approval_rules::EvaluatePullRequestApprovalRulesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "EncryptionKeyNotFoundException" => {
            crate::operation::evaluate_pull_request_approval_rules::EvaluatePullRequestApprovalRulesError::EncryptionKeyNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::EncryptionKeyNotFoundExceptionBuilder::default();
                    output = crate::protocol_serde::shape_encryption_key_not_found_exception::de_encryption_key_not_found_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::evaluate_pull_request_approval_rules::EvaluatePullRequestApprovalRulesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "EncryptionKeyUnavailableException" => {
            crate::operation::evaluate_pull_request_approval_rules::EvaluatePullRequestApprovalRulesError::EncryptionKeyUnavailableException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::EncryptionKeyUnavailableExceptionBuilder::default();
                    output = crate::protocol_serde::shape_encryption_key_unavailable_exception::de_encryption_key_unavailable_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::evaluate_pull_request_approval_rules::EvaluatePullRequestApprovalRulesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidPullRequestIdException" => {
            crate::operation::evaluate_pull_request_approval_rules::EvaluatePullRequestApprovalRulesError::InvalidPullRequestIdException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidPullRequestIdExceptionBuilder::default();
                    output = crate::protocol_serde::shape_invalid_pull_request_id_exception::de_invalid_pull_request_id_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::evaluate_pull_request_approval_rules::EvaluatePullRequestApprovalRulesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidRevisionIdException" => {
            crate::operation::evaluate_pull_request_approval_rules::EvaluatePullRequestApprovalRulesError::InvalidRevisionIdException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidRevisionIdExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_invalid_revision_id_exception::de_invalid_revision_id_exception_json_err(_response_body, output)
                            .map_err(crate::operation::evaluate_pull_request_approval_rules::EvaluatePullRequestApprovalRulesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "PullRequestDoesNotExistException" => {
            crate::operation::evaluate_pull_request_approval_rules::EvaluatePullRequestApprovalRulesError::PullRequestDoesNotExistException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::PullRequestDoesNotExistExceptionBuilder::default();
                    output = crate::protocol_serde::shape_pull_request_does_not_exist_exception::de_pull_request_does_not_exist_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::evaluate_pull_request_approval_rules::EvaluatePullRequestApprovalRulesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "PullRequestIdRequiredException" => {
            crate::operation::evaluate_pull_request_approval_rules::EvaluatePullRequestApprovalRulesError::PullRequestIdRequiredException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::PullRequestIdRequiredExceptionBuilder::default();
                    output = crate::protocol_serde::shape_pull_request_id_required_exception::de_pull_request_id_required_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::evaluate_pull_request_approval_rules::EvaluatePullRequestApprovalRulesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "RevisionIdRequiredException" => {
            crate::operation::evaluate_pull_request_approval_rules::EvaluatePullRequestApprovalRulesError::RevisionIdRequiredException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::RevisionIdRequiredExceptionBuilder::default();
                    output = crate::protocol_serde::shape_revision_id_required_exception::de_revision_id_required_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::evaluate_pull_request_approval_rules::EvaluatePullRequestApprovalRulesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "RevisionNotCurrentException" => {
            crate::operation::evaluate_pull_request_approval_rules::EvaluatePullRequestApprovalRulesError::RevisionNotCurrentException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::RevisionNotCurrentExceptionBuilder::default();
                    output = crate::protocol_serde::shape_revision_not_current_exception::de_revision_not_current_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::evaluate_pull_request_approval_rules::EvaluatePullRequestApprovalRulesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::evaluate_pull_request_approval_rules::EvaluatePullRequestApprovalRulesError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_evaluate_pull_request_approval_rules_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::evaluate_pull_request_approval_rules::EvaluatePullRequestApprovalRulesOutput,
    crate::operation::evaluate_pull_request_approval_rules::EvaluatePullRequestApprovalRulesError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::evaluate_pull_request_approval_rules::builders::EvaluatePullRequestApprovalRulesOutputBuilder::default();
        output = crate::protocol_serde::shape_evaluate_pull_request_approval_rules::de_evaluate_pull_request_approval_rules(_response_body, output)
            .map_err(crate::operation::evaluate_pull_request_approval_rules::EvaluatePullRequestApprovalRulesError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::evaluate_pull_request_approval_rules_output_output_correct_errors(output).build()
    })
}

pub fn ser_evaluate_pull_request_approval_rules_input(
    input: &crate::operation::evaluate_pull_request_approval_rules::EvaluatePullRequestApprovalRulesInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_evaluate_pull_request_approval_rules_input::ser_evaluate_pull_request_approval_rules_input_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_evaluate_pull_request_approval_rules(
    value: &[u8],
    mut builder: crate::operation::evaluate_pull_request_approval_rules::builders::EvaluatePullRequestApprovalRulesOutputBuilder,
) -> ::std::result::Result<
    crate::operation::evaluate_pull_request_approval_rules::builders::EvaluatePullRequestApprovalRulesOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "evaluation" => {
                    builder = builder.set_evaluation(crate::protocol_serde::shape_evaluation::de_evaluation(tokens)?);
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
