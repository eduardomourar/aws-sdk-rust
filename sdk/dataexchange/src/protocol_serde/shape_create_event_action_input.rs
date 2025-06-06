// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_event_action_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_event_action::CreateEventActionInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.action {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Action").start_object();
        crate::protocol_serde::shape_action::ser_action(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.event {
        #[allow(unused_mut)]
        let mut object_4 = object.key("Event").start_object();
        crate::protocol_serde::shape_event::ser_event(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.tags {
        #[allow(unused_mut)]
        let mut object_6 = object.key("Tags").start_object();
        for (key_7, value_8) in var_5 {
            {
                object_6.key(key_7.as_str()).string(value_8.as_str());
            }
        }
        object_6.finish();
    }
    Ok(())
}
