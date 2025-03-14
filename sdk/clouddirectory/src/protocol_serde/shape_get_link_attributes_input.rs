// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_link_attributes_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::get_link_attributes::GetLinkAttributesInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.attribute_names {
        let mut array_2 = object.key("AttributeNames").start_array();
        for item_3 in var_1 {
            {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.consistency_level {
        object.key("ConsistencyLevel").string(var_4.as_str());
    }
    if let Some(var_5) = &input.typed_link_specifier {
        #[allow(unused_mut)]
        let mut object_6 = object.key("TypedLinkSpecifier").start_object();
        crate::protocol_serde::shape_typed_link_specifier::ser_typed_link_specifier(&mut object_6, var_5)?;
        object_6.finish();
    }
    Ok(())
}
