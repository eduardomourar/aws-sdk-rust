// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_extended_key_usage(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ExtendedKeyUsage,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.extended_key_usage_type {
        object.key("ExtendedKeyUsageType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.extended_key_usage_object_identifier {
        object.key("ExtendedKeyUsageObjectIdentifier").string(var_2.as_str());
    }
    Ok(())
}
