// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_document_permission_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::modify_document_permission::ModifyDocumentPermissionOutput,
    crate::operation::modify_document_permission::ModifyDocumentPermissionError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::modify_document_permission::ModifyDocumentPermissionError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::modify_document_permission::ModifyDocumentPermissionError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "DocumentLimitExceeded" => crate::operation::modify_document_permission::ModifyDocumentPermissionError::DocumentLimitExceeded({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::DocumentLimitExceededBuilder::default();
                output = crate::protocol_serde::shape_document_limit_exceeded::de_document_limit_exceeded_json_err(_response_body, output)
                    .map_err(crate::operation::modify_document_permission::ModifyDocumentPermissionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "DocumentPermissionLimit" => crate::operation::modify_document_permission::ModifyDocumentPermissionError::DocumentPermissionLimit({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::DocumentPermissionLimitBuilder::default();
                output = crate::protocol_serde::shape_document_permission_limit::de_document_permission_limit_json_err(_response_body, output)
                    .map_err(crate::operation::modify_document_permission::ModifyDocumentPermissionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InternalServerError" => crate::operation::modify_document_permission::ModifyDocumentPermissionError::InternalServerError({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalServerErrorBuilder::default();
                output = crate::protocol_serde::shape_internal_server_error::de_internal_server_error_json_err(_response_body, output)
                    .map_err(crate::operation::modify_document_permission::ModifyDocumentPermissionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidDocument" => crate::operation::modify_document_permission::ModifyDocumentPermissionError::InvalidDocument({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidDocumentBuilder::default();
                output = crate::protocol_serde::shape_invalid_document::de_invalid_document_json_err(_response_body, output)
                    .map_err(crate::operation::modify_document_permission::ModifyDocumentPermissionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidPermissionType" => crate::operation::modify_document_permission::ModifyDocumentPermissionError::InvalidPermissionType({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidPermissionTypeBuilder::default();
                output = crate::protocol_serde::shape_invalid_permission_type::de_invalid_permission_type_json_err(_response_body, output)
                    .map_err(crate::operation::modify_document_permission::ModifyDocumentPermissionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::modify_document_permission::ModifyDocumentPermissionError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_document_permission_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::modify_document_permission::ModifyDocumentPermissionOutput,
    crate::operation::modify_document_permission::ModifyDocumentPermissionError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::modify_document_permission::builders::ModifyDocumentPermissionOutputBuilder::default();
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_modify_document_permission_input(
    input: &crate::operation::modify_document_permission::ModifyDocumentPermissionInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_modify_document_permission_input::ser_modify_document_permission_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
