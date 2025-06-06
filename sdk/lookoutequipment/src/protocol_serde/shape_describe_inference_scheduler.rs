// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_inference_scheduler_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_inference_scheduler::DescribeInferenceSchedulerOutput,
    crate::operation::describe_inference_scheduler::DescribeInferenceSchedulerError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::describe_inference_scheduler::DescribeInferenceSchedulerError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::describe_inference_scheduler::DescribeInferenceSchedulerError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::operation::describe_inference_scheduler::DescribeInferenceSchedulerError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                    .map_err(crate::operation::describe_inference_scheduler::DescribeInferenceSchedulerError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::access_denied_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::describe_inference_scheduler::DescribeInferenceSchedulerError::unhandled)?
            };
            tmp
        }),
        "InternalServerException" => crate::operation::describe_inference_scheduler::DescribeInferenceSchedulerError::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                    .map_err(crate::operation::describe_inference_scheduler::DescribeInferenceSchedulerError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::internal_server_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::describe_inference_scheduler::DescribeInferenceSchedulerError::unhandled)?
            };
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::describe_inference_scheduler::DescribeInferenceSchedulerError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::describe_inference_scheduler::DescribeInferenceSchedulerError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::resource_not_found_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::describe_inference_scheduler::DescribeInferenceSchedulerError::unhandled)?
            };
            tmp
        }),
        "ThrottlingException" => crate::operation::describe_inference_scheduler::DescribeInferenceSchedulerError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                    .map_err(crate::operation::describe_inference_scheduler::DescribeInferenceSchedulerError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::throttling_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::describe_inference_scheduler::DescribeInferenceSchedulerError::unhandled)?
            };
            tmp
        }),
        "ValidationException" => crate::operation::describe_inference_scheduler::DescribeInferenceSchedulerError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                    .map_err(crate::operation::describe_inference_scheduler::DescribeInferenceSchedulerError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::validation_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::describe_inference_scheduler::DescribeInferenceSchedulerError::unhandled)?
            };
            tmp
        }),
        _ => crate::operation::describe_inference_scheduler::DescribeInferenceSchedulerError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_inference_scheduler_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_inference_scheduler::DescribeInferenceSchedulerOutput,
    crate::operation::describe_inference_scheduler::DescribeInferenceSchedulerError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_inference_scheduler::builders::DescribeInferenceSchedulerOutputBuilder::default();
        output = crate::protocol_serde::shape_describe_inference_scheduler::de_describe_inference_scheduler(_response_body, output)
            .map_err(crate::operation::describe_inference_scheduler::DescribeInferenceSchedulerError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_describe_inference_scheduler_input(
    input: &crate::operation::describe_inference_scheduler::DescribeInferenceSchedulerInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_describe_inference_scheduler_input::ser_describe_inference_scheduler_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_describe_inference_scheduler(
    value: &[u8],
    mut builder: crate::operation::describe_inference_scheduler::builders::DescribeInferenceSchedulerOutputBuilder,
) -> ::std::result::Result<
    crate::operation::describe_inference_scheduler::builders::DescribeInferenceSchedulerOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "ModelArn" => {
                    builder = builder.set_model_arn(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "ModelName" => {
                    builder = builder.set_model_name(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "InferenceSchedulerName" => {
                    builder = builder.set_inference_scheduler_name(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "InferenceSchedulerArn" => {
                    builder = builder.set_inference_scheduler_arn(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "Status" => {
                    builder = builder.set_status(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| crate::types::InferenceSchedulerStatus::from(u.as_ref())))
                            .transpose()?,
                    );
                }
                "DataDelayOffsetInMinutes" => {
                    builder = builder.set_data_delay_offset_in_minutes(
                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                            .map(i64::try_from)
                            .transpose()?,
                    );
                }
                "DataUploadFrequency" => {
                    builder = builder.set_data_upload_frequency(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| crate::types::DataUploadFrequency::from(u.as_ref())))
                            .transpose()?,
                    );
                }
                "CreatedAt" => {
                    builder = builder.set_created_at(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                        tokens.next(),
                        ::aws_smithy_types::date_time::Format::EpochSeconds,
                    )?);
                }
                "UpdatedAt" => {
                    builder = builder.set_updated_at(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                        tokens.next(),
                        ::aws_smithy_types::date_time::Format::EpochSeconds,
                    )?);
                }
                "DataInputConfiguration" => {
                    builder = builder.set_data_input_configuration(
                        crate::protocol_serde::shape_inference_input_configuration::de_inference_input_configuration(tokens)?,
                    );
                }
                "DataOutputConfiguration" => {
                    builder = builder.set_data_output_configuration(
                        crate::protocol_serde::shape_inference_output_configuration::de_inference_output_configuration(tokens)?,
                    );
                }
                "RoleArn" => {
                    builder = builder.set_role_arn(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "ServerSideKmsKeyId" => {
                    builder = builder.set_server_side_kms_key_id(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "LatestInferenceResult" => {
                    builder = builder.set_latest_inference_result(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| crate::types::LatestInferenceResult::from(u.as_ref())))
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
