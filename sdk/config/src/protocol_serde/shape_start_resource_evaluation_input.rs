// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_resource_evaluation_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::start_resource_evaluation::StartResourceEvaluationInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.resource_details {
        #[allow(unused_mut)]
        let mut object_2 = object.key("ResourceDetails").start_object();
        crate::protocol_serde::shape_resource_details::ser_resource_details(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.evaluation_context {
        #[allow(unused_mut)]
        let mut object_4 = object.key("EvaluationContext").start_object();
        crate::protocol_serde::shape_evaluation_context::ser_evaluation_context(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.evaluation_mode {
        object.key("EvaluationMode").string(var_5.as_str());
    }
    if let Some(var_6) = &input.evaluation_timeout {
        object.key("EvaluationTimeout").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_6).into()),
        );
    }
    if let Some(var_7) = &input.client_token {
        object.key("ClientToken").string(var_7.as_str());
    }
    Ok(())
}
