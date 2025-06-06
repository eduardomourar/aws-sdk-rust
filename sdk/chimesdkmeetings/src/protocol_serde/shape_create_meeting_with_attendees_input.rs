// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_meeting_with_attendees_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_meeting_with_attendees::CreateMeetingWithAttendeesInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.attendees {
        let mut array_2 = object.key("Attendees").start_array();
        for item_3 in var_1 {
            {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_create_attendee_request_item::ser_create_attendee_request_item(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_5.as_str());
    }
    if let Some(var_6) = &input.external_meeting_id {
        object.key("ExternalMeetingId").string(var_6.as_str());
    }
    if let Some(var_7) = &input.media_region {
        object.key("MediaRegion").string(var_7.as_str());
    }
    if let Some(var_8) = &input.meeting_features {
        #[allow(unused_mut)]
        let mut object_9 = object.key("MeetingFeatures").start_object();
        crate::protocol_serde::shape_meeting_features_configuration::ser_meeting_features_configuration(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.meeting_host_id {
        object.key("MeetingHostId").string(var_10.as_str());
    }
    if let Some(var_11) = &input.notifications_configuration {
        #[allow(unused_mut)]
        let mut object_12 = object.key("NotificationsConfiguration").start_object();
        crate::protocol_serde::shape_notifications_configuration::ser_notifications_configuration(&mut object_12, var_11)?;
        object_12.finish();
    }
    if let Some(var_13) = &input.primary_meeting_id {
        object.key("PrimaryMeetingId").string(var_13.as_str());
    }
    if let Some(var_14) = &input.tags {
        let mut array_15 = object.key("Tags").start_array();
        for item_16 in var_14 {
            {
                #[allow(unused_mut)]
                let mut object_17 = array_15.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_17, item_16)?;
                object_17.finish();
            }
        }
        array_15.finish();
    }
    if let Some(var_18) = &input.tenant_ids {
        let mut array_19 = object.key("TenantIds").start_array();
        for item_20 in var_18 {
            {
                array_19.value().string(item_20.as_str());
            }
        }
        array_19.finish();
    }
    Ok(())
}
