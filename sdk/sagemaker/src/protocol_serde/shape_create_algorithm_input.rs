// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_algorithm_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_algorithm::CreateAlgorithmInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.algorithm_name {
        object.key("AlgorithmName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.algorithm_description {
        object.key("AlgorithmDescription").string(var_2.as_str());
    }
    if let Some(var_3) = &input.training_specification {
        #[allow(unused_mut)]
        let mut object_4 = object.key("TrainingSpecification").start_object();
        crate::protocol_serde::shape_training_specification::ser_training_specification(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.inference_specification {
        #[allow(unused_mut)]
        let mut object_6 = object.key("InferenceSpecification").start_object();
        crate::protocol_serde::shape_inference_specification::ser_inference_specification(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.validation_specification {
        #[allow(unused_mut)]
        let mut object_8 = object.key("ValidationSpecification").start_object();
        crate::protocol_serde::shape_algorithm_validation_specification::ser_algorithm_validation_specification(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.certify_for_marketplace {
        object.key("CertifyForMarketplace").boolean(*var_9);
    }
    if let Some(var_10) = &input.tags {
        let mut array_11 = object.key("Tags").start_array();
        for item_12 in var_10 {
            {
                #[allow(unused_mut)]
                let mut object_13 = array_11.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_13, item_12)?;
                object_13.finish();
            }
        }
        array_11.finish();
    }
    Ok(())
}
