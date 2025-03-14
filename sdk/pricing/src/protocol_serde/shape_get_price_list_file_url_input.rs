// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_price_list_file_url_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::get_price_list_file_url::GetPriceListFileUrlInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.price_list_arn {
        object.key("PriceListArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.file_format {
        object.key("FileFormat").string(var_2.as_str());
    }
    Ok(())
}
