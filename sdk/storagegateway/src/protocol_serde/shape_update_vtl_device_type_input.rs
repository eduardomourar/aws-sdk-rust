// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_vtl_device_type_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_vtl_device_type::UpdateVtlDeviceTypeInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.vtl_device_arn {
        object.key("VTLDeviceARN").string(var_1.as_str());
    }
    if let Some(var_2) = &input.device_type {
        object.key("DeviceType").string(var_2.as_str());
    }
    Ok(())
}
