// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_environment_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_environment::UpdateEnvironmentInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.blueprint_version {
        object.key("blueprintVersion").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.glossary_terms {
        let mut array_4 = object.key("glossaryTerms").start_array();
        for item_5 in var_3 {
            {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    if let Some(var_6) = &input.name {
        object.key("name").string(var_6.as_str());
    }
    if let Some(var_7) = &input.user_parameters {
        let mut array_8 = object.key("userParameters").start_array();
        for item_9 in var_7 {
            {
                #[allow(unused_mut)]
                let mut object_10 = array_8.value().start_object();
                crate::protocol_serde::shape_environment_parameter::ser_environment_parameter(&mut object_10, item_9)?;
                object_10.finish();
            }
        }
        array_8.finish();
    }
    Ok(())
}
