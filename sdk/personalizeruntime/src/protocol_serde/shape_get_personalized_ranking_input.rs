// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_personalized_ranking_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::get_personalized_ranking::GetPersonalizedRankingInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.campaign_arn {
        object.key("campaignArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.context {
        #[allow(unused_mut)]
        let mut object_3 = object.key("context").start_object();
        for (key_4, value_5) in var_2 {
            {
                object_3.key(key_4.as_str()).string(value_5.as_str());
            }
        }
        object_3.finish();
    }
    if let Some(var_6) = &input.filter_arn {
        object.key("filterArn").string(var_6.as_str());
    }
    if let Some(var_7) = &input.filter_values {
        #[allow(unused_mut)]
        let mut object_8 = object.key("filterValues").start_object();
        for (key_9, value_10) in var_7 {
            {
                object_8.key(key_9.as_str()).string(value_10.as_str());
            }
        }
        object_8.finish();
    }
    if let Some(var_11) = &input.input_list {
        let mut array_12 = object.key("inputList").start_array();
        for item_13 in var_11 {
            {
                array_12.value().string(item_13.as_str());
            }
        }
        array_12.finish();
    }
    if let Some(var_14) = &input.metadata_columns {
        #[allow(unused_mut)]
        let mut object_15 = object.key("metadataColumns").start_object();
        for (key_16, value_17) in var_14 {
            {
                let mut array_18 = object_15.key(key_16.as_str()).start_array();
                for item_19 in value_17 {
                    {
                        array_18.value().string(item_19.as_str());
                    }
                }
                array_18.finish();
            }
        }
        object_15.finish();
    }
    if let Some(var_20) = &input.user_id {
        object.key("userId").string(var_20.as_str());
    }
    Ok(())
}
