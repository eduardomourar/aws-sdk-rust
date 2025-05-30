// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_broker_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_broker::CreateBrokerInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.authentication_strategy {
        object.key("authenticationStrategy").string(var_1.as_str());
    }
    if let Some(var_2) = &input.auto_minor_version_upgrade {
        object.key("autoMinorVersionUpgrade").boolean(*var_2);
    }
    if let Some(var_3) = &input.broker_name {
        object.key("brokerName").string(var_3.as_str());
    }
    if let Some(var_4) = &input.configuration {
        #[allow(unused_mut)]
        let mut object_5 = object.key("configuration").start_object();
        crate::protocol_serde::shape_configuration_id::ser_configuration_id(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.creator_request_id {
        object.key("creatorRequestId").string(var_6.as_str());
    }
    if let Some(var_7) = &input.data_replication_mode {
        object.key("dataReplicationMode").string(var_7.as_str());
    }
    if let Some(var_8) = &input.data_replication_primary_broker_arn {
        object.key("dataReplicationPrimaryBrokerArn").string(var_8.as_str());
    }
    if let Some(var_9) = &input.deployment_mode {
        object.key("deploymentMode").string(var_9.as_str());
    }
    if let Some(var_10) = &input.encryption_options {
        #[allow(unused_mut)]
        let mut object_11 = object.key("encryptionOptions").start_object();
        crate::protocol_serde::shape_encryption_options::ser_encryption_options(&mut object_11, var_10)?;
        object_11.finish();
    }
    if let Some(var_12) = &input.engine_type {
        object.key("engineType").string(var_12.as_str());
    }
    if let Some(var_13) = &input.engine_version {
        object.key("engineVersion").string(var_13.as_str());
    }
    if let Some(var_14) = &input.host_instance_type {
        object.key("hostInstanceType").string(var_14.as_str());
    }
    if let Some(var_15) = &input.ldap_server_metadata {
        #[allow(unused_mut)]
        let mut object_16 = object.key("ldapServerMetadata").start_object();
        crate::protocol_serde::shape_ldap_server_metadata_input::ser_ldap_server_metadata_input(&mut object_16, var_15)?;
        object_16.finish();
    }
    if let Some(var_17) = &input.logs {
        #[allow(unused_mut)]
        let mut object_18 = object.key("logs").start_object();
        crate::protocol_serde::shape_logs::ser_logs(&mut object_18, var_17)?;
        object_18.finish();
    }
    if let Some(var_19) = &input.maintenance_window_start_time {
        #[allow(unused_mut)]
        let mut object_20 = object.key("maintenanceWindowStartTime").start_object();
        crate::protocol_serde::shape_weekly_start_time::ser_weekly_start_time(&mut object_20, var_19)?;
        object_20.finish();
    }
    if let Some(var_21) = &input.publicly_accessible {
        object.key("publiclyAccessible").boolean(*var_21);
    }
    if let Some(var_22) = &input.security_groups {
        let mut array_23 = object.key("securityGroups").start_array();
        for item_24 in var_22 {
            {
                array_23.value().string(item_24.as_str());
            }
        }
        array_23.finish();
    }
    if let Some(var_25) = &input.storage_type {
        object.key("storageType").string(var_25.as_str());
    }
    if let Some(var_26) = &input.subnet_ids {
        let mut array_27 = object.key("subnetIds").start_array();
        for item_28 in var_26 {
            {
                array_27.value().string(item_28.as_str());
            }
        }
        array_27.finish();
    }
    if let Some(var_29) = &input.tags {
        #[allow(unused_mut)]
        let mut object_30 = object.key("tags").start_object();
        for (key_31, value_32) in var_29 {
            {
                object_30.key(key_31.as_str()).string(value_32.as_str());
            }
        }
        object_30.finish();
    }
    if let Some(var_33) = &input.users {
        let mut array_34 = object.key("users").start_array();
        for item_35 in var_33 {
            {
                #[allow(unused_mut)]
                let mut object_36 = array_34.value().start_object();
                crate::protocol_serde::shape_user::ser_user(&mut object_36, item_35)?;
                object_36.finish();
            }
        }
        array_34.finish();
    }
    Ok(())
}
