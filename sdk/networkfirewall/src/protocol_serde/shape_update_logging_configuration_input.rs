// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_logging_configuration_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_logging_configuration::UpdateLoggingConfigurationInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.firewall_arn {
        object.key("FirewallArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.firewall_name {
        object.key("FirewallName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.logging_configuration {
        #[allow(unused_mut)]
        let mut object_4 = object.key("LoggingConfiguration").start_object();
        crate::protocol_serde::shape_logging_configuration::ser_logging_configuration(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.enable_monitoring_dashboard {
        object.key("EnableMonitoringDashboard").boolean(*var_5);
    }
    Ok(())
}
