// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_custom_key_store_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_custom_key_store::DeleteCustomKeyStoreOutput,
    crate::operation::delete_custom_key_store::DeleteCustomKeyStoreError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::delete_custom_key_store::DeleteCustomKeyStoreError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::delete_custom_key_store::DeleteCustomKeyStoreError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "CustomKeyStoreHasCMKsException" => crate::operation::delete_custom_key_store::DeleteCustomKeyStoreError::CustomKeyStoreHasCmKsException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::CustomKeyStoreHasCmKsExceptionBuilder::default();
                output = crate::protocol_serde::shape_custom_key_store_has_cmks_exception::de_custom_key_store_has_cmks_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::delete_custom_key_store::DeleteCustomKeyStoreError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "CustomKeyStoreInvalidStateException" => {
            crate::operation::delete_custom_key_store::DeleteCustomKeyStoreError::CustomKeyStoreInvalidStateException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::CustomKeyStoreInvalidStateExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_custom_key_store_invalid_state_exception::de_custom_key_store_invalid_state_exception_json_err(
                            _response_body,
                            output,
                        )
                        .map_err(crate::operation::delete_custom_key_store::DeleteCustomKeyStoreError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "CustomKeyStoreNotFoundException" => crate::operation::delete_custom_key_store::DeleteCustomKeyStoreError::CustomKeyStoreNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::CustomKeyStoreNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_custom_key_store_not_found_exception::de_custom_key_store_not_found_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::delete_custom_key_store::DeleteCustomKeyStoreError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "KMSInternalException" => crate::operation::delete_custom_key_store::DeleteCustomKeyStoreError::KmsInternalException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::KmsInternalExceptionBuilder::default();
                output = crate::protocol_serde::shape_kms_internal_exception::de_kms_internal_exception_json_err(_response_body, output)
                    .map_err(crate::operation::delete_custom_key_store::DeleteCustomKeyStoreError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::delete_custom_key_store::DeleteCustomKeyStoreError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_custom_key_store_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_custom_key_store::DeleteCustomKeyStoreOutput,
    crate::operation::delete_custom_key_store::DeleteCustomKeyStoreError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_custom_key_store::builders::DeleteCustomKeyStoreOutputBuilder::default();
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_delete_custom_key_store_input(
    input: &crate::operation::delete_custom_key_store::DeleteCustomKeyStoreInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_delete_custom_key_store_input::ser_delete_custom_key_store_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
