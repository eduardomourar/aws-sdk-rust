// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_resource_data_sync_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_resource_data_sync::UpdateResourceDataSyncInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.sync_name {
        object.key("SyncName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.sync_type {
        object.key("SyncType").string(var_2.as_str());
    }
    if let Some(var_3) = &input.sync_source {
        #[allow(unused_mut)]
        let mut object_4 = object.key("SyncSource").start_object();
        crate::protocol_serde::shape_resource_data_sync_source::ser_resource_data_sync_source(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}
