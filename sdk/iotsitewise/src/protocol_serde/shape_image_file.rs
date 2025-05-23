// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_image_file(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ImageFile,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("data").string_unchecked(&::aws_smithy_types::base64::encode(&input.data));
    }
    {
        object.key("type").string(input.r#type.as_str());
    }
    Ok(())
}
