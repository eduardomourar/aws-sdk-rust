// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_admin_set_user_password_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::admin_set_user_password::AdminSetUserPasswordInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.user_pool_id {
        object.key("UserPoolId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.username {
        object.key("Username").string(var_2.as_str());
    }
    if let Some(var_3) = &input.password {
        object.key("Password").string(var_3.as_str());
    }
    if let Some(var_4) = &input.permanent {
        object.key("Permanent").boolean(*var_4);
    }
    Ok(())
}
