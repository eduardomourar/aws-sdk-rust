// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_sidewalk_start_import_info(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::SidewalkStartImportInfo,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.device_creation_file {
        object.key("DeviceCreationFile").string(var_1.as_str());
    }
    if let Some(var_2) = &input.role {
        object.key("Role").string(var_2.as_str());
    }
    Ok(())
}
