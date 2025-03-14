// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_group_version_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_group_version::CreateGroupVersionInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.connector_definition_version_arn {
        object.key("ConnectorDefinitionVersionArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.core_definition_version_arn {
        object.key("CoreDefinitionVersionArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.device_definition_version_arn {
        object.key("DeviceDefinitionVersionArn").string(var_3.as_str());
    }
    if let Some(var_4) = &input.function_definition_version_arn {
        object.key("FunctionDefinitionVersionArn").string(var_4.as_str());
    }
    if let Some(var_5) = &input.logger_definition_version_arn {
        object.key("LoggerDefinitionVersionArn").string(var_5.as_str());
    }
    if let Some(var_6) = &input.resource_definition_version_arn {
        object.key("ResourceDefinitionVersionArn").string(var_6.as_str());
    }
    if let Some(var_7) = &input.subscription_definition_version_arn {
        object.key("SubscriptionDefinitionVersionArn").string(var_7.as_str());
    }
    Ok(())
}
