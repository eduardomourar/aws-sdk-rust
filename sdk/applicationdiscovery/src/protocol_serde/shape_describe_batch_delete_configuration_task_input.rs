// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_batch_delete_configuration_task_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::describe_batch_delete_configuration_task::DescribeBatchDeleteConfigurationTaskInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.task_id {
        object.key("taskId").string(var_1.as_str());
    }
    Ok(())
}
