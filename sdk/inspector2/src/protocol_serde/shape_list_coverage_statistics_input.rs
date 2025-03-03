// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_coverage_statistics_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::list_coverage_statistics::ListCoverageStatisticsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.filter_criteria {
        #[allow(unused_mut)]
        let mut object_2 = object.key("filterCriteria").start_object();
        crate::protocol_serde::shape_coverage_filter_criteria::ser_coverage_filter_criteria(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.group_by {
        object.key("groupBy").string(var_3.as_str());
    }
    if let Some(var_4) = &input.next_token {
        object.key("nextToken").string(var_4.as_str());
    }
    Ok(())
}
