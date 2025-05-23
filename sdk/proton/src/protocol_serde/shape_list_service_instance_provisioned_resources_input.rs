// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_service_instance_provisioned_resources_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::list_service_instance_provisioned_resources::ListServiceInstanceProvisionedResourcesInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.service_name {
        object.key("serviceName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.service_instance_name {
        object.key("serviceInstanceName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.next_token {
        object.key("nextToken").string(var_3.as_str());
    }
    Ok(())
}
