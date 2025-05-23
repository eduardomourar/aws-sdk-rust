// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_configured_model_algorithm_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_configured_model_algorithm::CreateConfiguredModelAlgorithmInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.description {
        object.key("description").string(var_1.as_str());
    }
    if let Some(var_2) = &input.inference_container_config {
        #[allow(unused_mut)]
        let mut object_3 = object.key("inferenceContainerConfig").start_object();
        crate::protocol_serde::shape_inference_container_config::ser_inference_container_config(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.kms_key_arn {
        object.key("kmsKeyArn").string(var_4.as_str());
    }
    if let Some(var_5) = &input.name {
        object.key("name").string(var_5.as_str());
    }
    if let Some(var_6) = &input.role_arn {
        object.key("roleArn").string(var_6.as_str());
    }
    if let Some(var_7) = &input.tags {
        #[allow(unused_mut)]
        let mut object_8 = object.key("tags").start_object();
        for (key_9, value_10) in var_7 {
            {
                object_8.key(key_9.as_str()).string(value_10.as_str());
            }
        }
        object_8.finish();
    }
    if let Some(var_11) = &input.training_container_config {
        #[allow(unused_mut)]
        let mut object_12 = object.key("trainingContainerConfig").start_object();
        crate::protocol_serde::shape_container_config::ser_container_config(&mut object_12, var_11)?;
        object_12.finish();
    }
    Ok(())
}
