// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_search_vulnerabilities_filter_criteria(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::SearchVulnerabilitiesFilterCriteria,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        let mut array_1 = object.key("vulnerabilityIds").start_array();
        for item_2 in &input.vulnerability_ids {
            {
                array_1.value().string(item_2.as_str());
            }
        }
        array_1.finish();
    }
    Ok(())
}
