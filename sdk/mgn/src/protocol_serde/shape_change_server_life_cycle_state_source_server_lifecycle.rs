// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_change_server_life_cycle_state_source_server_lifecycle(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ChangeServerLifeCycleStateSourceServerLifecycle,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("state").string(input.state.as_str());
    }
    Ok(())
}
