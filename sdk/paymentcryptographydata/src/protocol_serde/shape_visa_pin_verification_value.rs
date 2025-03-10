// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_visa_pin_verification_value(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::VisaPinVerificationValue,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("EncryptedPinBlock").string(input.encrypted_pin_block.as_str());
    }
    {
        object.key("PinVerificationKeyIndex").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((input.pin_verification_key_index).into()),
        );
    }
    Ok(())
}
