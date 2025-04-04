// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_pipeline_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_pipeline::UpdatePipelineInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.pipeline_name {
        object.key("PipelineName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.pipeline_display_name {
        object.key("PipelineDisplayName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.pipeline_definition {
        object.key("PipelineDefinition").string(var_3.as_str());
    }
    if let Some(var_4) = &input.pipeline_definition_s3_location {
        #[allow(unused_mut)]
        let mut object_5 = object.key("PipelineDefinitionS3Location").start_object();
        crate::protocol_serde::shape_pipeline_definition_s3_location::ser_pipeline_definition_s3_location(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.pipeline_description {
        object.key("PipelineDescription").string(var_6.as_str());
    }
    if let Some(var_7) = &input.role_arn {
        object.key("RoleArn").string(var_7.as_str());
    }
    if let Some(var_8) = &input.parallelism_configuration {
        #[allow(unused_mut)]
        let mut object_9 = object.key("ParallelismConfiguration").start_object();
        crate::protocol_serde::shape_parallelism_configuration::ser_parallelism_configuration(&mut object_9, var_8)?;
        object_9.finish();
    }
    Ok(())
}
