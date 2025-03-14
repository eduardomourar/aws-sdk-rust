// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_participant_token_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_participant_token::CreateParticipantTokenInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.attributes {
        #[allow(unused_mut)]
        let mut object_2 = object.key("attributes").start_object();
        for (key_3, value_4) in var_1 {
            {
                object_2.key(key_3.as_str()).string(value_4.as_str());
            }
        }
        object_2.finish();
    }
    if let Some(var_5) = &input.capabilities {
        let mut array_6 = object.key("capabilities").start_array();
        for item_7 in var_5 {
            {
                array_6.value().string(item_7.as_str());
            }
        }
        array_6.finish();
    }
    if let Some(var_8) = &input.duration {
        object.key("duration").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_8).into()),
        );
    }
    if let Some(var_9) = &input.stage_arn {
        object.key("stageArn").string(var_9.as_str());
    }
    if let Some(var_10) = &input.user_id {
        object.key("userId").string(var_10.as_str());
    }
    Ok(())
}
