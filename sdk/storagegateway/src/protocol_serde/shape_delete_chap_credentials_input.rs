// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_chap_credentials_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::delete_chap_credentials::DeleteChapCredentialsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.target_arn {
        object.key("TargetARN").string(var_1.as_str());
    }
    if let Some(var_2) = &input.initiator_name {
        object.key("InitiatorName").string(var_2.as_str());
    }
    Ok(())
}
