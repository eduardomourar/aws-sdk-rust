// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_profile_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_profile::UpdateProfileInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.profile_id {
        object.key("profileId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.name {
        object.key("name").string(var_2.as_str());
    }
    if let Some(var_3) = &input.email {
        object.key("email").string(var_3.as_str());
    }
    if let Some(var_4) = &input.phone {
        object.key("phone").string(var_4.as_str());
    }
    if let Some(var_5) = &input.business_name {
        object.key("businessName").string(var_5.as_str());
    }
    Ok(())
}
