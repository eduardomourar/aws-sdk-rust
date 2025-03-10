// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_cis_targets(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::UpdateCisTargets,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.account_ids {
        let mut array_2 = object.key("accountIds").start_array();
        for item_3 in var_1 {
            {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.target_resource_tags {
        #[allow(unused_mut)]
        let mut object_5 = object.key("targetResourceTags").start_object();
        for (key_6, value_7) in var_4 {
            {
                let mut array_8 = object_5.key(key_6.as_str()).start_array();
                for item_9 in value_7 {
                    {
                        array_8.value().string(item_9.as_str());
                    }
                }
                array_8.finish();
            }
        }
        object_5.finish();
    }
    Ok(())
}
