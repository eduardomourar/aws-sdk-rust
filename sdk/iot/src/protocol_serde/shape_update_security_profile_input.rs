// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_security_profile_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_security_profile::UpdateSecurityProfileInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.additional_metrics_to_retain {
        let mut array_2 = object.key("additionalMetricsToRetain").start_array();
        for item_3 in var_1 {
            {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.additional_metrics_to_retain_v2 {
        let mut array_5 = object.key("additionalMetricsToRetainV2").start_array();
        for item_6 in var_4 {
            {
                #[allow(unused_mut)]
                let mut object_7 = array_5.value().start_object();
                crate::protocol_serde::shape_metric_to_retain::ser_metric_to_retain(&mut object_7, item_6)?;
                object_7.finish();
            }
        }
        array_5.finish();
    }
    if let Some(var_8) = &input.alert_targets {
        #[allow(unused_mut)]
        let mut object_9 = object.key("alertTargets").start_object();
        for (key_10, value_11) in var_8 {
            {
                #[allow(unused_mut)]
                let mut object_12 = object_9.key(key_10.as_str()).start_object();
                crate::protocol_serde::shape_alert_target::ser_alert_target(&mut object_12, value_11)?;
                object_12.finish();
            }
        }
        object_9.finish();
    }
    if let Some(var_13) = &input.behaviors {
        let mut array_14 = object.key("behaviors").start_array();
        for item_15 in var_13 {
            {
                #[allow(unused_mut)]
                let mut object_16 = array_14.value().start_object();
                crate::protocol_serde::shape_behavior::ser_behavior(&mut object_16, item_15)?;
                object_16.finish();
            }
        }
        array_14.finish();
    }
    if let Some(var_17) = &input.delete_additional_metrics_to_retain {
        object.key("deleteAdditionalMetricsToRetain").boolean(*var_17);
    }
    if let Some(var_18) = &input.delete_alert_targets {
        object.key("deleteAlertTargets").boolean(*var_18);
    }
    if let Some(var_19) = &input.delete_behaviors {
        object.key("deleteBehaviors").boolean(*var_19);
    }
    if let Some(var_20) = &input.delete_metrics_export_config {
        object.key("deleteMetricsExportConfig").boolean(*var_20);
    }
    if let Some(var_21) = &input.metrics_export_config {
        #[allow(unused_mut)]
        let mut object_22 = object.key("metricsExportConfig").start_object();
        crate::protocol_serde::shape_metrics_export_config::ser_metrics_export_config(&mut object_22, var_21)?;
        object_22.finish();
    }
    if let Some(var_23) = &input.security_profile_description {
        object.key("securityProfileDescription").string(var_23.as_str());
    }
    Ok(())
}
