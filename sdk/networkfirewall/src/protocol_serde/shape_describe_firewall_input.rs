// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_firewall_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::describe_firewall::DescribeFirewallInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.firewall_name {
        object.key("FirewallName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.firewall_arn {
        object.key("FirewallArn").string(var_2.as_str());
    }
    Ok(())
}
