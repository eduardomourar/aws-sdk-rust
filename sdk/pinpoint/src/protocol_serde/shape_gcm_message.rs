// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_gcm_message(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::GcmMessage,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.action {
        object.key("Action").string(var_1.as_str());
    }
    if let Some(var_2) = &input.body {
        object.key("Body").string(var_2.as_str());
    }
    if let Some(var_3) = &input.collapse_key {
        object.key("CollapseKey").string(var_3.as_str());
    }
    if let Some(var_4) = &input.data {
        #[allow(unused_mut)]
        let mut object_5 = object.key("Data").start_object();
        for (key_6, value_7) in var_4 {
            {
                object_5.key(key_6.as_str()).string(value_7.as_str());
            }
        }
        object_5.finish();
    }
    if let Some(var_8) = &input.icon_reference {
        object.key("IconReference").string(var_8.as_str());
    }
    if let Some(var_9) = &input.image_icon_url {
        object.key("ImageIconUrl").string(var_9.as_str());
    }
    if let Some(var_10) = &input.image_url {
        object.key("ImageUrl").string(var_10.as_str());
    }
    if let Some(var_11) = &input.preferred_authentication_method {
        object.key("PreferredAuthenticationMethod").string(var_11.as_str());
    }
    if let Some(var_12) = &input.priority {
        object.key("Priority").string(var_12.as_str());
    }
    if let Some(var_13) = &input.raw_content {
        object.key("RawContent").string(var_13.as_str());
    }
    if let Some(var_14) = &input.restricted_package_name {
        object.key("RestrictedPackageName").string(var_14.as_str());
    }
    if let Some(var_15) = &input.silent_push {
        object.key("SilentPush").boolean(*var_15);
    }
    if let Some(var_16) = &input.small_image_icon_url {
        object.key("SmallImageIconUrl").string(var_16.as_str());
    }
    if let Some(var_17) = &input.sound {
        object.key("Sound").string(var_17.as_str());
    }
    if let Some(var_18) = &input.substitutions {
        #[allow(unused_mut)]
        let mut object_19 = object.key("Substitutions").start_object();
        for (key_20, value_21) in var_18 {
            {
                let mut array_22 = object_19.key(key_20.as_str()).start_array();
                for item_23 in value_21 {
                    {
                        array_22.value().string(item_23.as_str());
                    }
                }
                array_22.finish();
            }
        }
        object_19.finish();
    }
    if let Some(var_24) = &input.time_to_live {
        object.key("TimeToLive").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_24).into()),
        );
    }
    if let Some(var_25) = &input.title {
        object.key("Title").string(var_25.as_str());
    }
    if let Some(var_26) = &input.url {
        object.key("Url").string(var_26.as_str());
    }
    Ok(())
}
