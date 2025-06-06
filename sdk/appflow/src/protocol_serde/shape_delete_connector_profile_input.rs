// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_connector_profile_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::delete_connector_profile::DeleteConnectorProfileInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.connector_profile_name {
        object.key("connectorProfileName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.force_delete {
        object.key("forceDelete").boolean(*var_2);
    }
    Ok(())
}
