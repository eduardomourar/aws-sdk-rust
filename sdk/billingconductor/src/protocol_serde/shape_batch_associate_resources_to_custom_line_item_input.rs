// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_batch_associate_resources_to_custom_line_item_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::batch_associate_resources_to_custom_line_item::BatchAssociateResourcesToCustomLineItemInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.billing_period_range {
        #[allow(unused_mut)]
        let mut object_2 = object.key("BillingPeriodRange").start_object();
        crate::protocol_serde::shape_custom_line_item_billing_period_range::ser_custom_line_item_billing_period_range(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.resource_arns {
        let mut array_4 = object.key("ResourceArns").start_array();
        for item_5 in var_3 {
            {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    if let Some(var_6) = &input.target_arn {
        object.key("TargetArn").string(var_6.as_str());
    }
    Ok(())
}
