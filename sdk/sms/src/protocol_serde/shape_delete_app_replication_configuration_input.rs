// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_app_replication_configuration_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::delete_app_replication_configuration::DeleteAppReplicationConfigurationInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.app_id {
        object.key("appId").string(var_1.as_str());
    }
    Ok(())
}
