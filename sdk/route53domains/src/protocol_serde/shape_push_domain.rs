// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_push_domain_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::push_domain::PushDomainOutput, crate::operation::push_domain::PushDomainError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::push_domain::PushDomainError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::push_domain::PushDomainError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidInput" => crate::operation::push_domain::PushDomainError::InvalidInput({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidInputBuilder::default();
                output = crate::protocol_serde::shape_invalid_input::de_invalid_input_json_err(_response_body, output)
                    .map_err(crate::operation::push_domain::PushDomainError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "OperationLimitExceeded" => crate::operation::push_domain::PushDomainError::OperationLimitExceeded({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::OperationLimitExceededBuilder::default();
                output = crate::protocol_serde::shape_operation_limit_exceeded::de_operation_limit_exceeded_json_err(_response_body, output)
                    .map_err(crate::operation::push_domain::PushDomainError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "UnsupportedTLD" => crate::operation::push_domain::PushDomainError::UnsupportedTld({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::UnsupportedTldBuilder::default();
                output = crate::protocol_serde::shape_unsupported_tld::de_unsupported_tld_json_err(_response_body, output)
                    .map_err(crate::operation::push_domain::PushDomainError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::push_domain::PushDomainError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_push_domain_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::push_domain::PushDomainOutput, crate::operation::push_domain::PushDomainError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::push_domain::builders::PushDomainOutputBuilder::default();
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_push_domain_input(
    input: &crate::operation::push_domain::PushDomainInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_push_domain_input::ser_push_domain_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
