// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_face_liveness_session_results_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_face_liveness_session_results::GetFaceLivenessSessionResultsOutput,
    crate::operation::get_face_liveness_session_results::GetFaceLivenessSessionResultsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::get_face_liveness_session_results::GetFaceLivenessSessionResultsError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::get_face_liveness_session_results::GetFaceLivenessSessionResultsError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::operation::get_face_liveness_session_results::GetFaceLivenessSessionResultsError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_face_liveness_session_results::GetFaceLivenessSessionResultsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InternalServerError" => crate::operation::get_face_liveness_session_results::GetFaceLivenessSessionResultsError::InternalServerError({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalServerErrorBuilder::default();
                output = crate::protocol_serde::shape_internal_server_error::de_internal_server_error_json_err(_response_body, output)
                    .map_err(crate::operation::get_face_liveness_session_results::GetFaceLivenessSessionResultsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidParameterException" => {
            crate::operation::get_face_liveness_session_results::GetFaceLivenessSessionResultsError::InvalidParameterException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidParameterExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(_response_body, output)
                            .map_err(crate::operation::get_face_liveness_session_results::GetFaceLivenessSessionResultsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ProvisionedThroughputExceededException" => {
            crate::operation::get_face_liveness_session_results::GetFaceLivenessSessionResultsError::ProvisionedThroughputExceededException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ProvisionedThroughputExceededExceptionBuilder::default();
                    output = crate::protocol_serde::shape_provisioned_throughput_exceeded_exception::de_provisioned_throughput_exceeded_exception_json_err(_response_body, output).map_err(crate::operation::get_face_liveness_session_results::GetFaceLivenessSessionResultsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "SessionNotFoundException" => {
            crate::operation::get_face_liveness_session_results::GetFaceLivenessSessionResultsError::SessionNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::SessionNotFoundExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_session_not_found_exception::de_session_not_found_exception_json_err(_response_body, output)
                            .map_err(crate::operation::get_face_liveness_session_results::GetFaceLivenessSessionResultsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ThrottlingException" => crate::operation::get_face_liveness_session_results::GetFaceLivenessSessionResultsError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_face_liveness_session_results::GetFaceLivenessSessionResultsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::get_face_liveness_session_results::GetFaceLivenessSessionResultsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_face_liveness_session_results_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_face_liveness_session_results::GetFaceLivenessSessionResultsOutput,
    crate::operation::get_face_liveness_session_results::GetFaceLivenessSessionResultsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_face_liveness_session_results::builders::GetFaceLivenessSessionResultsOutputBuilder::default();
        output = crate::protocol_serde::shape_get_face_liveness_session_results::de_get_face_liveness_session_results(_response_body, output)
            .map_err(crate::operation::get_face_liveness_session_results::GetFaceLivenessSessionResultsError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::get_face_liveness_session_results_output_output_correct_errors(output)
            .build()
            .map_err(crate::operation::get_face_liveness_session_results::GetFaceLivenessSessionResultsError::unhandled)?
    })
}

pub fn ser_get_face_liveness_session_results_input(
    input: &crate::operation::get_face_liveness_session_results::GetFaceLivenessSessionResultsInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_get_face_liveness_session_results_input::ser_get_face_liveness_session_results_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_get_face_liveness_session_results(
    value: &[u8],
    mut builder: crate::operation::get_face_liveness_session_results::builders::GetFaceLivenessSessionResultsOutputBuilder,
) -> ::std::result::Result<
    crate::operation::get_face_liveness_session_results::builders::GetFaceLivenessSessionResultsOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "SessionId" => {
                    builder = builder.set_session_id(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "Status" => {
                    builder = builder.set_status(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| crate::types::LivenessSessionStatus::from(u.as_ref())))
                            .transpose()?,
                    );
                }
                "Confidence" => {
                    builder = builder
                        .set_confidence(::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?.map(|v| v.to_f32_lossy()));
                }
                "ReferenceImage" => {
                    builder = builder.set_reference_image(crate::protocol_serde::shape_audit_image::de_audit_image(tokens)?);
                }
                "AuditImages" => {
                    builder = builder.set_audit_images(crate::protocol_serde::shape_audit_images::de_audit_images(tokens)?);
                }
                "Challenge" => {
                    builder = builder.set_challenge(crate::protocol_serde::shape_challenge::de_challenge(tokens)?);
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
