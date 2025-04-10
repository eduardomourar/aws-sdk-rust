// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_gdg_attributes(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::GdgAttributes,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if input.limit != 0 {
        object.key("limit").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.limit).into()),
        );
    }
    if let Some(var_1) = &input.roll_disposition {
        object.key("rollDisposition").string(var_1.as_str());
    }
    Ok(())
}
