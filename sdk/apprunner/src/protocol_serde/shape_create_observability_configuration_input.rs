// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_observability_configuration_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_observability_configuration::CreateObservabilityConfigurationInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.observability_configuration_name {
        object.key("ObservabilityConfigurationName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.trace_configuration {
        #[allow(unused_mut)]
        let mut object_3 = object.key("TraceConfiguration").start_object();
        crate::protocol_serde::shape_trace_configuration::ser_trace_configuration(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.tags {
        let mut array_5 = object.key("Tags").start_array();
        for item_6 in var_4 {
            {
                #[allow(unused_mut)]
                let mut object_7 = array_5.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_7, item_6)?;
                object_7.finish();
            }
        }
        array_5.finish();
    }
    Ok(())
}
