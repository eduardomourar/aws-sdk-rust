// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_product_view_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::describe_product_view::DescribeProductViewInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.accept_language {
        object.key("AcceptLanguage").string(var_1.as_str());
    }
    if let Some(var_2) = &input.id {
        object.key("Id").string(var_2.as_str());
    }
    Ok(())
}
