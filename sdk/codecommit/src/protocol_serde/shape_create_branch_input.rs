// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_branch_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_branch::CreateBranchInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.repository_name {
        object.key("repositoryName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.branch_name {
        object.key("branchName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.commit_id {
        object.key("commitId").string(var_3.as_str());
    }
    Ok(())
}
