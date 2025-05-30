// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_mobile_device_access_rule_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_mobile_device_access_rule::UpdateMobileDeviceAccessRuleInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.organization_id {
        object.key("OrganizationId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.mobile_device_access_rule_id {
        object.key("MobileDeviceAccessRuleId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.name {
        object.key("Name").string(var_3.as_str());
    }
    if let Some(var_4) = &input.description {
        object.key("Description").string(var_4.as_str());
    }
    if let Some(var_5) = &input.effect {
        object.key("Effect").string(var_5.as_str());
    }
    if let Some(var_6) = &input.device_types {
        let mut array_7 = object.key("DeviceTypes").start_array();
        for item_8 in var_6 {
            {
                array_7.value().string(item_8.as_str());
            }
        }
        array_7.finish();
    }
    if let Some(var_9) = &input.not_device_types {
        let mut array_10 = object.key("NotDeviceTypes").start_array();
        for item_11 in var_9 {
            {
                array_10.value().string(item_11.as_str());
            }
        }
        array_10.finish();
    }
    if let Some(var_12) = &input.device_models {
        let mut array_13 = object.key("DeviceModels").start_array();
        for item_14 in var_12 {
            {
                array_13.value().string(item_14.as_str());
            }
        }
        array_13.finish();
    }
    if let Some(var_15) = &input.not_device_models {
        let mut array_16 = object.key("NotDeviceModels").start_array();
        for item_17 in var_15 {
            {
                array_16.value().string(item_17.as_str());
            }
        }
        array_16.finish();
    }
    if let Some(var_18) = &input.device_operating_systems {
        let mut array_19 = object.key("DeviceOperatingSystems").start_array();
        for item_20 in var_18 {
            {
                array_19.value().string(item_20.as_str());
            }
        }
        array_19.finish();
    }
    if let Some(var_21) = &input.not_device_operating_systems {
        let mut array_22 = object.key("NotDeviceOperatingSystems").start_array();
        for item_23 in var_21 {
            {
                array_22.value().string(item_23.as_str());
            }
        }
        array_22.finish();
    }
    if let Some(var_24) = &input.device_user_agents {
        let mut array_25 = object.key("DeviceUserAgents").start_array();
        for item_26 in var_24 {
            {
                array_25.value().string(item_26.as_str());
            }
        }
        array_25.finish();
    }
    if let Some(var_27) = &input.not_device_user_agents {
        let mut array_28 = object.key("NotDeviceUserAgents").start_array();
        for item_29 in var_27 {
            {
                array_28.value().string(item_29.as_str());
            }
        }
        array_28.finish();
    }
    Ok(())
}
