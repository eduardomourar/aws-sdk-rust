// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_custom_auth_credentials(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::CustomAuthCredentials,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("customAuthenticationType").string(input.custom_authentication_type.as_str());
    }
    if let Some(var_1) = &input.credentials_map {
        #[allow(unused_mut)]
        let mut object_2 = object.key("credentialsMap").start_object();
        for (key_3, value_4) in var_1 {
            {
                object_2.key(key_3.as_str()).string(value_4.as_str());
            }
        }
        object_2.finish();
    }
    Ok(())
}
