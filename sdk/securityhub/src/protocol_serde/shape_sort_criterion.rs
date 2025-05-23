// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_sort_criterion(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::SortCriterion,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.field {
        object.key("Field").string(var_1.as_str());
    }
    if let Some(var_2) = &input.sort_order {
        object.key("SortOrder").string(var_2.as_str());
    }
    Ok(())
}
