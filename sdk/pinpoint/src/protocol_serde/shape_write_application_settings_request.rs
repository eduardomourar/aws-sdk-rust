// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_write_application_settings_request(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::WriteApplicationSettingsRequest,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.campaign_hook {
        #[allow(unused_mut)]
        let mut object_2 = object.key("CampaignHook").start_object();
        crate::protocol_serde::shape_campaign_hook::ser_campaign_hook(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.cloud_watch_metrics_enabled {
        object.key("CloudWatchMetricsEnabled").boolean(*var_3);
    }
    if let Some(var_4) = &input.event_tagging_enabled {
        object.key("EventTaggingEnabled").boolean(*var_4);
    }
    if let Some(var_5) = &input.limits {
        #[allow(unused_mut)]
        let mut object_6 = object.key("Limits").start_object();
        crate::protocol_serde::shape_campaign_limits::ser_campaign_limits(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.quiet_time {
        #[allow(unused_mut)]
        let mut object_8 = object.key("QuietTime").start_object();
        crate::protocol_serde::shape_quiet_time::ser_quiet_time(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.journey_limits {
        #[allow(unused_mut)]
        let mut object_10 = object.key("JourneyLimits").start_object();
        crate::protocol_serde::shape_application_settings_journey_limits::ser_application_settings_journey_limits(&mut object_10, var_9)?;
        object_10.finish();
    }
    Ok(())
}
