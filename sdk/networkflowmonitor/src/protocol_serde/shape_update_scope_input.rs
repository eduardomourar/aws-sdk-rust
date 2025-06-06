// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_scope_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_scope::UpdateScopeInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.resources_to_add {
        let mut array_2 = object.key("resourcesToAdd").start_array();
        for item_3 in var_1 {
            {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_target_resource::ser_target_resource(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.resources_to_delete {
        let mut array_6 = object.key("resourcesToDelete").start_array();
        for item_7 in var_5 {
            {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_target_resource::ser_target_resource(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    Ok(())
}
