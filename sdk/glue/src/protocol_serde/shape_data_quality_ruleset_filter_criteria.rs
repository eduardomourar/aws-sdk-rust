// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_data_quality_ruleset_filter_criteria(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::DataQualityRulesetFilterCriteria,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("Name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("Description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.created_before {
        object
            .key("CreatedBefore")
            .date_time(var_3, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_4) = &input.created_after {
        object
            .key("CreatedAfter")
            .date_time(var_4, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_5) = &input.last_modified_before {
        object
            .key("LastModifiedBefore")
            .date_time(var_5, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_6) = &input.last_modified_after {
        object
            .key("LastModifiedAfter")
            .date_time(var_6, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_7) = &input.target_table {
        #[allow(unused_mut)]
        let mut object_8 = object.key("TargetTable").start_object();
        crate::protocol_serde::shape_data_quality_target_table::ser_data_quality_target_table(&mut object_8, var_7)?;
        object_8.finish();
    }
    Ok(())
}
