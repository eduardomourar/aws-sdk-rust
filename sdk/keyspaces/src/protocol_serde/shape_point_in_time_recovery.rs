// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_point_in_time_recovery(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::PointInTimeRecovery,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("status").string(input.status.as_str());
    }
    Ok(())
}
