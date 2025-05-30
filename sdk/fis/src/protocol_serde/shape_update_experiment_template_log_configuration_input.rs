// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_experiment_template_log_configuration_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::UpdateExperimentTemplateLogConfigurationInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.cloud_watch_logs_configuration {
        #[allow(unused_mut)]
        let mut object_2 = object.key("cloudWatchLogsConfiguration").start_object();
        crate::protocol_serde::shape_experiment_template_cloud_watch_logs_log_configuration_input::ser_experiment_template_cloud_watch_logs_log_configuration_input(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.s3_configuration {
        #[allow(unused_mut)]
        let mut object_4 = object.key("s3Configuration").start_object();
        crate::protocol_serde::shape_experiment_template_s3_log_configuration_input::ser_experiment_template_s3_log_configuration_input(
            &mut object_4,
            var_3,
        )?;
        object_4.finish();
    }
    if let Some(var_5) = &input.log_schema_version {
        object.key("logSchemaVersion").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_5).into()),
        );
    }
    Ok(())
}
