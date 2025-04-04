// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_open_id_connect_group_configuration(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::OpenIdConnectGroupConfiguration,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("groupClaim").string(input.group_claim.as_str());
    }
    {
        object.key("groupEntityType").string(input.group_entity_type.as_str());
    }
    Ok(())
}
