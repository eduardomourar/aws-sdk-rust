// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_event_action_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_event_action::UpdateEventActionInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.action {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Action").start_object();
        crate::protocol_serde::shape_action::ser_action(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}
