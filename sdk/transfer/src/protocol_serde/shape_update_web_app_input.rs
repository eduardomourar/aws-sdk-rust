// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_web_app_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_web_app::UpdateWebAppInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.web_app_id {
        object.key("WebAppId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.identity_provider_details {
        #[allow(unused_mut)]
        let mut object_3 = object.key("IdentityProviderDetails").start_object();
        crate::protocol_serde::shape_update_web_app_identity_provider_details::ser_update_web_app_identity_provider_details(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.access_endpoint {
        object.key("AccessEndpoint").string(var_4.as_str());
    }
    if let Some(var_5) = &input.web_app_units {
        #[allow(unused_mut)]
        let mut object_6 = object.key("WebAppUnits").start_object();
        crate::protocol_serde::shape_web_app_units::ser_web_app_units(&mut object_6, var_5)?;
        object_6.finish();
    }
    Ok(())
}
