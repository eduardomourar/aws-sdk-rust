// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_event_subscription_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_event_subscription::DeleteEventSubscriptionOutput,
    crate::operation::delete_event_subscription::DeleteEventSubscriptionError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::delete_event_subscription::DeleteEventSubscriptionError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::delete_event_subscription::DeleteEventSubscriptionError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidEventSubscriptionState" => {
            crate::operation::delete_event_subscription::DeleteEventSubscriptionError::InvalidEventSubscriptionStateFault({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidEventSubscriptionStateFaultBuilder::default();
                    output = crate::protocol_serde::shape_invalid_event_subscription_state_fault::de_invalid_event_subscription_state_fault_xml_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::delete_event_subscription::DeleteEventSubscriptionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "SubscriptionNotFound" => crate::operation::delete_event_subscription::DeleteEventSubscriptionError::SubscriptionNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::SubscriptionNotFoundFaultBuilder::default();
                output = crate::protocol_serde::shape_subscription_not_found_fault::de_subscription_not_found_fault_xml_err(_response_body, output)
                    .map_err(crate::operation::delete_event_subscription::DeleteEventSubscriptionError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::delete_event_subscription::DeleteEventSubscriptionError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_event_subscription_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_event_subscription::DeleteEventSubscriptionOutput,
    crate::operation::delete_event_subscription::DeleteEventSubscriptionError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_event_subscription::builders::DeleteEventSubscriptionOutputBuilder::default();
        output = crate::protocol_serde::shape_delete_event_subscription::de_delete_event_subscription(_response_body, output)
            .map_err(crate::operation::delete_event_subscription::DeleteEventSubscriptionError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_delete_event_subscription(
    inp: &[u8],
    mut builder: crate::operation::delete_event_subscription::builders::DeleteEventSubscriptionOutputBuilder,
) -> std::result::Result<
    crate::operation::delete_event_subscription::builders::DeleteEventSubscriptionOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DeleteEventSubscriptionResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DeleteEventSubscriptionResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("DeleteEventSubscriptionResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected DeleteEventSubscriptionResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("EventSubscription") /* EventSubscription com.amazonaws.rds.synthetic#DeleteEventSubscriptionOutput$EventSubscription */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_event_subscription::de_event_subscription(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_event_subscription(var_1);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
            "expected DeleteEventSubscriptionResult tag",
        ));
    };
    Ok(builder)
}
