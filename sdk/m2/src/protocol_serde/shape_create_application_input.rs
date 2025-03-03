// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_application_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_application::CreateApplicationInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("clientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.definition {
        #[allow(unused_mut)]
        let mut object_3 = object.key("definition").start_object();
        crate::protocol_serde::shape_definition::ser_definition(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.description {
        object.key("description").string(var_4.as_str());
    }
    if let Some(var_5) = &input.engine_type {
        object.key("engineType").string(var_5.as_str());
    }
    if let Some(var_6) = &input.kms_key_id {
        object.key("kmsKeyId").string(var_6.as_str());
    }
    if let Some(var_7) = &input.name {
        object.key("name").string(var_7.as_str());
    }
    if let Some(var_8) = &input.role_arn {
        object.key("roleArn").string(var_8.as_str());
    }
    if let Some(var_9) = &input.tags {
        #[allow(unused_mut)]
        let mut object_10 = object.key("tags").start_object();
        for (key_11, value_12) in var_9 {
            {
                object_10.key(key_11.as_str()).string(value_12.as_str());
            }
        }
        object_10.finish();
    }
    Ok(())
}
