// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_default_retention_policy_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::get_default_retention_policy::GetDefaultRetentionPolicyInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.organization_id {
        object.key("OrganizationId").string(var_1.as_str());
    }
    Ok(())
}
