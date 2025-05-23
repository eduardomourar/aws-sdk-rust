// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_repository_link_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_repository_link::UpdateRepositoryLinkInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.connection_arn {
        object.key("ConnectionArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.encryption_key_arn {
        object.key("EncryptionKeyArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.repository_link_id {
        object.key("RepositoryLinkId").string(var_3.as_str());
    }
    Ok(())
}
