// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_revoke_signature_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::revoke_signature::RevokeSignatureInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.job_owner {
        object.key("jobOwner").string(var_1.as_str());
    }
    if let Some(var_2) = &input.reason {
        object.key("reason").string(var_2.as_str());
    }
    Ok(())
}
