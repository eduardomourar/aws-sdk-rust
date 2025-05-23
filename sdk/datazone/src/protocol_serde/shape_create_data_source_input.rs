// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_data_source_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_data_source::CreateDataSourceInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.asset_forms_input {
        let mut array_2 = object.key("assetFormsInput").start_array();
        for item_3 in var_1 {
            {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_form_input::ser_form_input(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.client_token {
        object.key("clientToken").string(var_5.as_str());
    }
    if let Some(var_6) = &input.configuration {
        #[allow(unused_mut)]
        let mut object_7 = object.key("configuration").start_object();
        crate::protocol_serde::shape_data_source_configuration_input::ser_data_source_configuration_input(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.connection_identifier {
        object.key("connectionIdentifier").string(var_8.as_str());
    }
    if let Some(var_9) = &input.description {
        object.key("description").string(var_9.as_str());
    }
    if let Some(var_10) = &input.enable_setting {
        object.key("enableSetting").string(var_10.as_str());
    }
    if let Some(var_11) = &input.environment_identifier {
        object.key("environmentIdentifier").string(var_11.as_str());
    }
    if let Some(var_12) = &input.name {
        object.key("name").string(var_12.as_str());
    }
    if let Some(var_13) = &input.project_identifier {
        object.key("projectIdentifier").string(var_13.as_str());
    }
    if let Some(var_14) = &input.publish_on_import {
        object.key("publishOnImport").boolean(*var_14);
    }
    if let Some(var_15) = &input.recommendation {
        #[allow(unused_mut)]
        let mut object_16 = object.key("recommendation").start_object();
        crate::protocol_serde::shape_recommendation_configuration::ser_recommendation_configuration(&mut object_16, var_15)?;
        object_16.finish();
    }
    if let Some(var_17) = &input.schedule {
        #[allow(unused_mut)]
        let mut object_18 = object.key("schedule").start_object();
        crate::protocol_serde::shape_schedule_configuration::ser_schedule_configuration(&mut object_18, var_17)?;
        object_18.finish();
    }
    if let Some(var_19) = &input.r#type {
        object.key("type").string(var_19.as_str());
    }
    Ok(())
}
