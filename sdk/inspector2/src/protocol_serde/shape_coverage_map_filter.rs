// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_coverage_map_filter(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::CoverageMapFilter,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("comparison").string(input.comparison.as_str());
    }
    {
        object.key("key").string(input.key.as_str());
    }
    if let Some(var_1) = &input.value {
        object.key("value").string(var_1.as_str());
    }
    Ok(())
}
