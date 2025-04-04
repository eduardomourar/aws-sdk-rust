// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_nfs_file_share_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_nfs_file_share::CreateNfsFileShareInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("ClientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.nfs_file_share_defaults {
        #[allow(unused_mut)]
        let mut object_3 = object.key("NFSFileShareDefaults").start_object();
        crate::protocol_serde::shape_nfs_file_share_defaults::ser_nfs_file_share_defaults(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.gateway_arn {
        object.key("GatewayARN").string(var_4.as_str());
    }
    if let Some(var_5) = &input.encryption_type {
        object.key("EncryptionType").string(var_5.as_str());
    }
    if let Some(var_6) = &input.kms_encrypted {
        object.key("KMSEncrypted").boolean(*var_6);
    }
    if let Some(var_7) = &input.kms_key {
        object.key("KMSKey").string(var_7.as_str());
    }
    if let Some(var_8) = &input.role {
        object.key("Role").string(var_8.as_str());
    }
    if let Some(var_9) = &input.location_arn {
        object.key("LocationARN").string(var_9.as_str());
    }
    if let Some(var_10) = &input.default_storage_class {
        object.key("DefaultStorageClass").string(var_10.as_str());
    }
    if let Some(var_11) = &input.object_acl {
        object.key("ObjectACL").string(var_11.as_str());
    }
    if let Some(var_12) = &input.client_list {
        let mut array_13 = object.key("ClientList").start_array();
        for item_14 in var_12 {
            {
                array_13.value().string(item_14.as_str());
            }
        }
        array_13.finish();
    }
    if let Some(var_15) = &input.squash {
        object.key("Squash").string(var_15.as_str());
    }
    if let Some(var_16) = &input.read_only {
        object.key("ReadOnly").boolean(*var_16);
    }
    if let Some(var_17) = &input.guess_mime_type_enabled {
        object.key("GuessMIMETypeEnabled").boolean(*var_17);
    }
    if let Some(var_18) = &input.requester_pays {
        object.key("RequesterPays").boolean(*var_18);
    }
    if let Some(var_19) = &input.tags {
        let mut array_20 = object.key("Tags").start_array();
        for item_21 in var_19 {
            {
                #[allow(unused_mut)]
                let mut object_22 = array_20.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_22, item_21)?;
                object_22.finish();
            }
        }
        array_20.finish();
    }
    if let Some(var_23) = &input.file_share_name {
        object.key("FileShareName").string(var_23.as_str());
    }
    if let Some(var_24) = &input.cache_attributes {
        #[allow(unused_mut)]
        let mut object_25 = object.key("CacheAttributes").start_object();
        crate::protocol_serde::shape_cache_attributes::ser_cache_attributes(&mut object_25, var_24)?;
        object_25.finish();
    }
    if let Some(var_26) = &input.notification_policy {
        object.key("NotificationPolicy").string(var_26.as_str());
    }
    if let Some(var_27) = &input.vpc_endpoint_dns_name {
        object.key("VPCEndpointDNSName").string(var_27.as_str());
    }
    if let Some(var_28) = &input.bucket_region {
        object.key("BucketRegion").string(var_28.as_str());
    }
    if let Some(var_29) = &input.audit_destination_arn {
        object.key("AuditDestinationARN").string(var_29.as_str());
    }
    Ok(())
}
