// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_dimension_key_details_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::get_dimension_key_details::GetDimensionKeyDetailsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.service_type {
        object.key("ServiceType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.identifier {
        object.key("Identifier").string(var_2.as_str());
    }
    if let Some(var_3) = &input.group {
        object.key("Group").string(var_3.as_str());
    }
    if let Some(var_4) = &input.group_identifier {
        object.key("GroupIdentifier").string(var_4.as_str());
    }
    if let Some(var_5) = &input.requested_dimensions {
        let mut array_6 = object.key("RequestedDimensions").start_array();
        for item_7 in var_5 {
            {
                array_6.value().string(item_7.as_str());
            }
        }
        array_6.finish();
    }
    Ok(())
}
