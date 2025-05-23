// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_update_state_machine_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::update_state_machine::UpdateStateMachineOutput,
    crate::operation::update_state_machine::UpdateStateMachineError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::update_state_machine::UpdateStateMachineError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::update_state_machine::UpdateStateMachineError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ConflictException" => crate::operation::update_state_machine::UpdateStateMachineError::ConflictException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ConflictExceptionBuilder::default();
                output = crate::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output)
                    .map_err(crate::operation::update_state_machine::UpdateStateMachineError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidArn" => crate::operation::update_state_machine::UpdateStateMachineError::InvalidArn({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidArnBuilder::default();
                output = crate::protocol_serde::shape_invalid_arn::de_invalid_arn_json_err(_response_body, output)
                    .map_err(crate::operation::update_state_machine::UpdateStateMachineError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidDefinition" => crate::operation::update_state_machine::UpdateStateMachineError::InvalidDefinition({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidDefinitionBuilder::default();
                output = crate::protocol_serde::shape_invalid_definition::de_invalid_definition_json_err(_response_body, output)
                    .map_err(crate::operation::update_state_machine::UpdateStateMachineError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidEncryptionConfiguration" => crate::operation::update_state_machine::UpdateStateMachineError::InvalidEncryptionConfiguration({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidEncryptionConfigurationBuilder::default();
                output = crate::protocol_serde::shape_invalid_encryption_configuration::de_invalid_encryption_configuration_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::update_state_machine::UpdateStateMachineError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidLoggingConfiguration" => crate::operation::update_state_machine::UpdateStateMachineError::InvalidLoggingConfiguration({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidLoggingConfigurationBuilder::default();
                output =
                    crate::protocol_serde::shape_invalid_logging_configuration::de_invalid_logging_configuration_json_err(_response_body, output)
                        .map_err(crate::operation::update_state_machine::UpdateStateMachineError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidTracingConfiguration" => crate::operation::update_state_machine::UpdateStateMachineError::InvalidTracingConfiguration({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidTracingConfigurationBuilder::default();
                output =
                    crate::protocol_serde::shape_invalid_tracing_configuration::de_invalid_tracing_configuration_json_err(_response_body, output)
                        .map_err(crate::operation::update_state_machine::UpdateStateMachineError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "KmsAccessDeniedException" => crate::operation::update_state_machine::UpdateStateMachineError::KmsAccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::KmsAccessDeniedExceptionBuilder::default();
                output = crate::protocol_serde::shape_kms_access_denied_exception::de_kms_access_denied_exception_json_err(_response_body, output)
                    .map_err(crate::operation::update_state_machine::UpdateStateMachineError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "KmsThrottlingException" => crate::operation::update_state_machine::UpdateStateMachineError::KmsThrottlingException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::KmsThrottlingExceptionBuilder::default();
                output = crate::protocol_serde::shape_kms_throttling_exception::de_kms_throttling_exception_json_err(_response_body, output)
                    .map_err(crate::operation::update_state_machine::UpdateStateMachineError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "MissingRequiredParameter" => crate::operation::update_state_machine::UpdateStateMachineError::MissingRequiredParameter({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::MissingRequiredParameterBuilder::default();
                output = crate::protocol_serde::shape_missing_required_parameter::de_missing_required_parameter_json_err(_response_body, output)
                    .map_err(crate::operation::update_state_machine::UpdateStateMachineError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ServiceQuotaExceededException" => crate::operation::update_state_machine::UpdateStateMachineError::ServiceQuotaExceededException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ServiceQuotaExceededExceptionBuilder::default();
                output = crate::protocol_serde::shape_service_quota_exceeded_exception::de_service_quota_exceeded_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::update_state_machine::UpdateStateMachineError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "StateMachineDeleting" => crate::operation::update_state_machine::UpdateStateMachineError::StateMachineDeleting({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::StateMachineDeletingBuilder::default();
                output = crate::protocol_serde::shape_state_machine_deleting::de_state_machine_deleting_json_err(_response_body, output)
                    .map_err(crate::operation::update_state_machine::UpdateStateMachineError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "StateMachineDoesNotExist" => crate::operation::update_state_machine::UpdateStateMachineError::StateMachineDoesNotExist({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::StateMachineDoesNotExistBuilder::default();
                output = crate::protocol_serde::shape_state_machine_does_not_exist::de_state_machine_does_not_exist_json_err(_response_body, output)
                    .map_err(crate::operation::update_state_machine::UpdateStateMachineError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ValidationException" => crate::operation::update_state_machine::UpdateStateMachineError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                    .map_err(crate::operation::update_state_machine::UpdateStateMachineError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::update_state_machine::UpdateStateMachineError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_state_machine_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::update_state_machine::UpdateStateMachineOutput,
    crate::operation::update_state_machine::UpdateStateMachineError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::update_state_machine::builders::UpdateStateMachineOutputBuilder::default();
        output = crate::protocol_serde::shape_update_state_machine::de_update_state_machine(_response_body, output)
            .map_err(crate::operation::update_state_machine::UpdateStateMachineError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::update_state_machine_output_output_correct_errors(output)
            .build()
            .map_err(crate::operation::update_state_machine::UpdateStateMachineError::unhandled)?
    })
}

pub fn ser_update_state_machine_input(
    input: &crate::operation::update_state_machine::UpdateStateMachineInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_update_state_machine_input::ser_update_state_machine_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_update_state_machine(
    value: &[u8],
    mut builder: crate::operation::update_state_machine::builders::UpdateStateMachineOutputBuilder,
) -> ::std::result::Result<
    crate::operation::update_state_machine::builders::UpdateStateMachineOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "updateDate" => {
                    builder = builder.set_update_date(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                        tokens.next(),
                        ::aws_smithy_types::date_time::Format::EpochSeconds,
                    )?);
                }
                "revisionId" => {
                    builder = builder.set_revision_id(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "stateMachineVersionArn" => {
                    builder = builder.set_state_machine_version_arn(
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
