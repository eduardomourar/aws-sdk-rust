// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_suppressed_destination_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::put_suppressed_destination::PutSuppressedDestinationInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.email_address {
        object.key("EmailAddress").string(var_1.as_str());
    }
    if let Some(var_2) = &input.reason {
        object.key("Reason").string(var_2.as_str());
    }
    Ok(())
}
