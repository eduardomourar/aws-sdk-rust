// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_identity_center_configuration_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::IdentityCenterConfigurationInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.identity_center_instance_arn {
        object.key("identityCenterInstanceArn").string(var_1.as_str());
    }
    Ok(())
}
