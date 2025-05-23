// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_endpoint_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_endpoint::CreateEndpointInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.endpoint_name {
        object.key("EndpointName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.model_arn {
        object.key("ModelArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.desired_inference_units {
        object.key("DesiredInferenceUnits").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_3).into()),
        );
    }
    if let Some(var_4) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_4.as_str());
    }
    if let Some(var_5) = &input.tags {
        let mut array_6 = object.key("Tags").start_array();
        for item_7 in var_5 {
            {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    if let Some(var_9) = &input.data_access_role_arn {
        object.key("DataAccessRoleArn").string(var_9.as_str());
    }
    if let Some(var_10) = &input.flywheel_arn {
        object.key("FlywheelArn").string(var_10.as_str());
    }
    Ok(())
}
