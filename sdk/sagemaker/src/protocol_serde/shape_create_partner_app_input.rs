// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_partner_app_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_partner_app::CreatePartnerAppInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("Name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.r#type {
        object.key("Type").string(var_2.as_str());
    }
    if let Some(var_3) = &input.execution_role_arn {
        object.key("ExecutionRoleArn").string(var_3.as_str());
    }
    if let Some(var_4) = &input.kms_key_id {
        object.key("KmsKeyId").string(var_4.as_str());
    }
    if let Some(var_5) = &input.maintenance_config {
        #[allow(unused_mut)]
        let mut object_6 = object.key("MaintenanceConfig").start_object();
        crate::protocol_serde::shape_partner_app_maintenance_config::ser_partner_app_maintenance_config(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.tier {
        object.key("Tier").string(var_7.as_str());
    }
    if let Some(var_8) = &input.application_config {
        #[allow(unused_mut)]
        let mut object_9 = object.key("ApplicationConfig").start_object();
        crate::protocol_serde::shape_partner_app_config::ser_partner_app_config(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.auth_type {
        object.key("AuthType").string(var_10.as_str());
    }
    if let Some(var_11) = &input.enable_iam_session_based_identity {
        object.key("EnableIamSessionBasedIdentity").boolean(*var_11);
    }
    if let Some(var_12) = &input.client_token {
        object.key("ClientToken").string(var_12.as_str());
    }
    if let Some(var_13) = &input.tags {
        let mut array_14 = object.key("Tags").start_array();
        for item_15 in var_13 {
            {
                #[allow(unused_mut)]
                let mut object_16 = array_14.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_16, item_15)?;
                object_16.finish();
            }
        }
        array_14.finish();
    }
    Ok(())
}
