// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_transit_gateway_multicast_deregistered_group_sources(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::TransitGatewayMulticastDeregisteredGroupSources, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::TransitGatewayMulticastDeregisteredGroupSources::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("transitGatewayMulticastDomainId") /* TransitGatewayMulticastDomainId com.amazonaws.ec2#TransitGatewayMulticastDeregisteredGroupSources$TransitGatewayMulticastDomainId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_transit_gateway_multicast_domain_id(var_1);
            }
            ,
            s if s.matches("deregisteredNetworkInterfaceIds") /* DeregisteredNetworkInterfaceIds com.amazonaws.ec2#TransitGatewayMulticastDeregisteredGroupSources$DeregisteredNetworkInterfaceIds */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_value_string_list::de_value_string_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_deregistered_network_interface_ids(var_2);
            }
            ,
            s if s.matches("groupIpAddress") /* GroupIpAddress com.amazonaws.ec2#TransitGatewayMulticastDeregisteredGroupSources$GroupIpAddress */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_group_ip_address(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
