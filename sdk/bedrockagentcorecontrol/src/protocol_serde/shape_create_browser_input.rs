// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_browser_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_browser::CreateBrowserInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("clientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.execution_role_arn {
        object.key("executionRoleArn").string(var_3.as_str());
    }
    if let Some(var_4) = &input.name {
        object.key("name").string(var_4.as_str());
    }
    if let Some(var_5) = &input.network_configuration {
        #[allow(unused_mut)]
        let mut object_6 = object.key("networkConfiguration").start_object();
        crate::protocol_serde::shape_browser_network_configuration::ser_browser_network_configuration(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.recording {
        #[allow(unused_mut)]
        let mut object_8 = object.key("recording").start_object();
        crate::protocol_serde::shape_recording_config::ser_recording_config(&mut object_8, var_7)?;
        object_8.finish();
    }
    Ok(())
}
