// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_batch_get_calculated_attribute_for_profile_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::batch_get_calculated_attribute_for_profile::BatchGetCalculatedAttributeForProfileInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.condition_overrides {
        #[allow(unused_mut)]
        let mut object_2 = object.key("ConditionOverrides").start_object();
        crate::protocol_serde::shape_condition_overrides::ser_condition_overrides(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.profile_ids {
        let mut array_4 = object.key("ProfileIds").start_array();
        for item_5 in var_3 {
            {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    Ok(())
}
