// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_channel_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_channel::UpdateChannelInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.cdi_input_specification {
        #[allow(unused_mut)]
        let mut object_2 = object.key("cdiInputSpecification").start_object();
        crate::protocol_serde::shape_cdi_input_specification::ser_cdi_input_specification(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.channel_engine_version {
        #[allow(unused_mut)]
        let mut object_4 = object.key("channelEngineVersion").start_object();
        crate::protocol_serde::shape_channel_engine_version_request::ser_channel_engine_version_request(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.destinations {
        let mut array_6 = object.key("destinations").start_array();
        for item_7 in var_5 {
            {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_output_destination::ser_output_destination(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    if let Some(var_9) = &input.dry_run {
        object.key("dryRun").boolean(*var_9);
    }
    if let Some(var_10) = &input.encoder_settings {
        #[allow(unused_mut)]
        let mut object_11 = object.key("encoderSettings").start_object();
        crate::protocol_serde::shape_encoder_settings::ser_encoder_settings(&mut object_11, var_10)?;
        object_11.finish();
    }
    if let Some(var_12) = &input.input_attachments {
        let mut array_13 = object.key("inputAttachments").start_array();
        for item_14 in var_12 {
            {
                #[allow(unused_mut)]
                let mut object_15 = array_13.value().start_object();
                crate::protocol_serde::shape_input_attachment::ser_input_attachment(&mut object_15, item_14)?;
                object_15.finish();
            }
        }
        array_13.finish();
    }
    if let Some(var_16) = &input.input_specification {
        #[allow(unused_mut)]
        let mut object_17 = object.key("inputSpecification").start_object();
        crate::protocol_serde::shape_input_specification::ser_input_specification(&mut object_17, var_16)?;
        object_17.finish();
    }
    if let Some(var_18) = &input.log_level {
        object.key("logLevel").string(var_18.as_str());
    }
    if let Some(var_19) = &input.maintenance {
        #[allow(unused_mut)]
        let mut object_20 = object.key("maintenance").start_object();
        crate::protocol_serde::shape_maintenance_update_settings::ser_maintenance_update_settings(&mut object_20, var_19)?;
        object_20.finish();
    }
    if let Some(var_21) = &input.name {
        object.key("name").string(var_21.as_str());
    }
    if let Some(var_22) = &input.role_arn {
        object.key("roleArn").string(var_22.as_str());
    }
    Ok(())
}
