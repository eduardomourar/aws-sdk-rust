// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_disassociate_resource_share_permission_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::disassociate_resource_share_permission::DisassociateResourceSharePermissionInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("clientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.permission_arn {
        object.key("permissionArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.resource_share_arn {
        object.key("resourceShareArn").string(var_3.as_str());
    }
    Ok(())
}
