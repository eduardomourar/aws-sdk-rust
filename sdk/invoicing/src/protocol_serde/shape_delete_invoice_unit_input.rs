// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_invoice_unit_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::delete_invoice_unit::DeleteInvoiceUnitInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.invoice_unit_arn {
        object.key("InvoiceUnitArn").string(var_1.as_str());
    }
    Ok(())
}
