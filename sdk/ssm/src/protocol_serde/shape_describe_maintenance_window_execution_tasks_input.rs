// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_maintenance_window_execution_tasks_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::describe_maintenance_window_execution_tasks::DescribeMaintenanceWindowExecutionTasksInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.window_execution_id {
        object.key("WindowExecutionId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.filters {
        let mut array_3 = object.key("Filters").start_array();
        for item_4 in var_2 {
            {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_maintenance_window_filter::ser_maintenance_window_filter(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    if let Some(var_6) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_6).into()),
        );
    }
    if let Some(var_7) = &input.next_token {
        object.key("NextToken").string(var_7.as_str());
    }
    Ok(())
}
