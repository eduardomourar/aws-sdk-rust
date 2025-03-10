// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_dukpt_derivation_attributes(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::DukptDerivationAttributes,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("KeySerialNumber").string(input.key_serial_number.as_str());
    }
    if let Some(var_1) = &input.dukpt_key_derivation_type {
        object.key("DukptKeyDerivationType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.dukpt_key_variant {
        object.key("DukptKeyVariant").string(var_2.as_str());
    }
    Ok(())
}
