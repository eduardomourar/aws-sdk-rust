// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutRuntimeLogConfiguration`](crate::operation::put_runtime_log_configuration::builders::PutRuntimeLogConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`managed_thing_id(impl Into<String>)`](crate::operation::put_runtime_log_configuration::builders::PutRuntimeLogConfigurationFluentBuilder::managed_thing_id) / [`set_managed_thing_id(Option<String>)`](crate::operation::put_runtime_log_configuration::builders::PutRuntimeLogConfigurationFluentBuilder::set_managed_thing_id):<br>required: **true**<br><p>The id for a managed thing.</p><br>
    ///   - [`runtime_log_configurations(RuntimeLogConfigurations)`](crate::operation::put_runtime_log_configuration::builders::PutRuntimeLogConfigurationFluentBuilder::runtime_log_configurations) / [`set_runtime_log_configurations(Option<RuntimeLogConfigurations>)`](crate::operation::put_runtime_log_configuration::builders::PutRuntimeLogConfigurationFluentBuilder::set_runtime_log_configurations):<br>required: **true**<br><p>The runtime log configuration for a managed thing.</p><br>
    /// - On success, responds with [`PutRuntimeLogConfigurationOutput`](crate::operation::put_runtime_log_configuration::PutRuntimeLogConfigurationOutput)
    /// - On failure, responds with [`SdkError<PutRuntimeLogConfigurationError>`](crate::operation::put_runtime_log_configuration::PutRuntimeLogConfigurationError)
    pub fn put_runtime_log_configuration(
        &self,
    ) -> crate::operation::put_runtime_log_configuration::builders::PutRuntimeLogConfigurationFluentBuilder {
        crate::operation::put_runtime_log_configuration::builders::PutRuntimeLogConfigurationFluentBuilder::new(self.handle.clone())
    }
}
