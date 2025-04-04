// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_activation_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_activation::CreateActivationInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.description {
        object.key("Description").string(var_1.as_str());
    }
    if let Some(var_2) = &input.default_instance_name {
        object.key("DefaultInstanceName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.iam_role {
        object.key("IamRole").string(var_3.as_str());
    }
    if let Some(var_4) = &input.registration_limit {
        object.key("RegistrationLimit").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    if let Some(var_5) = &input.expiration_date {
        object
            .key("ExpirationDate")
            .date_time(var_5, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_6) = &input.tags {
        let mut array_7 = object.key("Tags").start_array();
        for item_8 in var_6 {
            {
                #[allow(unused_mut)]
                let mut object_9 = array_7.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_9, item_8)?;
                object_9.finish();
            }
        }
        array_7.finish();
    }
    if let Some(var_10) = &input.registration_metadata {
        let mut array_11 = object.key("RegistrationMetadata").start_array();
        for item_12 in var_10 {
            {
                #[allow(unused_mut)]
                let mut object_13 = array_11.value().start_object();
                crate::protocol_serde::shape_registration_metadata_item::ser_registration_metadata_item(&mut object_13, item_12)?;
                object_13.finish();
            }
        }
        array_11.finish();
    }
    Ok(())
}
