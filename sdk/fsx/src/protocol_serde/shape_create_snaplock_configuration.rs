// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_snaplock_configuration(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::CreateSnaplockConfiguration,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.audit_log_volume {
        object.key("AuditLogVolume").boolean(*var_1);
    }
    if let Some(var_2) = &input.autocommit_period {
        #[allow(unused_mut)]
        let mut object_3 = object.key("AutocommitPeriod").start_object();
        crate::protocol_serde::shape_autocommit_period::ser_autocommit_period(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.privileged_delete {
        object.key("PrivilegedDelete").string(var_4.as_str());
    }
    if let Some(var_5) = &input.retention_period {
        #[allow(unused_mut)]
        let mut object_6 = object.key("RetentionPeriod").start_object();
        crate::protocol_serde::shape_snaplock_retention_period::ser_snaplock_retention_period(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.snaplock_type {
        object.key("SnaplockType").string(var_7.as_str());
    }
    if let Some(var_8) = &input.volume_append_mode_enabled {
        object.key("VolumeAppendModeEnabled").boolean(*var_8);
    }
    Ok(())
}
