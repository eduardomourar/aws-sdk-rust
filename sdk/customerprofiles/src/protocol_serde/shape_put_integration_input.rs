// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_integration_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::put_integration::PutIntegrationInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.flow_definition {
        #[allow(unused_mut)]
        let mut object_2 = object.key("FlowDefinition").start_object();
        crate::protocol_serde::shape_flow_definition::ser_flow_definition(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.object_type_name {
        object.key("ObjectTypeName").string(var_3.as_str());
    }
    if let Some(var_4) = &input.object_type_names {
        #[allow(unused_mut)]
        let mut object_5 = object.key("ObjectTypeNames").start_object();
        for (key_6, value_7) in var_4 {
            {
                object_5.key(key_6.as_str()).string(value_7.as_str());
            }
        }
        object_5.finish();
    }
    if let Some(var_8) = &input.role_arn {
        object.key("RoleArn").string(var_8.as_str());
    }
    if let Some(var_9) = &input.tags {
        #[allow(unused_mut)]
        let mut object_10 = object.key("Tags").start_object();
        for (key_11, value_12) in var_9 {
            {
                object_10.key(key_11.as_str()).string(value_12.as_str());
            }
        }
        object_10.finish();
    }
    if let Some(var_13) = &input.uri {
        object.key("Uri").string(var_13.as_str());
    }
    Ok(())
}
