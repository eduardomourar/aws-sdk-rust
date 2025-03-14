// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_registration_association_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_registration_association::CreateRegistrationAssociationInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.registration_id {
        object.key("RegistrationId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.resource_id {
        object.key("ResourceId").string(var_2.as_str());
    }
    Ok(())
}
