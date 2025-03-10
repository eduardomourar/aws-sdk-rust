// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_instance_specification(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::InstanceSpecification,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("InstanceId");
    if let Some(var_2) = &input.instance_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("ExcludeBootVolume");
    if let Some(var_4) = &input.exclude_boot_volume {
        scope_3.boolean(*var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("ExcludeDataVolumeId");
    if let Some(var_6) = &input.exclude_data_volume_ids {
        if !var_6.is_empty() {
            let mut list_8 = scope_5.start_list(true, Some("VolumeId"));
            for item_7 in var_6 {
                #[allow(unused_mut)]
                let mut entry_9 = list_8.entry();
                entry_9.string(item_7);
            }
            list_8.finish();
        }
    }
    Ok(())
}
