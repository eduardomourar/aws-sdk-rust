// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_telemetry_record(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::TelemetryRecord,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object
            .key("Timestamp")
            .date_time(&input.timestamp, ::aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_1) = &input.segments_received_count {
        object.key("SegmentsReceivedCount").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_1).into()),
        );
    }
    if let Some(var_2) = &input.segments_sent_count {
        object.key("SegmentsSentCount").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    if let Some(var_3) = &input.segments_spillover_count {
        object.key("SegmentsSpilloverCount").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_3).into()),
        );
    }
    if let Some(var_4) = &input.segments_rejected_count {
        object.key("SegmentsRejectedCount").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    if let Some(var_5) = &input.backend_connection_errors {
        #[allow(unused_mut)]
        let mut object_6 = object.key("BackendConnectionErrors").start_object();
        crate::protocol_serde::shape_backend_connection_errors::ser_backend_connection_errors(&mut object_6, var_5)?;
        object_6.finish();
    }
    Ok(())
}
