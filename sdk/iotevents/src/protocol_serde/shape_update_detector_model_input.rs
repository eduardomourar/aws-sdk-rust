// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_detector_model_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_detector_model::UpdateDetectorModelInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.detector_model_definition {
        #[allow(unused_mut)]
        let mut object_2 = object.key("detectorModelDefinition").start_object();
        crate::protocol_serde::shape_detector_model_definition::ser_detector_model_definition(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.detector_model_description {
        object.key("detectorModelDescription").string(var_3.as_str());
    }
    if let Some(var_4) = &input.evaluation_method {
        object.key("evaluationMethod").string(var_4.as_str());
    }
    if let Some(var_5) = &input.role_arn {
        object.key("roleArn").string(var_5.as_str());
    }
    Ok(())
}
