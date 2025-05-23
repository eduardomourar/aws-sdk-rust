// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_permission_set_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_permission_set::UpdatePermissionSetInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.instance_arn {
        object.key("InstanceArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.permission_set_arn {
        object.key("PermissionSetArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.description {
        object.key("Description").string(var_3.as_str());
    }
    if let Some(var_4) = &input.session_duration {
        object.key("SessionDuration").string(var_4.as_str());
    }
    if let Some(var_5) = &input.relay_state {
        object.key("RelayState").string(var_5.as_str());
    }
    Ok(())
}
