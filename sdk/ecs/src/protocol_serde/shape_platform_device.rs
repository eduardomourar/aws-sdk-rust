// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_platform_device(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::PlatformDevice,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("id").string(input.id.as_str());
    }
    {
        object.key("type").string(input.r#type.as_str());
    }
    Ok(())
}
