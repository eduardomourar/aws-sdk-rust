// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_cache_cluster_input_input_input(
    input: &crate::operation::create_cache_cluster::CreateCacheClusterInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "CreateCacheCluster", "2015-02-02");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("CacheClusterId");
    if let Some(var_2) = &input.cache_cluster_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("ReplicationGroupId");
    if let Some(var_4) = &input.replication_group_id {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("AZMode");
    if let Some(var_6) = &input.az_mode {
        scope_5.string(var_6.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("PreferredAvailabilityZone");
    if let Some(var_8) = &input.preferred_availability_zone {
        scope_7.string(var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("PreferredAvailabilityZones");
    if let Some(var_10) = &input.preferred_availability_zones {
        let mut list_12 = scope_9.start_list(false, Some("PreferredAvailabilityZone"));
        for item_11 in var_10 {
            #[allow(unused_mut)]
            let mut entry_13 = list_12.entry();
            entry_13.string(item_11);
        }
        list_12.finish();
    }
    #[allow(unused_mut)]
    let mut scope_14 = writer.prefix("NumCacheNodes");
    if let Some(var_15) = &input.num_cache_nodes {
        scope_14.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_15).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_16 = writer.prefix("CacheNodeType");
    if let Some(var_17) = &input.cache_node_type {
        scope_16.string(var_17);
    }
    #[allow(unused_mut)]
    let mut scope_18 = writer.prefix("Engine");
    if let Some(var_19) = &input.engine {
        scope_18.string(var_19);
    }
    #[allow(unused_mut)]
    let mut scope_20 = writer.prefix("EngineVersion");
    if let Some(var_21) = &input.engine_version {
        scope_20.string(var_21);
    }
    #[allow(unused_mut)]
    let mut scope_22 = writer.prefix("CacheParameterGroupName");
    if let Some(var_23) = &input.cache_parameter_group_name {
        scope_22.string(var_23);
    }
    #[allow(unused_mut)]
    let mut scope_24 = writer.prefix("CacheSubnetGroupName");
    if let Some(var_25) = &input.cache_subnet_group_name {
        scope_24.string(var_25);
    }
    #[allow(unused_mut)]
    let mut scope_26 = writer.prefix("CacheSecurityGroupNames");
    if let Some(var_27) = &input.cache_security_group_names {
        let mut list_29 = scope_26.start_list(false, Some("CacheSecurityGroupName"));
        for item_28 in var_27 {
            #[allow(unused_mut)]
            let mut entry_30 = list_29.entry();
            entry_30.string(item_28);
        }
        list_29.finish();
    }
    #[allow(unused_mut)]
    let mut scope_31 = writer.prefix("SecurityGroupIds");
    if let Some(var_32) = &input.security_group_ids {
        let mut list_34 = scope_31.start_list(false, Some("SecurityGroupId"));
        for item_33 in var_32 {
            #[allow(unused_mut)]
            let mut entry_35 = list_34.entry();
            entry_35.string(item_33);
        }
        list_34.finish();
    }
    #[allow(unused_mut)]
    let mut scope_36 = writer.prefix("Tags");
    if let Some(var_37) = &input.tags {
        let mut list_39 = scope_36.start_list(false, Some("Tag"));
        for item_38 in var_37 {
            #[allow(unused_mut)]
            let mut entry_40 = list_39.entry();
            crate::protocol_serde::shape_tag::ser_tag(entry_40, item_38)?;
        }
        list_39.finish();
    }
    #[allow(unused_mut)]
    let mut scope_41 = writer.prefix("SnapshotArns");
    if let Some(var_42) = &input.snapshot_arns {
        let mut list_44 = scope_41.start_list(false, Some("SnapshotArn"));
        for item_43 in var_42 {
            #[allow(unused_mut)]
            let mut entry_45 = list_44.entry();
            entry_45.string(item_43);
        }
        list_44.finish();
    }
    #[allow(unused_mut)]
    let mut scope_46 = writer.prefix("SnapshotName");
    if let Some(var_47) = &input.snapshot_name {
        scope_46.string(var_47);
    }
    #[allow(unused_mut)]
    let mut scope_48 = writer.prefix("PreferredMaintenanceWindow");
    if let Some(var_49) = &input.preferred_maintenance_window {
        scope_48.string(var_49);
    }
    #[allow(unused_mut)]
    let mut scope_50 = writer.prefix("Port");
    if let Some(var_51) = &input.port {
        scope_50.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_51).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_52 = writer.prefix("NotificationTopicArn");
    if let Some(var_53) = &input.notification_topic_arn {
        scope_52.string(var_53);
    }
    #[allow(unused_mut)]
    let mut scope_54 = writer.prefix("AutoMinorVersionUpgrade");
    if let Some(var_55) = &input.auto_minor_version_upgrade {
        scope_54.boolean(*var_55);
    }
    #[allow(unused_mut)]
    let mut scope_56 = writer.prefix("SnapshotRetentionLimit");
    if let Some(var_57) = &input.snapshot_retention_limit {
        scope_56.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_57).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_58 = writer.prefix("SnapshotWindow");
    if let Some(var_59) = &input.snapshot_window {
        scope_58.string(var_59);
    }
    #[allow(unused_mut)]
    let mut scope_60 = writer.prefix("AuthToken");
    if let Some(var_61) = &input.auth_token {
        scope_60.string(var_61);
    }
    #[allow(unused_mut)]
    let mut scope_62 = writer.prefix("OutpostMode");
    if let Some(var_63) = &input.outpost_mode {
        scope_62.string(var_63.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_64 = writer.prefix("PreferredOutpostArn");
    if let Some(var_65) = &input.preferred_outpost_arn {
        scope_64.string(var_65);
    }
    #[allow(unused_mut)]
    let mut scope_66 = writer.prefix("PreferredOutpostArns");
    if let Some(var_67) = &input.preferred_outpost_arns {
        let mut list_69 = scope_66.start_list(false, Some("PreferredOutpostArn"));
        for item_68 in var_67 {
            #[allow(unused_mut)]
            let mut entry_70 = list_69.entry();
            entry_70.string(item_68);
        }
        list_69.finish();
    }
    #[allow(unused_mut)]
    let mut scope_71 = writer.prefix("LogDeliveryConfigurations");
    if let Some(var_72) = &input.log_delivery_configurations {
        let mut list_74 = scope_71.start_list(false, Some("LogDeliveryConfigurationRequest"));
        for item_73 in var_72 {
            #[allow(unused_mut)]
            let mut entry_75 = list_74.entry();
            crate::protocol_serde::shape_log_delivery_configuration_request::ser_log_delivery_configuration_request(entry_75, item_73)?;
        }
        list_74.finish();
    }
    #[allow(unused_mut)]
    let mut scope_76 = writer.prefix("TransitEncryptionEnabled");
    if let Some(var_77) = &input.transit_encryption_enabled {
        scope_76.boolean(*var_77);
    }
    #[allow(unused_mut)]
    let mut scope_78 = writer.prefix("NetworkType");
    if let Some(var_79) = &input.network_type {
        scope_78.string(var_79.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_80 = writer.prefix("IpDiscovery");
    if let Some(var_81) = &input.ip_discovery {
        scope_80.string(var_81.as_str());
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
