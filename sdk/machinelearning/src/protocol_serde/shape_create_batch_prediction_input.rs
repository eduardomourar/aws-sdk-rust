// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_batch_prediction_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_batch_prediction::CreateBatchPredictionInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.batch_prediction_id {
        object.key("BatchPredictionId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.batch_prediction_name {
        object.key("BatchPredictionName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.ml_model_id {
        object.key("MLModelId").string(var_3.as_str());
    }
    if let Some(var_4) = &input.batch_prediction_data_source_id {
        object.key("BatchPredictionDataSourceId").string(var_4.as_str());
    }
    if let Some(var_5) = &input.output_uri {
        object.key("OutputUri").string(var_5.as_str());
    }
    Ok(())
}
