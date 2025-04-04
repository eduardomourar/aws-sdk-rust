// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_capacity_reservation_input_input_input(
    input: &crate::operation::create_capacity_reservation::CreateCapacityReservationInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "CreateCapacityReservation", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ClientToken");
    if let Some(var_2) = &input.client_token {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("InstanceType");
    if let Some(var_4) = &input.instance_type {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("InstancePlatform");
    if let Some(var_6) = &input.instance_platform {
        scope_5.string(var_6.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("AvailabilityZone");
    if let Some(var_8) = &input.availability_zone {
        scope_7.string(var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("AvailabilityZoneId");
    if let Some(var_10) = &input.availability_zone_id {
        scope_9.string(var_10);
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("Tenancy");
    if let Some(var_12) = &input.tenancy {
        scope_11.string(var_12.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("InstanceCount");
    if let Some(var_14) = &input.instance_count {
        scope_13.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_14).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("EbsOptimized");
    if let Some(var_16) = &input.ebs_optimized {
        scope_15.boolean(*var_16);
    }
    #[allow(unused_mut)]
    let mut scope_17 = writer.prefix("EphemeralStorage");
    if let Some(var_18) = &input.ephemeral_storage {
        scope_17.boolean(*var_18);
    }
    #[allow(unused_mut)]
    let mut scope_19 = writer.prefix("EndDate");
    if let Some(var_20) = &input.end_date {
        scope_19.date_time(var_20, ::aws_smithy_types::date_time::Format::DateTime)?;
    }
    #[allow(unused_mut)]
    let mut scope_21 = writer.prefix("EndDateType");
    if let Some(var_22) = &input.end_date_type {
        scope_21.string(var_22.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_23 = writer.prefix("InstanceMatchCriteria");
    if let Some(var_24) = &input.instance_match_criteria {
        scope_23.string(var_24.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_25 = writer.prefix("TagSpecifications");
    if let Some(var_26) = &input.tag_specifications {
        if !var_26.is_empty() {
            let mut list_28 = scope_25.start_list(true, Some("item"));
            for item_27 in var_26 {
                #[allow(unused_mut)]
                let mut entry_29 = list_28.entry();
                crate::protocol_serde::shape_tag_specification::ser_tag_specification(entry_29, item_27)?;
            }
            list_28.finish();
        }
    }
    #[allow(unused_mut)]
    let mut scope_30 = writer.prefix("DryRun");
    if let Some(var_31) = &input.dry_run {
        scope_30.boolean(*var_31);
    }
    #[allow(unused_mut)]
    let mut scope_32 = writer.prefix("OutpostArn");
    if let Some(var_33) = &input.outpost_arn {
        scope_32.string(var_33);
    }
    #[allow(unused_mut)]
    let mut scope_34 = writer.prefix("PlacementGroupArn");
    if let Some(var_35) = &input.placement_group_arn {
        scope_34.string(var_35);
    }
    #[allow(unused_mut)]
    let mut scope_36 = writer.prefix("StartDate");
    if let Some(var_37) = &input.start_date {
        scope_36.date_time(var_37, ::aws_smithy_types::date_time::Format::DateTime)?;
    }
    #[allow(unused_mut)]
    let mut scope_38 = writer.prefix("CommitmentDuration");
    if let Some(var_39) = &input.commitment_duration {
        scope_38.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_39).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_40 = writer.prefix("DeliveryPreference");
    if let Some(var_41) = &input.delivery_preference {
        scope_40.string(var_41.as_str());
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
