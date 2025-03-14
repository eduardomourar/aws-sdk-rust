// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_temporary_glue_partition_credentials_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::get_temporary_glue_partition_credentials::GetTemporaryGluePartitionCredentialsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.audit_context {
        #[allow(unused_mut)]
        let mut object_2 = object.key("AuditContext").start_object();
        crate::protocol_serde::shape_audit_context::ser_audit_context(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.duration_seconds {
        object.key("DurationSeconds").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_3).into()),
        );
    }
    if let Some(var_4) = &input.partition {
        #[allow(unused_mut)]
        let mut object_5 = object.key("Partition").start_object();
        crate::protocol_serde::shape_partition_value_list::ser_partition_value_list(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.permissions {
        let mut array_7 = object.key("Permissions").start_array();
        for item_8 in var_6 {
            {
                array_7.value().string(item_8.as_str());
            }
        }
        array_7.finish();
    }
    if let Some(var_9) = &input.supported_permission_types {
        let mut array_10 = object.key("SupportedPermissionTypes").start_array();
        for item_11 in var_9 {
            {
                array_10.value().string(item_11.as_str());
            }
        }
        array_10.finish();
    }
    if let Some(var_12) = &input.table_arn {
        object.key("TableArn").string(var_12.as_str());
    }
    Ok(())
}
