// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_tag_search_condition(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::TagSearchCondition,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.tag_key {
        object.key("tagKey").string(var_1.as_str());
    }
    if let Some(var_2) = &input.tag_value {
        object.key("tagValue").string(var_2.as_str());
    }
    if let Some(var_3) = &input.tag_key_comparison_type {
        object.key("tagKeyComparisonType").string(var_3.as_str());
    }
    if let Some(var_4) = &input.tag_value_comparison_type {
        object.key("tagValueComparisonType").string(var_4.as_str());
    }
    Ok(())
}
