// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_user_jobs_first(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::UserJobsFirst,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("userIdentityId").string(input.user_identity_id.as_str());
    }
    Ok(())
}
