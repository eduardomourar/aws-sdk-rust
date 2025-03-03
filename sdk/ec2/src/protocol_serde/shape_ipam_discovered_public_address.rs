// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_ipam_discovered_public_address(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::IpamDiscoveredPublicAddress, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::IpamDiscoveredPublicAddress::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("ipamResourceDiscoveryId") /* IpamResourceDiscoveryId com.amazonaws.ec2#IpamDiscoveredPublicAddress$IpamResourceDiscoveryId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_ipam_resource_discovery_id(var_1);
            }
            ,
            s if s.matches("addressRegion") /* AddressRegion com.amazonaws.ec2#IpamDiscoveredPublicAddress$AddressRegion */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_address_region(var_2);
            }
            ,
            s if s.matches("address") /* Address com.amazonaws.ec2#IpamDiscoveredPublicAddress$Address */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_address(var_3);
            }
            ,
            s if s.matches("addressOwnerId") /* AddressOwnerId com.amazonaws.ec2#IpamDiscoveredPublicAddress$AddressOwnerId */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_address_owner_id(var_4);
            }
            ,
            s if s.matches("addressAllocationId") /* AddressAllocationId com.amazonaws.ec2#IpamDiscoveredPublicAddress$AddressAllocationId */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_address_allocation_id(var_5);
            }
            ,
            s if s.matches("associationStatus") /* AssociationStatus com.amazonaws.ec2#IpamDiscoveredPublicAddress$AssociationStatus */ =>  {
                let var_6 =
                    Some(
                        Result::<crate::types::IpamPublicAddressAssociationStatus, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::IpamPublicAddressAssociationStatus::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_association_status(var_6);
            }
            ,
            s if s.matches("addressType") /* AddressType com.amazonaws.ec2#IpamDiscoveredPublicAddress$AddressType */ =>  {
                let var_7 =
                    Some(
                        Result::<crate::types::IpamPublicAddressType, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::IpamPublicAddressType::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_address_type(var_7);
            }
            ,
            s if s.matches("service") /* Service com.amazonaws.ec2#IpamDiscoveredPublicAddress$Service */ =>  {
                let var_8 =
                    Some(
                        Result::<crate::types::IpamPublicAddressAwsService, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::IpamPublicAddressAwsService::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_service(var_8);
            }
            ,
            s if s.matches("serviceResource") /* ServiceResource com.amazonaws.ec2#IpamDiscoveredPublicAddress$ServiceResource */ =>  {
                let var_9 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_service_resource(var_9);
            }
            ,
            s if s.matches("vpcId") /* VpcId com.amazonaws.ec2#IpamDiscoveredPublicAddress$VpcId */ =>  {
                let var_10 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_vpc_id(var_10);
            }
            ,
            s if s.matches("subnetId") /* SubnetId com.amazonaws.ec2#IpamDiscoveredPublicAddress$SubnetId */ =>  {
                let var_11 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_subnet_id(var_11);
            }
            ,
            s if s.matches("publicIpv4PoolId") /* PublicIpv4PoolId com.amazonaws.ec2#IpamDiscoveredPublicAddress$PublicIpv4PoolId */ =>  {
                let var_12 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_public_ipv4_pool_id(var_12);
            }
            ,
            s if s.matches("networkInterfaceId") /* NetworkInterfaceId com.amazonaws.ec2#IpamDiscoveredPublicAddress$NetworkInterfaceId */ =>  {
                let var_13 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_network_interface_id(var_13);
            }
            ,
            s if s.matches("networkInterfaceDescription") /* NetworkInterfaceDescription com.amazonaws.ec2#IpamDiscoveredPublicAddress$NetworkInterfaceDescription */ =>  {
                let var_14 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_network_interface_description(var_14);
            }
            ,
            s if s.matches("instanceId") /* InstanceId com.amazonaws.ec2#IpamDiscoveredPublicAddress$InstanceId */ =>  {
                let var_15 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_instance_id(var_15);
            }
            ,
            s if s.matches("tags") /* Tags com.amazonaws.ec2#IpamDiscoveredPublicAddress$Tags */ =>  {
                let var_16 =
                    Some(
                        crate::protocol_serde::shape_ipam_public_address_tags::de_ipam_public_address_tags(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_16);
            }
            ,
            s if s.matches("networkBorderGroup") /* NetworkBorderGroup com.amazonaws.ec2#IpamDiscoveredPublicAddress$NetworkBorderGroup */ =>  {
                let var_17 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_network_border_group(var_17);
            }
            ,
            s if s.matches("securityGroupSet") /* SecurityGroups com.amazonaws.ec2#IpamDiscoveredPublicAddress$SecurityGroups */ =>  {
                let var_18 =
                    Some(
                        crate::protocol_serde::shape_ipam_public_address_security_group_list::de_ipam_public_address_security_group_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_security_groups(var_18);
            }
            ,
            s if s.matches("sampleTime") /* SampleTime com.amazonaws.ec2#IpamDiscoveredPublicAddress$SampleTime */ =>  {
                let var_19 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#MillisecondDateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_sample_time(var_19);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
