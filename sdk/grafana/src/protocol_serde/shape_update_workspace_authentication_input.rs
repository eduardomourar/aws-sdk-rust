// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_workspace_authentication_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_workspace_authentication::UpdateWorkspaceAuthenticationInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.authentication_providers {
        let mut array_2 = object.key("authenticationProviders").start_array();
        for item_3 in var_1 {
            {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.saml_configuration {
        #[allow(unused_mut)]
        let mut object_5 = object.key("samlConfiguration").start_object();
        crate::protocol_serde::shape_saml_configuration::ser_saml_configuration(&mut object_5, var_4)?;
        object_5.finish();
    }
    Ok(())
}
