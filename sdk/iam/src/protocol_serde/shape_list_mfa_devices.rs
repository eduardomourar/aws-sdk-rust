// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_list_mfa_devices_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::list_mfa_devices::ListMfaDevicesOutput, crate::operation::list_mfa_devices::ListMFADevicesError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::list_mfa_devices::ListMFADevicesError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::list_mfa_devices::ListMFADevicesError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "NoSuchEntity" => crate::operation::list_mfa_devices::ListMFADevicesError::NoSuchEntityException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::NoSuchEntityExceptionBuilder::default();
                output = crate::protocol_serde::shape_no_such_entity_exception::de_no_such_entity_exception_xml_err(_response_body, output)
                    .map_err(crate::operation::list_mfa_devices::ListMFADevicesError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ServiceFailure" => crate::operation::list_mfa_devices::ListMFADevicesError::ServiceFailureException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ServiceFailureExceptionBuilder::default();
                output = crate::protocol_serde::shape_service_failure_exception::de_service_failure_exception_xml_err(_response_body, output)
                    .map_err(crate::operation::list_mfa_devices::ListMFADevicesError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::list_mfa_devices::ListMFADevicesError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_mfa_devices_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::list_mfa_devices::ListMfaDevicesOutput, crate::operation::list_mfa_devices::ListMFADevicesError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::list_mfa_devices::builders::ListMfaDevicesOutputBuilder::default();
        output = crate::protocol_serde::shape_list_mfa_devices::de_list_mfa_devices(_response_body, output)
            .map_err(crate::operation::list_mfa_devices::ListMFADevicesError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::list_mfa_devices_output_output_correct_errors(output)
            .build()
            .map_err(crate::operation::list_mfa_devices::ListMFADevicesError::unhandled)?
    })
}

#[allow(unused_mut)]
pub fn de_list_mfa_devices(
    inp: &[u8],
    mut builder: crate::operation::list_mfa_devices::builders::ListMfaDevicesOutputBuilder,
) -> std::result::Result<crate::operation::list_mfa_devices::builders::ListMfaDevicesOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("ListMFADevicesResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected ListMFADevicesResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("ListMFADevicesResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected ListMFADevicesResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("MFADevices") /* MFADevices com.amazonaws.iam.synthetic#ListMFADevicesOutput$MFADevices */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_mfa_device_list_type::de_mfa_device_list_type(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_mfa_devices(var_1);
            }
            ,
            s if s.matches("IsTruncated") /* IsTruncated com.amazonaws.iam.synthetic#ListMFADevicesOutput$IsTruncated */ =>  {
                let var_2 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.iam#booleanType`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_is_truncated(var_2);
            }
            ,
            s if s.matches("Marker") /* Marker com.amazonaws.iam.synthetic#ListMFADevicesOutput$Marker */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_marker(var_3);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("expected ListMFADevicesResult tag"));
    };
    Ok(builder)
}
