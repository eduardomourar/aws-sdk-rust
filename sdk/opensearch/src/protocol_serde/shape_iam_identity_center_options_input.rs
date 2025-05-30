// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_iam_identity_center_options_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::IamIdentityCenterOptionsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.enabled {
        object.key("enabled").boolean(*var_1);
    }
    if let Some(var_2) = &input.iam_identity_center_instance_arn {
        object.key("iamIdentityCenterInstanceArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.iam_role_for_identity_center_application_arn {
        object.key("iamRoleForIdentityCenterApplicationArn").string(var_3.as_str());
    }
    Ok(())
}
