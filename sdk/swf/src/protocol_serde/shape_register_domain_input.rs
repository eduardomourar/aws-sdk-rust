// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_register_domain_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::register_domain::RegisterDomainInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.workflow_execution_retention_period_in_days {
        object.key("workflowExecutionRetentionPeriodInDays").string(var_3.as_str());
    }
    if let Some(var_4) = &input.tags {
        let mut array_5 = object.key("tags").start_array();
        for item_6 in var_4 {
            {
                #[allow(unused_mut)]
                let mut object_7 = array_5.value().start_object();
                crate::protocol_serde::shape_resource_tag::ser_resource_tag(&mut object_7, item_6)?;
                object_7.finish();
            }
        }
        array_5.finish();
    }
    Ok(())
}
