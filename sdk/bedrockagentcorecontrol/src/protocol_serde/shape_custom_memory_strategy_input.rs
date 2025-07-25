// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_custom_memory_strategy_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::CustomMemoryStrategyInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("name").string(input.name.as_str());
    }
    if let Some(var_1) = &input.description {
        object.key("description").string(var_1.as_str());
    }
    if let Some(var_2) = &input.namespaces {
        let mut array_3 = object.key("namespaces").start_array();
        for item_4 in var_2 {
            {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    if let Some(var_5) = &input.configuration {
        #[allow(unused_mut)]
        let mut object_6 = object.key("configuration").start_object();
        crate::protocol_serde::shape_custom_configuration_input::ser_custom_configuration_input(&mut object_6, var_5)?;
        object_6.finish();
    }
    Ok(())
}
