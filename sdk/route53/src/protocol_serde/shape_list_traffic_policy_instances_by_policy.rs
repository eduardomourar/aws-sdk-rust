// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_list_traffic_policy_instances_by_policy_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::list_traffic_policy_instances_by_policy::ListTrafficPolicyInstancesByPolicyOutput,
    crate::operation::list_traffic_policy_instances_by_policy::ListTrafficPolicyInstancesByPolicyError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::list_traffic_policy_instances_by_policy::ListTrafficPolicyInstancesByPolicyError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::list_traffic_policy_instances_by_policy::ListTrafficPolicyInstancesByPolicyError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidInput" => crate::operation::list_traffic_policy_instances_by_policy::ListTrafficPolicyInstancesByPolicyError::InvalidInput({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidInputBuilder::default();
                output = crate::protocol_serde::shape_invalid_input::de_invalid_input_xml_err(_response_body, output)
                    .map_err(crate::operation::list_traffic_policy_instances_by_policy::ListTrafficPolicyInstancesByPolicyError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "NoSuchTrafficPolicy" => {
            crate::operation::list_traffic_policy_instances_by_policy::ListTrafficPolicyInstancesByPolicyError::NoSuchTrafficPolicy({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NoSuchTrafficPolicyBuilder::default();
                    output = crate::protocol_serde::shape_no_such_traffic_policy::de_no_such_traffic_policy_xml_err(_response_body, output)
                        .map_err(crate::operation::list_traffic_policy_instances_by_policy::ListTrafficPolicyInstancesByPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "NoSuchTrafficPolicyInstance" => {
            crate::operation::list_traffic_policy_instances_by_policy::ListTrafficPolicyInstancesByPolicyError::NoSuchTrafficPolicyInstance({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::NoSuchTrafficPolicyInstanceBuilder::default();
                    output = crate::protocol_serde::shape_no_such_traffic_policy_instance::de_no_such_traffic_policy_instance_xml_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::list_traffic_policy_instances_by_policy::ListTrafficPolicyInstancesByPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::list_traffic_policy_instances_by_policy::ListTrafficPolicyInstancesByPolicyError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_traffic_policy_instances_by_policy_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::list_traffic_policy_instances_by_policy::ListTrafficPolicyInstancesByPolicyOutput,
    crate::operation::list_traffic_policy_instances_by_policy::ListTrafficPolicyInstancesByPolicyError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::list_traffic_policy_instances_by_policy::builders::ListTrafficPolicyInstancesByPolicyOutputBuilder::default();
        output =
            crate::protocol_serde::shape_list_traffic_policy_instances_by_policy::de_list_traffic_policy_instances_by_policy(_response_body, output)
                .map_err(crate::operation::list_traffic_policy_instances_by_policy::ListTrafficPolicyInstancesByPolicyError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::list_traffic_policy_instances_by_policy_output_output_correct_errors(output)
            .build()
            .map_err(crate::operation::list_traffic_policy_instances_by_policy::ListTrafficPolicyInstancesByPolicyError::unhandled)?
    })
}

#[allow(unused_mut)]
pub fn de_list_traffic_policy_instances_by_policy(
    inp: &[u8],
    mut builder: crate::operation::list_traffic_policy_instances_by_policy::builders::ListTrafficPolicyInstancesByPolicyOutputBuilder,
) -> std::result::Result<
    crate::operation::list_traffic_policy_instances_by_policy::builders::ListTrafficPolicyInstancesByPolicyOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !start_el.matches("ListTrafficPolicyInstancesByPolicyResponse") {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "encountered invalid XML root: expected ListTrafficPolicyInstancesByPolicyResponse but got {:?}. This is likely a bug in the SDK.",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("TrafficPolicyInstanceNameMarker") /* TrafficPolicyInstanceNameMarker com.amazonaws.route53.synthetic#ListTrafficPolicyInstancesByPolicyOutput$TrafficPolicyInstanceNameMarker */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_traffic_policy_instance_name_marker(var_1);
            }
            ,
            s if s.matches("TrafficPolicyInstances") /* TrafficPolicyInstances com.amazonaws.route53.synthetic#ListTrafficPolicyInstancesByPolicyOutput$TrafficPolicyInstances */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_traffic_policy_instances::de_traffic_policy_instances(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_traffic_policy_instances(var_2);
            }
            ,
            s if s.matches("TrafficPolicyInstanceTypeMarker") /* TrafficPolicyInstanceTypeMarker com.amazonaws.route53.synthetic#ListTrafficPolicyInstancesByPolicyOutput$TrafficPolicyInstanceTypeMarker */ =>  {
                let var_3 =
                    Some(
                        Result::<crate::types::RrType, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::RrType::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_traffic_policy_instance_type_marker(var_3);
            }
            ,
            s if s.matches("IsTruncated") /* IsTruncated com.amazonaws.route53.synthetic#ListTrafficPolicyInstancesByPolicyOutput$IsTruncated */ =>  {
                let var_4 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.route53#PageTruncated`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_is_truncated(var_4);
            }
            ,
            s if s.matches("HostedZoneIdMarker") /* HostedZoneIdMarker com.amazonaws.route53.synthetic#ListTrafficPolicyInstancesByPolicyOutput$HostedZoneIdMarker */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_hosted_zone_id_marker(var_5);
            }
            ,
            s if s.matches("MaxItems") /* MaxItems com.amazonaws.route53.synthetic#ListTrafficPolicyInstancesByPolicyOutput$MaxItems */ =>  {
                let var_6 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `smithy.api#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_max_items(var_6);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
