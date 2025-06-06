// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_create_cluster_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::create_cluster::CreateClusterOutput, crate::operation::create_cluster::CreateClusterError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::create_cluster::CreateClusterError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::create_cluster::CreateClusterError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ClusterAlreadyExists" => crate::operation::create_cluster::CreateClusterError::ClusterAlreadyExistsFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ClusterAlreadyExistsFaultBuilder::default();
                output = crate::protocol_serde::shape_cluster_already_exists_fault::de_cluster_already_exists_fault_xml_err(_response_body, output)
                    .map_err(crate::operation::create_cluster::CreateClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ClusterParameterGroupNotFound" => crate::operation::create_cluster::CreateClusterError::ClusterParameterGroupNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ClusterParameterGroupNotFoundFaultBuilder::default();
                output = crate::protocol_serde::shape_cluster_parameter_group_not_found_fault::de_cluster_parameter_group_not_found_fault_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::create_cluster::CreateClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ClusterQuotaExceeded" => crate::operation::create_cluster::CreateClusterError::ClusterQuotaExceededFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ClusterQuotaExceededFaultBuilder::default();
                output = crate::protocol_serde::shape_cluster_quota_exceeded_fault::de_cluster_quota_exceeded_fault_xml_err(_response_body, output)
                    .map_err(crate::operation::create_cluster::CreateClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ClusterSecurityGroupNotFound" => crate::operation::create_cluster::CreateClusterError::ClusterSecurityGroupNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ClusterSecurityGroupNotFoundFaultBuilder::default();
                output = crate::protocol_serde::shape_cluster_security_group_not_found_fault::de_cluster_security_group_not_found_fault_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::create_cluster::CreateClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ClusterSubnetGroupNotFoundFault" => crate::operation::create_cluster::CreateClusterError::ClusterSubnetGroupNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ClusterSubnetGroupNotFoundFaultBuilder::default();
                output = crate::protocol_serde::shape_cluster_subnet_group_not_found_fault::de_cluster_subnet_group_not_found_fault_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::create_cluster::CreateClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "DependentServiceRequestThrottlingFault" => {
            crate::operation::create_cluster::CreateClusterError::DependentServiceRequestThrottlingFault({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::DependentServiceRequestThrottlingFaultBuilder::default();
                    output = crate::protocol_serde::shape_dependent_service_request_throttling_fault::de_dependent_service_request_throttling_fault_xml_err(_response_body, output).map_err(crate::operation::create_cluster::CreateClusterError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "HsmClientCertificateNotFoundFault" => crate::operation::create_cluster::CreateClusterError::HsmClientCertificateNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::HsmClientCertificateNotFoundFaultBuilder::default();
                output = crate::protocol_serde::shape_hsm_client_certificate_not_found_fault::de_hsm_client_certificate_not_found_fault_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::create_cluster::CreateClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "HsmConfigurationNotFoundFault" => crate::operation::create_cluster::CreateClusterError::HsmConfigurationNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::HsmConfigurationNotFoundFaultBuilder::default();
                output = crate::protocol_serde::shape_hsm_configuration_not_found_fault::de_hsm_configuration_not_found_fault_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::create_cluster::CreateClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InsufficientClusterCapacity" => crate::operation::create_cluster::CreateClusterError::InsufficientClusterCapacityFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InsufficientClusterCapacityFaultBuilder::default();
                output = crate::protocol_serde::shape_insufficient_cluster_capacity_fault::de_insufficient_cluster_capacity_fault_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::create_cluster::CreateClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidClusterSubnetGroupStateFault" => crate::operation::create_cluster::CreateClusterError::InvalidClusterSubnetGroupStateFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidClusterSubnetGroupStateFaultBuilder::default();
                output = crate::protocol_serde::shape_invalid_cluster_subnet_group_state_fault::de_invalid_cluster_subnet_group_state_fault_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::create_cluster::CreateClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidClusterTrack" => crate::operation::create_cluster::CreateClusterError::InvalidClusterTrackFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidClusterTrackFaultBuilder::default();
                output = crate::protocol_serde::shape_invalid_cluster_track_fault::de_invalid_cluster_track_fault_xml_err(_response_body, output)
                    .map_err(crate::operation::create_cluster::CreateClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidElasticIpFault" => crate::operation::create_cluster::CreateClusterError::InvalidElasticIpFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidElasticIpFaultBuilder::default();
                output = crate::protocol_serde::shape_invalid_elastic_ip_fault::de_invalid_elastic_ip_fault_xml_err(_response_body, output)
                    .map_err(crate::operation::create_cluster::CreateClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidRetentionPeriodFault" => crate::operation::create_cluster::CreateClusterError::InvalidRetentionPeriodFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidRetentionPeriodFaultBuilder::default();
                output =
                    crate::protocol_serde::shape_invalid_retention_period_fault::de_invalid_retention_period_fault_xml_err(_response_body, output)
                        .map_err(crate::operation::create_cluster::CreateClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidSubnet" => crate::operation::create_cluster::CreateClusterError::InvalidSubnet({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidSubnetBuilder::default();
                output = crate::protocol_serde::shape_invalid_subnet::de_invalid_subnet_xml_err(_response_body, output)
                    .map_err(crate::operation::create_cluster::CreateClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidTagFault" => crate::operation::create_cluster::CreateClusterError::InvalidTagFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidTagFaultBuilder::default();
                output = crate::protocol_serde::shape_invalid_tag_fault::de_invalid_tag_fault_xml_err(_response_body, output)
                    .map_err(crate::operation::create_cluster::CreateClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidVPCNetworkStateFault" => crate::operation::create_cluster::CreateClusterError::InvalidVpcNetworkStateFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidVpcNetworkStateFaultBuilder::default();
                output =
                    crate::protocol_serde::shape_invalid_vpc_network_state_fault::de_invalid_vpc_network_state_fault_xml_err(_response_body, output)
                        .map_err(crate::operation::create_cluster::CreateClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "Ipv6CidrBlockNotFoundFault" => crate::operation::create_cluster::CreateClusterError::Ipv6CidrBlockNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::Ipv6CidrBlockNotFoundFaultBuilder::default();
                output =
                    crate::protocol_serde::shape_ipv6_cidr_block_not_found_fault::de_ipv6_cidr_block_not_found_fault_xml_err(_response_body, output)
                        .map_err(crate::operation::create_cluster::CreateClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "LimitExceededFault" => crate::operation::create_cluster::CreateClusterError::LimitExceededFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::LimitExceededFaultBuilder::default();
                output = crate::protocol_serde::shape_limit_exceeded_fault::de_limit_exceeded_fault_xml_err(_response_body, output)
                    .map_err(crate::operation::create_cluster::CreateClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "NumberOfNodesPerClusterLimitExceeded" => crate::operation::create_cluster::CreateClusterError::NumberOfNodesPerClusterLimitExceededFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::NumberOfNodesPerClusterLimitExceededFaultBuilder::default();
                output = crate::protocol_serde::shape_number_of_nodes_per_cluster_limit_exceeded_fault::de_number_of_nodes_per_cluster_limit_exceeded_fault_xml_err(_response_body, output).map_err(crate::operation::create_cluster::CreateClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "NumberOfNodesQuotaExceeded" => crate::operation::create_cluster::CreateClusterError::NumberOfNodesQuotaExceededFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::NumberOfNodesQuotaExceededFaultBuilder::default();
                output = crate::protocol_serde::shape_number_of_nodes_quota_exceeded_fault::de_number_of_nodes_quota_exceeded_fault_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::create_cluster::CreateClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "RedshiftIdcApplicationNotExists" => crate::operation::create_cluster::CreateClusterError::RedshiftIdcApplicationNotExistsFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::RedshiftIdcApplicationNotExistsFaultBuilder::default();
                output =
                    crate::protocol_serde::shape_redshift_idc_application_not_exists_fault::de_redshift_idc_application_not_exists_fault_xml_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::create_cluster::CreateClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "SnapshotScheduleNotFound" => crate::operation::create_cluster::CreateClusterError::SnapshotScheduleNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::SnapshotScheduleNotFoundFaultBuilder::default();
                output = crate::protocol_serde::shape_snapshot_schedule_not_found_fault::de_snapshot_schedule_not_found_fault_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::create_cluster::CreateClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "TagLimitExceededFault" => crate::operation::create_cluster::CreateClusterError::TagLimitExceededFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::TagLimitExceededFaultBuilder::default();
                output = crate::protocol_serde::shape_tag_limit_exceeded_fault::de_tag_limit_exceeded_fault_xml_err(_response_body, output)
                    .map_err(crate::operation::create_cluster::CreateClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "UnauthorizedOperation" => crate::operation::create_cluster::CreateClusterError::UnauthorizedOperation({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::UnauthorizedOperationBuilder::default();
                output = crate::protocol_serde::shape_unauthorized_operation::de_unauthorized_operation_xml_err(_response_body, output)
                    .map_err(crate::operation::create_cluster::CreateClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "UnsupportedOperation" => crate::operation::create_cluster::CreateClusterError::UnsupportedOperationFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::UnsupportedOperationFaultBuilder::default();
                output = crate::protocol_serde::shape_unsupported_operation_fault::de_unsupported_operation_fault_xml_err(_response_body, output)
                    .map_err(crate::operation::create_cluster::CreateClusterError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::create_cluster::CreateClusterError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_cluster_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::create_cluster::CreateClusterOutput, crate::operation::create_cluster::CreateClusterError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::create_cluster::builders::CreateClusterOutputBuilder::default();
        output = crate::protocol_serde::shape_create_cluster::de_create_cluster(_response_body, output)
            .map_err(crate::operation::create_cluster::CreateClusterError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_create_cluster(
    inp: &[u8],
    mut builder: crate::operation::create_cluster::builders::CreateClusterOutputBuilder,
) -> std::result::Result<crate::operation::create_cluster::builders::CreateClusterOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("CreateClusterResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected CreateClusterResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("CreateClusterResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected CreateClusterResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("Cluster") /* Cluster com.amazonaws.redshift.synthetic#CreateClusterOutput$Cluster */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_cluster::de_cluster(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_cluster(var_1);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("expected CreateClusterResult tag"));
    };
    Ok(builder)
}
