// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetLoggingConfiguration`](crate::operation::get_logging_configuration::builders::GetLoggingConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl Into<String>)`](crate::operation::get_logging_configuration::builders::GetLoggingConfigurationFluentBuilder::resource_arn) / [`set_resource_arn(Option<String>)`](crate::operation::get_logging_configuration::builders::GetLoggingConfigurationFluentBuilder::set_resource_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the web ACL for which you want to get the <code>LoggingConfiguration</code>.</p><br>
    ///   - [`log_type(LogType)`](crate::operation::get_logging_configuration::builders::GetLoggingConfigurationFluentBuilder::log_type) / [`set_log_type(Option<LogType>)`](crate::operation::get_logging_configuration::builders::GetLoggingConfigurationFluentBuilder::set_log_type):<br>required: **false**<br><p>Used to distinguish between various logging options. Currently, there is one option.</p> <p>Default: <code>WAF_LOGS</code></p><br>
    ///   - [`log_scope(LogScope)`](crate::operation::get_logging_configuration::builders::GetLoggingConfigurationFluentBuilder::log_scope) / [`set_log_scope(Option<LogScope>)`](crate::operation::get_logging_configuration::builders::GetLoggingConfigurationFluentBuilder::set_log_scope):<br>required: **false**<br><p>The owner of the logging configuration, which must be set to <code>CUSTOMER</code> for the configurations that you manage.</p> <p>The log scope <code>SECURITY_LAKE</code> indicates a configuration that is managed through Amazon Security Lake. You can use Security Lake to collect log and event data from various sources for normalization, analysis, and management. For information, see <a href="https://docs.aws.amazon.com/security-lake/latest/userguide/internal-sources.html">Collecting data from Amazon Web Services services</a> in the <i>Amazon Security Lake user guide</i>.</p> <p>Default: <code>CUSTOMER</code></p><br>
    /// - On success, responds with [`GetLoggingConfigurationOutput`](crate::operation::get_logging_configuration::GetLoggingConfigurationOutput) with field(s):
    ///   - [`logging_configuration(Option<LoggingConfiguration>)`](crate::operation::get_logging_configuration::GetLoggingConfigurationOutput::logging_configuration): <p>The <code>LoggingConfiguration</code> for the specified web ACL.</p>
    /// - On failure, responds with [`SdkError<GetLoggingConfigurationError>`](crate::operation::get_logging_configuration::GetLoggingConfigurationError)
    pub fn get_logging_configuration(&self) -> crate::operation::get_logging_configuration::builders::GetLoggingConfigurationFluentBuilder {
        crate::operation::get_logging_configuration::builders::GetLoggingConfigurationFluentBuilder::new(self.handle.clone())
    }
}
