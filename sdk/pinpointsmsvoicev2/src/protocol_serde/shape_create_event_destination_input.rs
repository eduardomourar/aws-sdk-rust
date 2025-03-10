// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_event_destination_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_event_destination::CreateEventDestinationInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.configuration_set_name {
        object.key("ConfigurationSetName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.event_destination_name {
        object.key("EventDestinationName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.matching_event_types {
        let mut array_4 = object.key("MatchingEventTypes").start_array();
        for item_5 in var_3 {
            {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    if let Some(var_6) = &input.cloud_watch_logs_destination {
        #[allow(unused_mut)]
        let mut object_7 = object.key("CloudWatchLogsDestination").start_object();
        crate::protocol_serde::shape_cloud_watch_logs_destination::ser_cloud_watch_logs_destination(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.kinesis_firehose_destination {
        #[allow(unused_mut)]
        let mut object_9 = object.key("KinesisFirehoseDestination").start_object();
        crate::protocol_serde::shape_kinesis_firehose_destination::ser_kinesis_firehose_destination(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.sns_destination {
        #[allow(unused_mut)]
        let mut object_11 = object.key("SnsDestination").start_object();
        crate::protocol_serde::shape_sns_destination::ser_sns_destination(&mut object_11, var_10)?;
        object_11.finish();
    }
    if let Some(var_12) = &input.client_token {
        object.key("ClientToken").string(var_12.as_str());
    }
    Ok(())
}
