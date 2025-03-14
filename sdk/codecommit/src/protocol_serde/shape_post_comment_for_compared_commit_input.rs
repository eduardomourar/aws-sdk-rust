// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_post_comment_for_compared_commit_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::post_comment_for_compared_commit::PostCommentForComparedCommitInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.repository_name {
        object.key("repositoryName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.before_commit_id {
        object.key("beforeCommitId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.after_commit_id {
        object.key("afterCommitId").string(var_3.as_str());
    }
    if let Some(var_4) = &input.location {
        #[allow(unused_mut)]
        let mut object_5 = object.key("location").start_object();
        crate::protocol_serde::shape_location::ser_location(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.content {
        object.key("content").string(var_6.as_str());
    }
    if let Some(var_7) = &input.client_request_token {
        object.key("clientRequestToken").string(var_7.as_str());
    }
    Ok(())
}
