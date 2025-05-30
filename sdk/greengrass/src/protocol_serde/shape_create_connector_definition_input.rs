// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_connector_definition_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_connector_definition::CreateConnectorDefinitionInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.initial_version {
        #[allow(unused_mut)]
        let mut object_2 = object.key("InitialVersion").start_object();
        crate::protocol_serde::shape_connector_definition_version::ser_connector_definition_version(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.name {
        object.key("Name").string(var_3.as_str());
    }
    if let Some(var_4) = &input.tags {
        #[allow(unused_mut)]
        let mut object_5 = object.key("tags").start_object();
        for (key_6, value_7) in var_4 {
            {
                object_5.key(key_6.as_str()).string(value_7.as_str());
            }
        }
        object_5.finish();
    }
    Ok(())
}
