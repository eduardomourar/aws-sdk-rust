// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_notification_setting_key(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::NotificationSettingKey,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("event").string(input.event.as_str());
    }
    if let Some(var_1) = &input.channel {
        object.key("channel").string(var_1.as_str());
    }
    Ok(())
}
