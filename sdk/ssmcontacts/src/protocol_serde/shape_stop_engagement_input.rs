// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_stop_engagement_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::stop_engagement::StopEngagementInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.engagement_id {
        object.key("EngagementId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.reason {
        object.key("Reason").string(var_2.as_str());
    }
    Ok(())
}
