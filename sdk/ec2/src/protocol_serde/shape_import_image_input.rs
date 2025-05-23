// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_import_image_input_input_input(
    input: &crate::operation::import_image::ImportImageInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "ImportImage", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Architecture");
    if let Some(var_2) = &input.architecture {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("ClientData");
    if let Some(var_4) = &input.client_data {
        crate::protocol_serde::shape_client_data::ser_client_data(scope_3, var_4)?;
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("ClientToken");
    if let Some(var_6) = &input.client_token {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("Description");
    if let Some(var_8) = &input.description {
        scope_7.string(var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("DiskContainer");
    if let Some(var_10) = &input.disk_containers {
        if !var_10.is_empty() {
            let mut list_12 = scope_9.start_list(true, Some("item"));
            for item_11 in var_10 {
                #[allow(unused_mut)]
                let mut entry_13 = list_12.entry();
                crate::protocol_serde::shape_image_disk_container::ser_image_disk_container(entry_13, item_11)?;
            }
            list_12.finish();
        }
    }
    #[allow(unused_mut)]
    let mut scope_14 = writer.prefix("DryRun");
    if let Some(var_15) = &input.dry_run {
        scope_14.boolean(*var_15);
    }
    #[allow(unused_mut)]
    let mut scope_16 = writer.prefix("Encrypted");
    if let Some(var_17) = &input.encrypted {
        scope_16.boolean(*var_17);
    }
    #[allow(unused_mut)]
    let mut scope_18 = writer.prefix("Hypervisor");
    if let Some(var_19) = &input.hypervisor {
        scope_18.string(var_19);
    }
    #[allow(unused_mut)]
    let mut scope_20 = writer.prefix("KmsKeyId");
    if let Some(var_21) = &input.kms_key_id {
        scope_20.string(var_21);
    }
    #[allow(unused_mut)]
    let mut scope_22 = writer.prefix("LicenseType");
    if let Some(var_23) = &input.license_type {
        scope_22.string(var_23);
    }
    #[allow(unused_mut)]
    let mut scope_24 = writer.prefix("Platform");
    if let Some(var_25) = &input.platform {
        scope_24.string(var_25);
    }
    #[allow(unused_mut)]
    let mut scope_26 = writer.prefix("RoleName");
    if let Some(var_27) = &input.role_name {
        scope_26.string(var_27);
    }
    #[allow(unused_mut)]
    let mut scope_28 = writer.prefix("LicenseSpecifications");
    if let Some(var_29) = &input.license_specifications {
        if !var_29.is_empty() {
            let mut list_31 = scope_28.start_list(true, Some("item"));
            for item_30 in var_29 {
                #[allow(unused_mut)]
                let mut entry_32 = list_31.entry();
                crate::protocol_serde::shape_import_image_license_configuration_request::ser_import_image_license_configuration_request(
                    entry_32, item_30,
                )?;
            }
            list_31.finish();
        }
    }
    #[allow(unused_mut)]
    let mut scope_33 = writer.prefix("TagSpecification");
    if let Some(var_34) = &input.tag_specifications {
        if !var_34.is_empty() {
            let mut list_36 = scope_33.start_list(true, Some("item"));
            for item_35 in var_34 {
                #[allow(unused_mut)]
                let mut entry_37 = list_36.entry();
                crate::protocol_serde::shape_tag_specification::ser_tag_specification(entry_37, item_35)?;
            }
            list_36.finish();
        }
    }
    #[allow(unused_mut)]
    let mut scope_38 = writer.prefix("UsageOperation");
    if let Some(var_39) = &input.usage_operation {
        scope_38.string(var_39);
    }
    #[allow(unused_mut)]
    let mut scope_40 = writer.prefix("BootMode");
    if let Some(var_41) = &input.boot_mode {
        scope_40.string(var_41.as_str());
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
