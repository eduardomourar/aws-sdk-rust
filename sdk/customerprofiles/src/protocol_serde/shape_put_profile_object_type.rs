// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_put_profile_object_type_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::put_profile_object_type::PutProfileObjectTypeOutput,
    crate::operation::put_profile_object_type::PutProfileObjectTypeError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::put_profile_object_type::PutProfileObjectTypeError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::put_profile_object_type::PutProfileObjectTypeError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::operation::put_profile_object_type::PutProfileObjectTypeError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                    .map_err(crate::operation::put_profile_object_type::PutProfileObjectTypeError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "BadRequestException" => crate::operation::put_profile_object_type::PutProfileObjectTypeError::BadRequestException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::BadRequestExceptionBuilder::default();
                output = crate::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(_response_body, output)
                    .map_err(crate::operation::put_profile_object_type::PutProfileObjectTypeError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InternalServerException" => crate::operation::put_profile_object_type::PutProfileObjectTypeError::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                    .map_err(crate::operation::put_profile_object_type::PutProfileObjectTypeError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::put_profile_object_type::PutProfileObjectTypeError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::put_profile_object_type::PutProfileObjectTypeError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ThrottlingException" => crate::operation::put_profile_object_type::PutProfileObjectTypeError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                    .map_err(crate::operation::put_profile_object_type::PutProfileObjectTypeError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::put_profile_object_type::PutProfileObjectTypeError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_profile_object_type_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::put_profile_object_type::PutProfileObjectTypeOutput,
    crate::operation::put_profile_object_type::PutProfileObjectTypeError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::put_profile_object_type::builders::PutProfileObjectTypeOutputBuilder::default();
        output = crate::protocol_serde::shape_put_profile_object_type::de_put_profile_object_type(_response_body, output)
            .map_err(crate::operation::put_profile_object_type::PutProfileObjectTypeError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::put_profile_object_type_output_output_correct_errors(output)
            .build()
            .map_err(crate::operation::put_profile_object_type::PutProfileObjectTypeError::unhandled)?
    })
}

pub fn ser_put_profile_object_type_input(
    input: &crate::operation::put_profile_object_type::PutProfileObjectTypeInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_put_profile_object_type_input::ser_put_profile_object_type_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_put_profile_object_type(
    value: &[u8],
    mut builder: crate::operation::put_profile_object_type::builders::PutProfileObjectTypeOutputBuilder,
) -> ::std::result::Result<
    crate::operation::put_profile_object_type::builders::PutProfileObjectTypeOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "AllowProfileCreation" => {
                    builder = builder.set_allow_profile_creation(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                }
                "CreatedAt" => {
                    builder = builder.set_created_at(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                        tokens.next(),
                        ::aws_smithy_types::date_time::Format::EpochSeconds,
                    )?);
                }
                "Description" => {
                    builder = builder.set_description(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "EncryptionKey" => {
                    builder = builder.set_encryption_key(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "ExpirationDays" => {
                    builder = builder.set_expiration_days(
                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                            .map(i32::try_from)
                            .transpose()?,
                    );
                }
                "Fields" => {
                    builder = builder.set_fields(crate::protocol_serde::shape_field_map::de_field_map(tokens)?);
                }
                "Keys" => {
                    builder = builder.set_keys(crate::protocol_serde::shape_key_map::de_key_map(tokens)?);
                }
                "LastUpdatedAt" => {
                    builder = builder.set_last_updated_at(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                        tokens.next(),
                        ::aws_smithy_types::date_time::Format::EpochSeconds,
                    )?);
                }
                "MaxAvailableProfileObjectCount" => {
                    builder = builder.set_max_available_profile_object_count(
                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                            .map(i32::try_from)
                            .transpose()?,
                    );
                }
                "MaxProfileObjectCount" => {
                    builder = builder.set_max_profile_object_count(
                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                            .map(i32::try_from)
                            .transpose()?,
                    );
                }
                "ObjectTypeName" => {
                    builder = builder.set_object_type_name(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "SourceLastUpdatedTimestampFormat" => {
                    builder = builder.set_source_last_updated_timestamp_format(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "Tags" => {
                    builder = builder.set_tags(crate::protocol_serde::shape_tag_map::de_tag_map(tokens)?);
                }
                "TemplateId" => {
                    builder = builder.set_template_id(
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
