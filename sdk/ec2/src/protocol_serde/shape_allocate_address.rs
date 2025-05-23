// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_allocate_address_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::allocate_address::AllocateAddressOutput, crate::operation::allocate_address::AllocateAddressError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::allocate_address::AllocateAddressError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::allocate_address::AllocateAddressError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_allocate_address_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::allocate_address::AllocateAddressOutput, crate::operation::allocate_address::AllocateAddressError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::allocate_address::builders::AllocateAddressOutputBuilder::default();
        output = crate::protocol_serde::shape_allocate_address::de_allocate_address(_response_body, output)
            .map_err(crate::operation::allocate_address::AllocateAddressError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_allocate_address(
    inp: &[u8],
    mut builder: crate::operation::allocate_address::builders::AllocateAddressOutputBuilder,
) -> std::result::Result<crate::operation::allocate_address::builders::AllocateAddressOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("AllocateAddressResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected AllocateAddressResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("allocationId") /* AllocationId com.amazonaws.ec2.synthetic#AllocateAddressOutput$AllocationId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_allocation_id(var_1);
            }
            ,
            s if s.matches("publicIpv4Pool") /* PublicIpv4Pool com.amazonaws.ec2.synthetic#AllocateAddressOutput$PublicIpv4Pool */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_public_ipv4_pool(var_2);
            }
            ,
            s if s.matches("networkBorderGroup") /* NetworkBorderGroup com.amazonaws.ec2.synthetic#AllocateAddressOutput$NetworkBorderGroup */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_network_border_group(var_3);
            }
            ,
            s if s.matches("domain") /* Domain com.amazonaws.ec2.synthetic#AllocateAddressOutput$Domain */ =>  {
                let var_4 =
                    Some(
                        Result::<crate::types::DomainType, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::DomainType::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_domain(var_4);
            }
            ,
            s if s.matches("customerOwnedIp") /* CustomerOwnedIp com.amazonaws.ec2.synthetic#AllocateAddressOutput$CustomerOwnedIp */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_customer_owned_ip(var_5);
            }
            ,
            s if s.matches("customerOwnedIpv4Pool") /* CustomerOwnedIpv4Pool com.amazonaws.ec2.synthetic#AllocateAddressOutput$CustomerOwnedIpv4Pool */ =>  {
                let var_6 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_customer_owned_ipv4_pool(var_6);
            }
            ,
            s if s.matches("carrierIp") /* CarrierIp com.amazonaws.ec2.synthetic#AllocateAddressOutput$CarrierIp */ =>  {
                let var_7 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_carrier_ip(var_7);
            }
            ,
            s if s.matches("publicIp") /* PublicIp com.amazonaws.ec2.synthetic#AllocateAddressOutput$PublicIp */ =>  {
                let var_8 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_public_ip(var_8);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
