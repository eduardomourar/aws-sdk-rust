// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_cloud_connector_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_cloud_connector::CreateCloudConnectorInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("ClientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("Description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.endpoint_config {
        #[allow(unused_mut)]
        let mut object_4 = object.key("EndpointConfig").start_object();
        crate::protocol_serde::shape_endpoint_config::ser_endpoint_config(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.endpoint_type {
        object.key("EndpointType").string(var_5.as_str());
    }
    if let Some(var_6) = &input.name {
        object.key("Name").string(var_6.as_str());
    }
    Ok(())
}
