// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_register_webhook_with_third_party_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::register_webhook_with_third_party::RegisterWebhookWithThirdPartyOutput,
    crate::operation::register_webhook_with_third_party::RegisterWebhookWithThirdPartyError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::register_webhook_with_third_party::RegisterWebhookWithThirdPartyError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::register_webhook_with_third_party::RegisterWebhookWithThirdPartyError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ValidationException" => crate::operation::register_webhook_with_third_party::RegisterWebhookWithThirdPartyError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                    .map_err(crate::operation::register_webhook_with_third_party::RegisterWebhookWithThirdPartyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "WebhookNotFoundException" => {
            crate::operation::register_webhook_with_third_party::RegisterWebhookWithThirdPartyError::WebhookNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::WebhookNotFoundExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_webhook_not_found_exception::de_webhook_not_found_exception_json_err(_response_body, output)
                            .map_err(crate::operation::register_webhook_with_third_party::RegisterWebhookWithThirdPartyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::register_webhook_with_third_party::RegisterWebhookWithThirdPartyError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_register_webhook_with_third_party_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::register_webhook_with_third_party::RegisterWebhookWithThirdPartyOutput,
    crate::operation::register_webhook_with_third_party::RegisterWebhookWithThirdPartyError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::register_webhook_with_third_party::builders::RegisterWebhookWithThirdPartyOutputBuilder::default();
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_register_webhook_with_third_party_input(
    input: &crate::operation::register_webhook_with_third_party::RegisterWebhookWithThirdPartyInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_register_webhook_with_third_party_input::ser_register_webhook_with_third_party_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
