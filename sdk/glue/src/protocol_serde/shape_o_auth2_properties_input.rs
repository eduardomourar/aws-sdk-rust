// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_o_auth2_properties_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::OAuth2PropertiesInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.o_auth2_grant_type {
        object.key("OAuth2GrantType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.o_auth2_client_application {
        #[allow(unused_mut)]
        let mut object_3 = object.key("OAuth2ClientApplication").start_object();
        crate::protocol_serde::shape_o_auth2_client_application::ser_o_auth2_client_application(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.token_url {
        object.key("TokenUrl").string(var_4.as_str());
    }
    if let Some(var_5) = &input.token_url_parameters_map {
        #[allow(unused_mut)]
        let mut object_6 = object.key("TokenUrlParametersMap").start_object();
        for (key_7, value_8) in var_5 {
            {
                object_6.key(key_7.as_str()).string(value_8.as_str());
            }
        }
        object_6.finish();
    }
    if let Some(var_9) = &input.authorization_code_properties {
        #[allow(unused_mut)]
        let mut object_10 = object.key("AuthorizationCodeProperties").start_object();
        crate::protocol_serde::shape_authorization_code_properties::ser_authorization_code_properties(&mut object_10, var_9)?;
        object_10.finish();
    }
    if let Some(var_11) = &input.o_auth2_credentials {
        #[allow(unused_mut)]
        let mut object_12 = object.key("OAuth2Credentials").start_object();
        crate::protocol_serde::shape_o_auth2_credentials::ser_o_auth2_credentials(&mut object_12, var_11)?;
        object_12.finish();
    }
    Ok(())
}
