// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateQueryLoggingConfiguration`](crate::operation::update_query_logging_configuration::builders::UpdateQueryLoggingConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`workspace_id(impl Into<String>)`](crate::operation::update_query_logging_configuration::builders::UpdateQueryLoggingConfigurationFluentBuilder::workspace_id) / [`set_workspace_id(Option<String>)`](crate::operation::update_query_logging_configuration::builders::UpdateQueryLoggingConfigurationFluentBuilder::set_workspace_id):<br>required: **true**<br><p>The ID of the workspace for which to update the query logging configuration.</p><br>
    ///   - [`destinations(LoggingDestination)`](crate::operation::update_query_logging_configuration::builders::UpdateQueryLoggingConfigurationFluentBuilder::destinations) / [`set_destinations(Option<Vec::<LoggingDestination>>)`](crate::operation::update_query_logging_configuration::builders::UpdateQueryLoggingConfigurationFluentBuilder::set_destinations):<br>required: **true**<br><p>The destinations where query logs will be sent. Only CloudWatch Logs destination is supported. The list must contain exactly one element.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::update_query_logging_configuration::builders::UpdateQueryLoggingConfigurationFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::update_query_logging_configuration::builders::UpdateQueryLoggingConfigurationFluentBuilder::set_client_token):<br>required: **false**<br><p>(Optional) A unique, case-sensitive identifier that you can provide to ensure the idempotency of the request.</p><br>
    /// - On success, responds with [`UpdateQueryLoggingConfigurationOutput`](crate::operation::update_query_logging_configuration::UpdateQueryLoggingConfigurationOutput) with field(s):
    ///   - [`status(Option<QueryLoggingConfigurationStatus>)`](crate::operation::update_query_logging_configuration::UpdateQueryLoggingConfigurationOutput::status): <p>The current status of the query logging configuration.</p>
    /// - On failure, responds with [`SdkError<UpdateQueryLoggingConfigurationError>`](crate::operation::update_query_logging_configuration::UpdateQueryLoggingConfigurationError)
    pub fn update_query_logging_configuration(
        &self,
    ) -> crate::operation::update_query_logging_configuration::builders::UpdateQueryLoggingConfigurationFluentBuilder {
        crate::operation::update_query_logging_configuration::builders::UpdateQueryLoggingConfigurationFluentBuilder::new(self.handle.clone())
    }
}
