// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aggregation_list_item(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AggregationListItem,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("attribute").string(input.attribute.as_str());
    }
    if let Some(var_1) = &input.display_value {
        object.key("displayValue").string(var_1.as_str());
    }
    Ok(())
}
