// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_cloud_vm_cluster_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::get_cloud_vm_cluster::GetCloudVmClusterInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.cloud_vm_cluster_id {
        object.key("cloudVmClusterId").string(var_1.as_str());
    }
    Ok(())
}
