// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_evaluate_feature_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::evaluate_feature::EvaluateFeatureInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.entity_id {
        object.key("entityId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.evaluation_context {
        object.key("evaluationContext").string(var_2.as_str());
    }
    Ok(())
}
