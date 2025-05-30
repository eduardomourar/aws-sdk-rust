// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_flow_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_flow::UpdateFlowInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.maintenance {
        #[allow(unused_mut)]
        let mut object_2 = object.key("maintenance").start_object();
        crate::protocol_serde::shape_update_maintenance::ser_update_maintenance(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.ndi_config {
        #[allow(unused_mut)]
        let mut object_4 = object.key("ndiConfig").start_object();
        crate::protocol_serde::shape_ndi_config::ser_ndi_config(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.source_failover_config {
        #[allow(unused_mut)]
        let mut object_6 = object.key("sourceFailoverConfig").start_object();
        crate::protocol_serde::shape_update_failover_config::ser_update_failover_config(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.source_monitoring_config {
        #[allow(unused_mut)]
        let mut object_8 = object.key("sourceMonitoringConfig").start_object();
        crate::protocol_serde::shape_monitoring_config::ser_monitoring_config(&mut object_8, var_7)?;
        object_8.finish();
    }
    Ok(())
}
