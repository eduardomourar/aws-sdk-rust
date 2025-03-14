// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_ice_server_config_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::get_ice_server_config::GetIceServerConfigInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.channel_arn {
        object.key("ChannelARN").string(var_1.as_str());
    }
    if let Some(var_2) = &input.client_id {
        object.key("ClientId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.service {
        object.key("Service").string(var_3.as_str());
    }
    if let Some(var_4) = &input.username {
        object.key("Username").string(var_4.as_str());
    }
    Ok(())
}
