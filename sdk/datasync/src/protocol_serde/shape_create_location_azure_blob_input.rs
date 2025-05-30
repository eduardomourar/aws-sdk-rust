// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_location_azure_blob_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_location_azure_blob::CreateLocationAzureBlobInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.container_url {
        object.key("ContainerUrl").string(var_1.as_str());
    }
    if let Some(var_2) = &input.authentication_type {
        object.key("AuthenticationType").string(var_2.as_str());
    }
    if let Some(var_3) = &input.sas_configuration {
        #[allow(unused_mut)]
        let mut object_4 = object.key("SasConfiguration").start_object();
        crate::protocol_serde::shape_azure_blob_sas_configuration::ser_azure_blob_sas_configuration(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.blob_type {
        object.key("BlobType").string(var_5.as_str());
    }
    if let Some(var_6) = &input.access_tier {
        object.key("AccessTier").string(var_6.as_str());
    }
    if let Some(var_7) = &input.subdirectory {
        object.key("Subdirectory").string(var_7.as_str());
    }
    if let Some(var_8) = &input.agent_arns {
        let mut array_9 = object.key("AgentArns").start_array();
        for item_10 in var_8 {
            {
                array_9.value().string(item_10.as_str());
            }
        }
        array_9.finish();
    }
    if let Some(var_11) = &input.tags {
        let mut array_12 = object.key("Tags").start_array();
        for item_13 in var_11 {
            {
                #[allow(unused_mut)]
                let mut object_14 = array_12.value().start_object();
                crate::protocol_serde::shape_tag_list_entry::ser_tag_list_entry(&mut object_14, item_13)?;
                object_14.finish();
            }
        }
        array_12.finish();
    }
    if let Some(var_15) = &input.cmk_secret_config {
        #[allow(unused_mut)]
        let mut object_16 = object.key("CmkSecretConfig").start_object();
        crate::protocol_serde::shape_cmk_secret_config::ser_cmk_secret_config(&mut object_16, var_15)?;
        object_16.finish();
    }
    if let Some(var_17) = &input.custom_secret_config {
        #[allow(unused_mut)]
        let mut object_18 = object.key("CustomSecretConfig").start_object();
        crate::protocol_serde::shape_custom_secret_config::ser_custom_secret_config(&mut object_18, var_17)?;
        object_18.finish();
    }
    Ok(())
}
