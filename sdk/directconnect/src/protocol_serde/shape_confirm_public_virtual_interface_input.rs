// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_confirm_public_virtual_interface_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::confirm_public_virtual_interface::ConfirmPublicVirtualInterfaceInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.virtual_interface_id {
        object.key("virtualInterfaceId").string(var_1.as_str());
    }
    Ok(())
}
