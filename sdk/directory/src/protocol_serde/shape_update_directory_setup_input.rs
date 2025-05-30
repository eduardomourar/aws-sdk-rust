// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_directory_setup_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_directory_setup::UpdateDirectorySetupInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.directory_id {
        object.key("DirectoryId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.update_type {
        object.key("UpdateType").string(var_2.as_str());
    }
    if let Some(var_3) = &input.os_update_settings {
        #[allow(unused_mut)]
        let mut object_4 = object.key("OSUpdateSettings").start_object();
        crate::protocol_serde::shape_os_update_settings::ser_os_update_settings(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.create_snapshot_before_update {
        object.key("CreateSnapshotBeforeUpdate").boolean(*var_5);
    }
    Ok(())
}
