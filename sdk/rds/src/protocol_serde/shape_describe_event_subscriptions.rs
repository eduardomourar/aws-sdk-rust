// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_event_subscriptions_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_event_subscriptions::DescribeEventSubscriptionsOutput,
    crate::operation::describe_event_subscriptions::DescribeEventSubscriptionsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::describe_event_subscriptions::DescribeEventSubscriptionsError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::describe_event_subscriptions::DescribeEventSubscriptionsError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "SubscriptionNotFound" => crate::operation::describe_event_subscriptions::DescribeEventSubscriptionsError::SubscriptionNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::SubscriptionNotFoundFaultBuilder::default();
                output = crate::protocol_serde::shape_subscription_not_found_fault::de_subscription_not_found_fault_xml_err(_response_body, output)
                    .map_err(crate::operation::describe_event_subscriptions::DescribeEventSubscriptionsError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::describe_event_subscriptions::DescribeEventSubscriptionsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_event_subscriptions_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_event_subscriptions::DescribeEventSubscriptionsOutput,
    crate::operation::describe_event_subscriptions::DescribeEventSubscriptionsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_event_subscriptions::builders::DescribeEventSubscriptionsOutputBuilder::default();
        output = crate::protocol_serde::shape_describe_event_subscriptions::de_describe_event_subscriptions(_response_body, output)
            .map_err(crate::operation::describe_event_subscriptions::DescribeEventSubscriptionsError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_event_subscriptions(
    inp: &[u8],
    mut builder: crate::operation::describe_event_subscriptions::builders::DescribeEventSubscriptionsOutputBuilder,
) -> std::result::Result<
    crate::operation::describe_event_subscriptions::builders::DescribeEventSubscriptionsOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DescribeEventSubscriptionsResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DescribeEventSubscriptionsResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("DescribeEventSubscriptionsResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected DescribeEventSubscriptionsResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("Marker") /* Marker com.amazonaws.rds.synthetic#DescribeEventSubscriptionsOutput$Marker */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_marker(var_1);
            }
            ,
            s if s.matches("EventSubscriptionsList") /* EventSubscriptionsList com.amazonaws.rds.synthetic#DescribeEventSubscriptionsOutput$EventSubscriptionsList */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_event_subscriptions_list::de_event_subscriptions_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_event_subscriptions_list(var_2);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
            "expected DescribeEventSubscriptionsResult tag",
        ));
    };
    Ok(builder)
}
