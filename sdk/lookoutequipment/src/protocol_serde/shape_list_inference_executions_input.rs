// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_inference_executions_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::list_inference_executions::ListInferenceExecutionsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.next_token {
        object.key("NextToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    if let Some(var_3) = &input.inference_scheduler_name {
        object.key("InferenceSchedulerName").string(var_3.as_str());
    }
    if let Some(var_4) = &input.data_start_time_after {
        object
            .key("DataStartTimeAfter")
            .date_time(var_4, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_5) = &input.data_end_time_before {
        object
            .key("DataEndTimeBefore")
            .date_time(var_5, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_6) = &input.status {
        object.key("Status").string(var_6.as_str());
    }
    Ok(())
}
