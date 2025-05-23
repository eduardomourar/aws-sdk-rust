// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_import_key_pair_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::import_key_pair::ImportKeyPairInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.key_pair_name {
        object.key("keyPairName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.public_key_base64 {
        object.key("publicKeyBase64").string(var_2.as_str());
    }
    Ok(())
}
