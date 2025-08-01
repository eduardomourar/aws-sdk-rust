// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_resource_tenants_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::list_resource_tenants::ListResourceTenantsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.next_token {
        object.key("NextToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.page_size {
        object.key("PageSize").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    if let Some(var_3) = &input.resource_arn {
        object.key("ResourceArn").string(var_3.as_str());
    }
    Ok(())
}
