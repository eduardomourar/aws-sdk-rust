// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_connector_registrations_v2_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::connector_registrations_v2::ConnectorRegistrationsV2Input,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.auth_code {
        object.key("AuthCode").string(var_1.as_str());
    }
    if let Some(var_2) = &input.auth_state {
        object.key("AuthState").string(var_2.as_str());
    }
    Ok(())
}
