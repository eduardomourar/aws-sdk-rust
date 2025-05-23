// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_bot_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_bot::UpdateBotInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.bot_members {
        let mut array_2 = object.key("botMembers").start_array();
        for item_3 in var_1 {
            {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_bot_member::ser_bot_member(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.bot_name {
        object.key("botName").string(var_5.as_str());
    }
    if let Some(var_6) = &input.bot_type {
        object.key("botType").string(var_6.as_str());
    }
    if let Some(var_7) = &input.data_privacy {
        #[allow(unused_mut)]
        let mut object_8 = object.key("dataPrivacy").start_object();
        crate::protocol_serde::shape_data_privacy::ser_data_privacy(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.description {
        object.key("description").string(var_9.as_str());
    }
    if let Some(var_10) = &input.error_log_settings {
        #[allow(unused_mut)]
        let mut object_11 = object.key("errorLogSettings").start_object();
        crate::protocol_serde::shape_error_log_settings::ser_error_log_settings(&mut object_11, var_10)?;
        object_11.finish();
    }
    if let Some(var_12) = &input.idle_session_ttl_in_seconds {
        object.key("idleSessionTTLInSeconds").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_12).into()),
        );
    }
    if let Some(var_13) = &input.role_arn {
        object.key("roleArn").string(var_13.as_str());
    }
    Ok(())
}
