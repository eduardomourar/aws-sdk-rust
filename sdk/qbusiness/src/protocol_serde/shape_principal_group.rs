// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_principal_group(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::PrincipalGroup,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("name").string(var_1.as_str());
    }
    {
        object.key("access").string(input.access.as_str());
    }
    if let Some(var_2) = &input.membership_type {
        object.key("membershipType").string(var_2.as_str());
    }
    Ok(())
}
