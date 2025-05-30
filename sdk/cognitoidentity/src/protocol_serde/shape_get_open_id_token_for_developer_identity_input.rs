// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_open_id_token_for_developer_identity_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::get_open_id_token_for_developer_identity::GetOpenIdTokenForDeveloperIdentityInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.identity_pool_id {
        object.key("IdentityPoolId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.identity_id {
        object.key("IdentityId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.logins {
        #[allow(unused_mut)]
        let mut object_4 = object.key("Logins").start_object();
        for (key_5, value_6) in var_3 {
            {
                object_4.key(key_5.as_str()).string(value_6.as_str());
            }
        }
        object_4.finish();
    }
    if let Some(var_7) = &input.principal_tags {
        #[allow(unused_mut)]
        let mut object_8 = object.key("PrincipalTags").start_object();
        for (key_9, value_10) in var_7 {
            {
                object_8.key(key_9.as_str()).string(value_10.as_str());
            }
        }
        object_8.finish();
    }
    if let Some(var_11) = &input.token_duration {
        object.key("TokenDuration").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_11).into()),
        );
    }
    Ok(())
}
