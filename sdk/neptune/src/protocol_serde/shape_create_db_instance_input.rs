// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_db_instance_input_input_input(
    input: &crate::operation::create_db_instance::CreateDbInstanceInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "CreateDBInstance", "2014-10-31");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DBName");
    if let Some(var_2) = &input.db_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("DBInstanceIdentifier");
    if let Some(var_4) = &input.db_instance_identifier {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("AllocatedStorage");
    if let Some(var_6) = &input.allocated_storage {
        scope_5.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_6).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("DBInstanceClass");
    if let Some(var_8) = &input.db_instance_class {
        scope_7.string(var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("Engine");
    if let Some(var_10) = &input.engine {
        scope_9.string(var_10);
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("MasterUsername");
    if let Some(var_12) = &input.master_username {
        scope_11.string(var_12);
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("MasterUserPassword");
    if let Some(var_14) = &input.master_user_password {
        scope_13.string(var_14);
    }
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("DBSecurityGroups");
    if let Some(var_16) = &input.db_security_groups {
        let mut list_18 = scope_15.start_list(false, Some("DBSecurityGroupName"));
        for item_17 in var_16 {
            #[allow(unused_mut)]
            let mut entry_19 = list_18.entry();
            entry_19.string(item_17);
        }
        list_18.finish();
    }
    #[allow(unused_mut)]
    let mut scope_20 = writer.prefix("VpcSecurityGroupIds");
    if let Some(var_21) = &input.vpc_security_group_ids {
        let mut list_23 = scope_20.start_list(false, Some("VpcSecurityGroupId"));
        for item_22 in var_21 {
            #[allow(unused_mut)]
            let mut entry_24 = list_23.entry();
            entry_24.string(item_22);
        }
        list_23.finish();
    }
    #[allow(unused_mut)]
    let mut scope_25 = writer.prefix("AvailabilityZone");
    if let Some(var_26) = &input.availability_zone {
        scope_25.string(var_26);
    }
    #[allow(unused_mut)]
    let mut scope_27 = writer.prefix("DBSubnetGroupName");
    if let Some(var_28) = &input.db_subnet_group_name {
        scope_27.string(var_28);
    }
    #[allow(unused_mut)]
    let mut scope_29 = writer.prefix("PreferredMaintenanceWindow");
    if let Some(var_30) = &input.preferred_maintenance_window {
        scope_29.string(var_30);
    }
    #[allow(unused_mut)]
    let mut scope_31 = writer.prefix("DBParameterGroupName");
    if let Some(var_32) = &input.db_parameter_group_name {
        scope_31.string(var_32);
    }
    #[allow(unused_mut)]
    let mut scope_33 = writer.prefix("BackupRetentionPeriod");
    if let Some(var_34) = &input.backup_retention_period {
        scope_33.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_34).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_35 = writer.prefix("PreferredBackupWindow");
    if let Some(var_36) = &input.preferred_backup_window {
        scope_35.string(var_36);
    }
    #[allow(unused_mut)]
    let mut scope_37 = writer.prefix("Port");
    if let Some(var_38) = &input.port {
        scope_37.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_38).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_39 = writer.prefix("MultiAZ");
    if let Some(var_40) = &input.multi_az {
        scope_39.boolean(*var_40);
    }
    #[allow(unused_mut)]
    let mut scope_41 = writer.prefix("EngineVersion");
    if let Some(var_42) = &input.engine_version {
        scope_41.string(var_42);
    }
    #[allow(unused_mut)]
    let mut scope_43 = writer.prefix("AutoMinorVersionUpgrade");
    if let Some(var_44) = &input.auto_minor_version_upgrade {
        scope_43.boolean(*var_44);
    }
    #[allow(unused_mut)]
    let mut scope_45 = writer.prefix("LicenseModel");
    if let Some(var_46) = &input.license_model {
        scope_45.string(var_46);
    }
    #[allow(unused_mut)]
    let mut scope_47 = writer.prefix("Iops");
    if let Some(var_48) = &input.iops {
        scope_47.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_48).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_49 = writer.prefix("OptionGroupName");
    if let Some(var_50) = &input.option_group_name {
        scope_49.string(var_50);
    }
    #[allow(unused_mut)]
    let mut scope_51 = writer.prefix("CharacterSetName");
    if let Some(var_52) = &input.character_set_name {
        scope_51.string(var_52);
    }
    #[allow(unused_mut)]
    let mut scope_53 = writer.prefix("PubliclyAccessible");
    if let Some(var_54) = &input.publicly_accessible {
        scope_53.boolean(*var_54);
    }
    #[allow(unused_mut)]
    let mut scope_55 = writer.prefix("Tags");
    if let Some(var_56) = &input.tags {
        let mut list_58 = scope_55.start_list(false, Some("Tag"));
        for item_57 in var_56 {
            #[allow(unused_mut)]
            let mut entry_59 = list_58.entry();
            crate::protocol_serde::shape_tag::ser_tag(entry_59, item_57)?;
        }
        list_58.finish();
    }
    #[allow(unused_mut)]
    let mut scope_60 = writer.prefix("DBClusterIdentifier");
    if let Some(var_61) = &input.db_cluster_identifier {
        scope_60.string(var_61);
    }
    #[allow(unused_mut)]
    let mut scope_62 = writer.prefix("StorageType");
    if let Some(var_63) = &input.storage_type {
        scope_62.string(var_63);
    }
    #[allow(unused_mut)]
    let mut scope_64 = writer.prefix("TdeCredentialArn");
    if let Some(var_65) = &input.tde_credential_arn {
        scope_64.string(var_65);
    }
    #[allow(unused_mut)]
    let mut scope_66 = writer.prefix("TdeCredentialPassword");
    if let Some(var_67) = &input.tde_credential_password {
        scope_66.string(var_67);
    }
    #[allow(unused_mut)]
    let mut scope_68 = writer.prefix("StorageEncrypted");
    if let Some(var_69) = &input.storage_encrypted {
        scope_68.boolean(*var_69);
    }
    #[allow(unused_mut)]
    let mut scope_70 = writer.prefix("KmsKeyId");
    if let Some(var_71) = &input.kms_key_id {
        scope_70.string(var_71);
    }
    #[allow(unused_mut)]
    let mut scope_72 = writer.prefix("Domain");
    if let Some(var_73) = &input.domain {
        scope_72.string(var_73);
    }
    #[allow(unused_mut)]
    let mut scope_74 = writer.prefix("CopyTagsToSnapshot");
    if let Some(var_75) = &input.copy_tags_to_snapshot {
        scope_74.boolean(*var_75);
    }
    #[allow(unused_mut)]
    let mut scope_76 = writer.prefix("MonitoringInterval");
    if let Some(var_77) = &input.monitoring_interval {
        scope_76.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_77).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_78 = writer.prefix("MonitoringRoleArn");
    if let Some(var_79) = &input.monitoring_role_arn {
        scope_78.string(var_79);
    }
    #[allow(unused_mut)]
    let mut scope_80 = writer.prefix("DomainIAMRoleName");
    if let Some(var_81) = &input.domain_iam_role_name {
        scope_80.string(var_81);
    }
    #[allow(unused_mut)]
    let mut scope_82 = writer.prefix("PromotionTier");
    if let Some(var_83) = &input.promotion_tier {
        scope_82.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_83).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_84 = writer.prefix("Timezone");
    if let Some(var_85) = &input.timezone {
        scope_84.string(var_85);
    }
    #[allow(unused_mut)]
    let mut scope_86 = writer.prefix("EnableIAMDatabaseAuthentication");
    if let Some(var_87) = &input.enable_iam_database_authentication {
        scope_86.boolean(*var_87);
    }
    #[allow(unused_mut)]
    let mut scope_88 = writer.prefix("EnablePerformanceInsights");
    if let Some(var_89) = &input.enable_performance_insights {
        scope_88.boolean(*var_89);
    }
    #[allow(unused_mut)]
    let mut scope_90 = writer.prefix("PerformanceInsightsKMSKeyId");
    if let Some(var_91) = &input.performance_insights_kms_key_id {
        scope_90.string(var_91);
    }
    #[allow(unused_mut)]
    let mut scope_92 = writer.prefix("EnableCloudwatchLogsExports");
    if let Some(var_93) = &input.enable_cloudwatch_logs_exports {
        let mut list_95 = scope_92.start_list(false, None);
        for item_94 in var_93 {
            #[allow(unused_mut)]
            let mut entry_96 = list_95.entry();
            entry_96.string(item_94);
        }
        list_95.finish();
    }
    #[allow(unused_mut)]
    let mut scope_97 = writer.prefix("DeletionProtection");
    if let Some(var_98) = &input.deletion_protection {
        scope_97.boolean(*var_98);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
