// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_batch_is_authorized_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::batch_is_authorized::BatchIsAuthorizedInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.policy_store_id {
        object.key("policyStoreId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.entities {
        #[allow(unused_mut)]
        let mut object_3 = object.key("entities").start_object();
        crate::protocol_serde::shape_entities_definition::ser_entities_definition(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.requests {
        let mut array_5 = object.key("requests").start_array();
        for item_6 in var_4 {
            {
                #[allow(unused_mut)]
                let mut object_7 = array_5.value().start_object();
                crate::protocol_serde::shape_batch_is_authorized_input_item::ser_batch_is_authorized_input_item(&mut object_7, item_6)?;
                object_7.finish();
            }
        }
        array_5.finish();
    }
    Ok(())
}
