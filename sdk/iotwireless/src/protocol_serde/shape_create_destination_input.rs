// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_destination_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_destination::CreateDestinationInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("Description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.expression {
        object.key("Expression").string(var_3.as_str());
    }
    if let Some(var_4) = &input.expression_type {
        object.key("ExpressionType").string(var_4.as_str());
    }
    if let Some(var_5) = &input.name {
        object.key("Name").string(var_5.as_str());
    }
    if let Some(var_6) = &input.role_arn {
        object.key("RoleArn").string(var_6.as_str());
    }
    if let Some(var_7) = &input.tags {
        let mut array_8 = object.key("Tags").start_array();
        for item_9 in var_7 {
            {
                #[allow(unused_mut)]
                let mut object_10 = array_8.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_10, item_9)?;
                object_10.finish();
            }
        }
        array_8.finish();
    }
    Ok(())
}
