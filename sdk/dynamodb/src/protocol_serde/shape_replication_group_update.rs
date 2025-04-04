// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_replication_group_update(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ReplicationGroupUpdate,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.create {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Create").start_object();
        crate::protocol_serde::shape_create_replication_group_member_action::ser_create_replication_group_member_action(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.update {
        #[allow(unused_mut)]
        let mut object_4 = object.key("Update").start_object();
        crate::protocol_serde::shape_update_replication_group_member_action::ser_update_replication_group_member_action(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.delete {
        #[allow(unused_mut)]
        let mut object_6 = object.key("Delete").start_object();
        crate::protocol_serde::shape_delete_replication_group_member_action::ser_delete_replication_group_member_action(&mut object_6, var_5)?;
        object_6.finish();
    }
    Ok(())
}
