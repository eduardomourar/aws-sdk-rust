// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_location_object_storage_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_location_object_storage::UpdateLocationObjectStorageInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.location_arn {
        object.key("LocationArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.server_port {
        object.key("ServerPort").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    if let Some(var_3) = &input.server_protocol {
        object.key("ServerProtocol").string(var_3.as_str());
    }
    if let Some(var_4) = &input.subdirectory {
        object.key("Subdirectory").string(var_4.as_str());
    }
    if let Some(var_5) = &input.access_key {
        object.key("AccessKey").string(var_5.as_str());
    }
    if let Some(var_6) = &input.secret_key {
        object.key("SecretKey").string(var_6.as_str());
    }
    if let Some(var_7) = &input.agent_arns {
        let mut array_8 = object.key("AgentArns").start_array();
        for item_9 in var_7 {
            {
                array_8.value().string(item_9.as_str());
            }
        }
        array_8.finish();
    }
    if let Some(var_10) = &input.server_certificate {
        object
            .key("ServerCertificate")
            .string_unchecked(&::aws_smithy_types::base64::encode(var_10));
    }
    Ok(())
}
