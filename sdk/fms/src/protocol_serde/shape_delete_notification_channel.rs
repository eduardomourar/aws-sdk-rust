// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_notification_channel_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_notification_channel::DeleteNotificationChannelOutput,
    crate::operation::delete_notification_channel::DeleteNotificationChannelError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::delete_notification_channel::DeleteNotificationChannelError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::delete_notification_channel::DeleteNotificationChannelError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalErrorException" => crate::operation::delete_notification_channel::DeleteNotificationChannelError::InternalErrorException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalErrorExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_error_exception::de_internal_error_exception_json_err(_response_body, output)
                    .map_err(crate::operation::delete_notification_channel::DeleteNotificationChannelError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidOperationException" => crate::operation::delete_notification_channel::DeleteNotificationChannelError::InvalidOperationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidOperationExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_operation_exception::de_invalid_operation_exception_json_err(_response_body, output)
                    .map_err(crate::operation::delete_notification_channel::DeleteNotificationChannelError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::delete_notification_channel::DeleteNotificationChannelError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::delete_notification_channel::DeleteNotificationChannelError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::delete_notification_channel::DeleteNotificationChannelError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_notification_channel_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_notification_channel::DeleteNotificationChannelOutput,
    crate::operation::delete_notification_channel::DeleteNotificationChannelError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_notification_channel::builders::DeleteNotificationChannelOutputBuilder::default();
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_delete_notification_channel_input(
    _input: &crate::operation::delete_notification_channel::DeleteNotificationChannelInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    Ok(::aws_smithy_types::body::SdkBody::from("{}"))
}
