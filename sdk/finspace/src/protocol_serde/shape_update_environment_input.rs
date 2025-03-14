// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_environment_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_environment::UpdateEnvironmentInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.description {
        object.key("description").string(var_1.as_str());
    }
    if let Some(var_2) = &input.federation_mode {
        object.key("federationMode").string(var_2.as_str());
    }
    if let Some(var_3) = &input.federation_parameters {
        #[allow(unused_mut)]
        let mut object_4 = object.key("federationParameters").start_object();
        crate::protocol_serde::shape_federation_parameters::ser_federation_parameters(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.name {
        object.key("name").string(var_5.as_str());
    }
    Ok(())
}
