// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_tls_context(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::TlsContext,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.server_name {
        object.key("serverName").string(var_1.as_str());
    }
    Ok(())
}
