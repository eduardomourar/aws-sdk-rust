// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_kx_cluster_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::get_kx_cluster::GetKxClusterOutput, crate::operation::get_kx_cluster::GetKxClusterError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::get_kx_cluster::GetKxClusterError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::get_kx_cluster::GetKxClusterError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::operation::get_kx_cluster::GetKxClusterError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_kx_cluster::GetKxClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ConflictException" => crate::operation::get_kx_cluster::GetKxClusterError::ConflictException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ConflictExceptionBuilder::default();
                output = crate::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_kx_cluster::GetKxClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InternalServerException" => crate::operation::get_kx_cluster::GetKxClusterError::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_kx_cluster::GetKxClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "LimitExceededException" => crate::operation::get_kx_cluster::GetKxClusterError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::LimitExceededExceptionBuilder::default();
                output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_kx_cluster::GetKxClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::get_kx_cluster::GetKxClusterError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_kx_cluster::GetKxClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ThrottlingException" => crate::operation::get_kx_cluster::GetKxClusterError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_kx_cluster::GetKxClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ValidationException" => crate::operation::get_kx_cluster::GetKxClusterError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_kx_cluster::GetKxClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::get_kx_cluster::GetKxClusterError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_kx_cluster_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::get_kx_cluster::GetKxClusterOutput, crate::operation::get_kx_cluster::GetKxClusterError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_kx_cluster::builders::GetKxClusterOutputBuilder::default();
        output = crate::protocol_serde::shape_get_kx_cluster::de_get_kx_cluster(_response_body, output)
            .map_err(crate::operation::get_kx_cluster::GetKxClusterError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_get_kx_cluster(
    value: &[u8],
    mut builder: crate::operation::get_kx_cluster::builders::GetKxClusterOutputBuilder,
) -> ::std::result::Result<
    crate::operation::get_kx_cluster::builders::GetKxClusterOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "autoScalingConfiguration" => {
                    builder = builder.set_auto_scaling_configuration(
                        crate::protocol_serde::shape_auto_scaling_configuration::de_auto_scaling_configuration(tokens)?,
                    );
                }
                "availabilityZoneId" => {
                    builder = builder.set_availability_zone_id(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "azMode" => {
                    builder = builder.set_az_mode(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| crate::types::KxAzMode::from(u.as_ref())))
                            .transpose()?,
                    );
                }
                "cacheStorageConfigurations" => {
                    builder = builder.set_cache_storage_configurations(
                        crate::protocol_serde::shape_kx_cache_storage_configurations::de_kx_cache_storage_configurations(tokens)?,
                    );
                }
                "capacityConfiguration" => {
                    builder =
                        builder.set_capacity_configuration(crate::protocol_serde::shape_capacity_configuration::de_capacity_configuration(tokens)?);
                }
                "clusterDescription" => {
                    builder = builder.set_cluster_description(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "clusterName" => {
                    builder = builder.set_cluster_name(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "clusterType" => {
                    builder = builder.set_cluster_type(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| crate::types::KxClusterType::from(u.as_ref())))
                            .transpose()?,
                    );
                }
                "code" => {
                    builder = builder.set_code(crate::protocol_serde::shape_code_configuration::de_code_configuration(tokens)?);
                }
                "commandLineArguments" => {
                    builder = builder.set_command_line_arguments(
                        crate::protocol_serde::shape_kx_command_line_arguments::de_kx_command_line_arguments(tokens)?,
                    );
                }
                "createdTimestamp" => {
                    builder = builder.set_created_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                        tokens.next(),
                        ::aws_smithy_types::date_time::Format::EpochSeconds,
                    )?);
                }
                "databases" => {
                    builder = builder.set_databases(crate::protocol_serde::shape_kx_database_configurations::de_kx_database_configurations(
                        tokens,
                    )?);
                }
                "executionRole" => {
                    builder = builder.set_execution_role(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "initializationScript" => {
                    builder = builder.set_initialization_script(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "lastModifiedTimestamp" => {
                    builder = builder.set_last_modified_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                        tokens.next(),
                        ::aws_smithy_types::date_time::Format::EpochSeconds,
                    )?);
                }
                "releaseLabel" => {
                    builder = builder.set_release_label(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "savedownStorageConfiguration" => {
                    builder = builder.set_savedown_storage_configuration(
                        crate::protocol_serde::shape_kx_savedown_storage_configuration::de_kx_savedown_storage_configuration(tokens)?,
                    );
                }
                "scalingGroupConfiguration" => {
                    builder = builder.set_scaling_group_configuration(
                        crate::protocol_serde::shape_kx_scaling_group_configuration::de_kx_scaling_group_configuration(tokens)?,
                    );
                }
                "status" => {
                    builder = builder.set_status(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| crate::types::KxClusterStatus::from(u.as_ref())))
                            .transpose()?,
                    );
                }
                "statusReason" => {
                    builder = builder.set_status_reason(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "tickerplantLogConfiguration" => {
                    builder = builder.set_tickerplant_log_configuration(
                        crate::protocol_serde::shape_tickerplant_log_configuration::de_tickerplant_log_configuration(tokens)?,
                    );
                }
                "volumes" => {
                    builder = builder.set_volumes(crate::protocol_serde::shape_volumes::de_volumes(tokens)?);
                }
                "vpcConfiguration" => {
                    builder = builder.set_vpc_configuration(crate::protocol_serde::shape_vpc_configuration::de_vpc_configuration(tokens)?);
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
