// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_hsm_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::describe_hsm::DescribeHsmInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.hsm_arn {
        object.key("HsmArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.hsm_serial_number {
        object.key("HsmSerialNumber").string(var_2.as_str());
    }
    Ok(())
}
