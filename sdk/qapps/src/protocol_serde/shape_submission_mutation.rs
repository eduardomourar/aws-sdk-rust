// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_submission_mutation(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::SubmissionMutation,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("submissionId").string(input.submission_id.as_str());
    }
    {
        object.key("mutationType").string(input.mutation_type.as_str());
    }
    Ok(())
}
