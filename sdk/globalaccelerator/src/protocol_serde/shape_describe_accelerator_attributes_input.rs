// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_accelerator_attributes_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::describe_accelerator_attributes::DescribeAcceleratorAttributesInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.accelerator_arn {
        object.key("AcceleratorArn").string(var_1.as_str());
    }
    Ok(())
}
