// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_notification_rules_filter(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ListNotificationRulesFilter,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("Name").string(input.name.as_str());
    }
    {
        object.key("Value").string(input.value.as_str());
    }
    Ok(())
}
