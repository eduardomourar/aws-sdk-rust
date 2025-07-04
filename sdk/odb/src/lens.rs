// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn reflens_list_autonomous_virtual_machines_output_output_next_token(
    input: &crate::operation::list_autonomous_virtual_machines::ListAutonomousVirtualMachinesOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_cloud_autonomous_vm_clusters_output_output_next_token(
    input: &crate::operation::list_cloud_autonomous_vm_clusters::ListCloudAutonomousVmClustersOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_cloud_exadata_infrastructures_output_output_next_token(
    input: &crate::operation::list_cloud_exadata_infrastructures::ListCloudExadataInfrastructuresOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_cloud_vm_clusters_output_output_next_token(
    input: &crate::operation::list_cloud_vm_clusters::ListCloudVmClustersOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_db_nodes_output_output_next_token(
    input: &crate::operation::list_db_nodes::ListDbNodesOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_db_servers_output_output_next_token(
    input: &crate::operation::list_db_servers::ListDbServersOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_db_system_shapes_output_output_next_token(
    input: &crate::operation::list_db_system_shapes::ListDbSystemShapesOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_gi_versions_output_output_next_token(
    input: &crate::operation::list_gi_versions::ListGiVersionsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_odb_networks_output_output_next_token(
    input: &crate::operation::list_odb_networks::ListOdbNetworksOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_odb_peering_connections_output_output_next_token(
    input: &crate::operation::list_odb_peering_connections::ListOdbPeeringConnectionsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn reflens_list_system_versions_output_output_next_token(
    input: &crate::operation::list_system_versions::ListSystemVersionsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_autonomous_virtual_machines_output_output_autonomous_virtual_machines(
    input: crate::operation::list_autonomous_virtual_machines::ListAutonomousVirtualMachinesOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::AutonomousVirtualMachineSummary>> {
    let input = input.autonomous_virtual_machines;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_cloud_autonomous_vm_clusters_output_output_cloud_autonomous_vm_clusters(
    input: crate::operation::list_cloud_autonomous_vm_clusters::ListCloudAutonomousVmClustersOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::CloudAutonomousVmClusterSummary>> {
    let input = input.cloud_autonomous_vm_clusters;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_cloud_exadata_infrastructures_output_output_cloud_exadata_infrastructures(
    input: crate::operation::list_cloud_exadata_infrastructures::ListCloudExadataInfrastructuresOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::CloudExadataInfrastructureSummary>> {
    let input = input.cloud_exadata_infrastructures;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_cloud_vm_clusters_output_output_cloud_vm_clusters(
    input: crate::operation::list_cloud_vm_clusters::ListCloudVmClustersOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::CloudVmClusterSummary>> {
    let input = input.cloud_vm_clusters;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_db_nodes_output_output_db_nodes(
    input: crate::operation::list_db_nodes::ListDbNodesOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::DbNodeSummary>> {
    let input = input.db_nodes;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_db_servers_output_output_db_servers(
    input: crate::operation::list_db_servers::ListDbServersOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::DbServerSummary>> {
    let input = input.db_servers;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_db_system_shapes_output_output_db_system_shapes(
    input: crate::operation::list_db_system_shapes::ListDbSystemShapesOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::DbSystemShapeSummary>> {
    let input = input.db_system_shapes;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_gi_versions_output_output_gi_versions(
    input: crate::operation::list_gi_versions::ListGiVersionsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::GiVersionSummary>> {
    let input = input.gi_versions;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_odb_networks_output_output_odb_networks(
    input: crate::operation::list_odb_networks::ListOdbNetworksOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::OdbNetworkSummary>> {
    let input = input.odb_networks;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_odb_peering_connections_output_output_odb_peering_connections(
    input: crate::operation::list_odb_peering_connections::ListOdbPeeringConnectionsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::OdbPeeringConnectionSummary>> {
    let input = input.odb_peering_connections;
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_list_system_versions_output_output_system_versions(
    input: crate::operation::list_system_versions::ListSystemVersionsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::SystemVersionSummary>> {
    let input = input.system_versions;
    ::std::option::Option::Some(input)
}
