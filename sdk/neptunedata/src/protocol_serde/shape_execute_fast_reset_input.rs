// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_execute_fast_reset_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::execute_fast_reset::ExecuteFastResetInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.action {
        object.key("action").string(var_1.as_str());
    }
    if let Some(var_2) = &input.token {
        object.key("token").string(var_2.as_str());
    }
    Ok(())
}
