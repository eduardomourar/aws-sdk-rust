// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_autoshift_observer_notification_status_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_autoshift_observer_notification_status::UpdateAutoshiftObserverNotificationStatusInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.status {
        object.key("status").string(var_1.as_str());
    }
    Ok(())
}
