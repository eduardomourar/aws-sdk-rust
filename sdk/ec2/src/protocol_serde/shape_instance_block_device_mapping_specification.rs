// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_instance_block_device_mapping_specification(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::InstanceBlockDeviceMappingSpecification,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DeviceName");
    if let Some(var_2) = &input.device_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Ebs");
    if let Some(var_4) = &input.ebs {
        crate::protocol_serde::shape_ebs_instance_block_device_specification::ser_ebs_instance_block_device_specification(scope_3, var_4)?;
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("VirtualName");
    if let Some(var_6) = &input.virtual_name {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("NoDevice");
    if let Some(var_8) = &input.no_device {
        scope_7.string(var_8);
    }
    Ok(())
}
