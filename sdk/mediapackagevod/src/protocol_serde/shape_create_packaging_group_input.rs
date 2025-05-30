// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_packaging_group_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_packaging_group::CreatePackagingGroupInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.authorization {
        #[allow(unused_mut)]
        let mut object_2 = object.key("authorization").start_object();
        crate::protocol_serde::shape_authorization::ser_authorization(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.egress_access_logs {
        #[allow(unused_mut)]
        let mut object_4 = object.key("egressAccessLogs").start_object();
        crate::protocol_serde::shape_egress_access_logs::ser_egress_access_logs(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.id {
        object.key("id").string(var_5.as_str());
    }
    if let Some(var_6) = &input.tags {
        #[allow(unused_mut)]
        let mut object_7 = object.key("tags").start_object();
        for (key_8, value_9) in var_6 {
            {
                object_7.key(key_8.as_str()).string(value_9.as_str());
            }
        }
        object_7.finish();
    }
    Ok(())
}
