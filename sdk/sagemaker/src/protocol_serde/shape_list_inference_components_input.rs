// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_inference_components_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::list_inference_components::ListInferenceComponentsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.sort_by {
        object.key("SortBy").string(var_1.as_str());
    }
    if let Some(var_2) = &input.sort_order {
        object.key("SortOrder").string(var_2.as_str());
    }
    if let Some(var_3) = &input.next_token {
        object.key("NextToken").string(var_3.as_str());
    }
    if let Some(var_4) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    if let Some(var_5) = &input.name_contains {
        object.key("NameContains").string(var_5.as_str());
    }
    if let Some(var_6) = &input.creation_time_before {
        object
            .key("CreationTimeBefore")
            .date_time(var_6, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_7) = &input.creation_time_after {
        object
            .key("CreationTimeAfter")
            .date_time(var_7, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_8) = &input.last_modified_time_before {
        object
            .key("LastModifiedTimeBefore")
            .date_time(var_8, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_9) = &input.last_modified_time_after {
        object
            .key("LastModifiedTimeAfter")
            .date_time(var_9, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_10) = &input.status_equals {
        object.key("StatusEquals").string(var_10.as_str());
    }
    if let Some(var_11) = &input.endpoint_name_equals {
        object.key("EndpointNameEquals").string(var_11.as_str());
    }
    if let Some(var_12) = &input.variant_name_equals {
        object.key("VariantNameEquals").string(var_12.as_str());
    }
    Ok(())
}
