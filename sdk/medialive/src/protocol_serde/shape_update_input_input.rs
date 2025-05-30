// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_input_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_input::UpdateInputInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.destinations {
        let mut array_2 = object.key("destinations").start_array();
        for item_3 in var_1 {
            {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_input_destination_request::ser_input_destination_request(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.input_devices {
        let mut array_6 = object.key("inputDevices").start_array();
        for item_7 in var_5 {
            {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_input_device_request::ser_input_device_request(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    if let Some(var_9) = &input.input_security_groups {
        let mut array_10 = object.key("inputSecurityGroups").start_array();
        for item_11 in var_9 {
            {
                array_10.value().string(item_11.as_str());
            }
        }
        array_10.finish();
    }
    if let Some(var_12) = &input.media_connect_flows {
        let mut array_13 = object.key("mediaConnectFlows").start_array();
        for item_14 in var_12 {
            {
                #[allow(unused_mut)]
                let mut object_15 = array_13.value().start_object();
                crate::protocol_serde::shape_media_connect_flow_request::ser_media_connect_flow_request(&mut object_15, item_14)?;
                object_15.finish();
            }
        }
        array_13.finish();
    }
    if let Some(var_16) = &input.multicast_settings {
        #[allow(unused_mut)]
        let mut object_17 = object.key("multicastSettings").start_object();
        crate::protocol_serde::shape_multicast_settings_update_request::ser_multicast_settings_update_request(&mut object_17, var_16)?;
        object_17.finish();
    }
    if let Some(var_18) = &input.name {
        object.key("name").string(var_18.as_str());
    }
    if let Some(var_19) = &input.role_arn {
        object.key("roleArn").string(var_19.as_str());
    }
    if let Some(var_20) = &input.sdi_sources {
        let mut array_21 = object.key("sdiSources").start_array();
        for item_22 in var_20 {
            {
                array_21.value().string(item_22.as_str());
            }
        }
        array_21.finish();
    }
    if let Some(var_23) = &input.smpte2110_receiver_group_settings {
        #[allow(unused_mut)]
        let mut object_24 = object.key("smpte2110ReceiverGroupSettings").start_object();
        crate::protocol_serde::shape_smpte2110_receiver_group_settings::ser_smpte2110_receiver_group_settings(&mut object_24, var_23)?;
        object_24.finish();
    }
    if let Some(var_25) = &input.sources {
        let mut array_26 = object.key("sources").start_array();
        for item_27 in var_25 {
            {
                #[allow(unused_mut)]
                let mut object_28 = array_26.value().start_object();
                crate::protocol_serde::shape_input_source_request::ser_input_source_request(&mut object_28, item_27)?;
                object_28.finish();
            }
        }
        array_26.finish();
    }
    if let Some(var_29) = &input.srt_settings {
        #[allow(unused_mut)]
        let mut object_30 = object.key("srtSettings").start_object();
        crate::protocol_serde::shape_srt_settings_request::ser_srt_settings_request(&mut object_30, var_29)?;
        object_30.finish();
    }
    Ok(())
}
