// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_endpoint_configuration(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::EndpointConfiguration,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.endpoint_id {
        object.key("EndpointId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.weight {
        object.key("Weight").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    if let Some(var_3) = &input.client_ip_preservation_enabled {
        object.key("ClientIPPreservationEnabled").boolean(*var_3);
    }
    if let Some(var_4) = &input.attachment_arn {
        object.key("AttachmentArn").string(var_4.as_str());
    }
    Ok(())
}
