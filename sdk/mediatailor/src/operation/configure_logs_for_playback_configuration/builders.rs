// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::configure_logs_for_playback_configuration::_configure_logs_for_playback_configuration_output::ConfigureLogsForPlaybackConfigurationOutputBuilder;

pub use crate::operation::configure_logs_for_playback_configuration::_configure_logs_for_playback_configuration_input::ConfigureLogsForPlaybackConfigurationInputBuilder;

impl crate::operation::configure_logs_for_playback_configuration::builders::ConfigureLogsForPlaybackConfigurationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::configure_logs_for_playback_configuration::ConfigureLogsForPlaybackConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::configure_logs_for_playback_configuration::ConfigureLogsForPlaybackConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.configure_logs_for_playback_configuration();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ConfigureLogsForPlaybackConfiguration`.
///
/// <p>Defines where AWS Elemental MediaTailor sends logs for the playback configuration.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ConfigureLogsForPlaybackConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::configure_logs_for_playback_configuration::builders::ConfigureLogsForPlaybackConfigurationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::configure_logs_for_playback_configuration::ConfigureLogsForPlaybackConfigurationOutput,
        crate::operation::configure_logs_for_playback_configuration::ConfigureLogsForPlaybackConfigurationError,
    > for ConfigureLogsForPlaybackConfigurationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::configure_logs_for_playback_configuration::ConfigureLogsForPlaybackConfigurationOutput,
            crate::operation::configure_logs_for_playback_configuration::ConfigureLogsForPlaybackConfigurationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ConfigureLogsForPlaybackConfigurationFluentBuilder {
    /// Creates a new `ConfigureLogsForPlaybackConfigurationFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ConfigureLogsForPlaybackConfiguration as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::configure_logs_for_playback_configuration::builders::ConfigureLogsForPlaybackConfigurationInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::configure_logs_for_playback_configuration::ConfigureLogsForPlaybackConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::configure_logs_for_playback_configuration::ConfigureLogsForPlaybackConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::configure_logs_for_playback_configuration::ConfigureLogsForPlaybackConfiguration::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::configure_logs_for_playback_configuration::ConfigureLogsForPlaybackConfiguration::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::configure_logs_for_playback_configuration::ConfigureLogsForPlaybackConfigurationOutput,
        crate::operation::configure_logs_for_playback_configuration::ConfigureLogsForPlaybackConfigurationError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
        self.set_config_override(::std::option::Option::Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The percentage of session logs that MediaTailor sends to your CloudWatch Logs account. For example, if your playback configuration has 1000 sessions and percentEnabled is set to <code>60</code>, MediaTailor sends logs for 600 of the sessions to CloudWatch Logs. MediaTailor decides at random which of the playback configuration sessions to send logs for. If you want to view logs for a specific session, you can use the <a href="https://docs.aws.amazon.com/mediatailor/latest/ug/debug-log-mode.html">debug log mode</a>.</p>
    /// <p>Valid values: <code>0</code> - <code>100</code></p>
    pub fn percent_enabled(mut self, input: i32) -> Self {
        self.inner = self.inner.percent_enabled(input);
        self
    }
    /// <p>The percentage of session logs that MediaTailor sends to your CloudWatch Logs account. For example, if your playback configuration has 1000 sessions and percentEnabled is set to <code>60</code>, MediaTailor sends logs for 600 of the sessions to CloudWatch Logs. MediaTailor decides at random which of the playback configuration sessions to send logs for. If you want to view logs for a specific session, you can use the <a href="https://docs.aws.amazon.com/mediatailor/latest/ug/debug-log-mode.html">debug log mode</a>.</p>
    /// <p>Valid values: <code>0</code> - <code>100</code></p>
    pub fn set_percent_enabled(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_percent_enabled(input);
        self
    }
    /// <p>The percentage of session logs that MediaTailor sends to your CloudWatch Logs account. For example, if your playback configuration has 1000 sessions and percentEnabled is set to <code>60</code>, MediaTailor sends logs for 600 of the sessions to CloudWatch Logs. MediaTailor decides at random which of the playback configuration sessions to send logs for. If you want to view logs for a specific session, you can use the <a href="https://docs.aws.amazon.com/mediatailor/latest/ug/debug-log-mode.html">debug log mode</a>.</p>
    /// <p>Valid values: <code>0</code> - <code>100</code></p>
    pub fn get_percent_enabled(&self) -> &::std::option::Option<i32> {
        self.inner.get_percent_enabled()
    }
    /// <p>The name of the playback configuration.</p>
    pub fn playback_configuration_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.playback_configuration_name(input.into());
        self
    }
    /// <p>The name of the playback configuration.</p>
    pub fn set_playback_configuration_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_playback_configuration_name(input);
        self
    }
    /// <p>The name of the playback configuration.</p>
    pub fn get_playback_configuration_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_playback_configuration_name()
    }
    ///
    /// Appends an item to `EnabledLoggingStrategies`.
    ///
    /// To override the contents of this collection use [`set_enabled_logging_strategies`](Self::set_enabled_logging_strategies).
    ///
    /// <p>The method used for collecting logs from AWS Elemental MediaTailor. To configure MediaTailor to send logs directly to Amazon CloudWatch Logs, choose <code>LEGACY_CLOUDWATCH</code>. To configure MediaTailor to send logs to CloudWatch, which then vends the logs to your destination of choice, choose <code>VENDED_LOGS</code>. Supported destinations are CloudWatch Logs log group, Amazon S3 bucket, and Amazon Data Firehose stream.</p>
    /// <p>To use vended logs, you must configure the delivery destination in Amazon CloudWatch, as described in <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/AWS-logs-and-resource-policy.html#AWS-vended-logs-permissions-V2">Enable logging from AWS services, Logging that requires additional permissions \[V2\]</a>.</p>
    pub fn enabled_logging_strategies(mut self, input: crate::types::LoggingStrategy) -> Self {
        self.inner = self.inner.enabled_logging_strategies(input);
        self
    }
    /// <p>The method used for collecting logs from AWS Elemental MediaTailor. To configure MediaTailor to send logs directly to Amazon CloudWatch Logs, choose <code>LEGACY_CLOUDWATCH</code>. To configure MediaTailor to send logs to CloudWatch, which then vends the logs to your destination of choice, choose <code>VENDED_LOGS</code>. Supported destinations are CloudWatch Logs log group, Amazon S3 bucket, and Amazon Data Firehose stream.</p>
    /// <p>To use vended logs, you must configure the delivery destination in Amazon CloudWatch, as described in <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/AWS-logs-and-resource-policy.html#AWS-vended-logs-permissions-V2">Enable logging from AWS services, Logging that requires additional permissions \[V2\]</a>.</p>
    pub fn set_enabled_logging_strategies(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::LoggingStrategy>>) -> Self {
        self.inner = self.inner.set_enabled_logging_strategies(input);
        self
    }
    /// <p>The method used for collecting logs from AWS Elemental MediaTailor. To configure MediaTailor to send logs directly to Amazon CloudWatch Logs, choose <code>LEGACY_CLOUDWATCH</code>. To configure MediaTailor to send logs to CloudWatch, which then vends the logs to your destination of choice, choose <code>VENDED_LOGS</code>. Supported destinations are CloudWatch Logs log group, Amazon S3 bucket, and Amazon Data Firehose stream.</p>
    /// <p>To use vended logs, you must configure the delivery destination in Amazon CloudWatch, as described in <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/logs/AWS-logs-and-resource-policy.html#AWS-vended-logs-permissions-V2">Enable logging from AWS services, Logging that requires additional permissions \[V2\]</a>.</p>
    pub fn get_enabled_logging_strategies(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::LoggingStrategy>> {
        self.inner.get_enabled_logging_strategies()
    }
    /// <p>The event types that MediaTailor emits in logs for interactions with the ADS.</p>
    pub fn ads_interaction_log(mut self, input: crate::types::AdsInteractionLog) -> Self {
        self.inner = self.inner.ads_interaction_log(input);
        self
    }
    /// <p>The event types that MediaTailor emits in logs for interactions with the ADS.</p>
    pub fn set_ads_interaction_log(mut self, input: ::std::option::Option<crate::types::AdsInteractionLog>) -> Self {
        self.inner = self.inner.set_ads_interaction_log(input);
        self
    }
    /// <p>The event types that MediaTailor emits in logs for interactions with the ADS.</p>
    pub fn get_ads_interaction_log(&self) -> &::std::option::Option<crate::types::AdsInteractionLog> {
        self.inner.get_ads_interaction_log()
    }
    /// <p>The event types that MediaTailor emits in logs for interactions with the origin server.</p>
    pub fn manifest_service_interaction_log(mut self, input: crate::types::ManifestServiceInteractionLog) -> Self {
        self.inner = self.inner.manifest_service_interaction_log(input);
        self
    }
    /// <p>The event types that MediaTailor emits in logs for interactions with the origin server.</p>
    pub fn set_manifest_service_interaction_log(mut self, input: ::std::option::Option<crate::types::ManifestServiceInteractionLog>) -> Self {
        self.inner = self.inner.set_manifest_service_interaction_log(input);
        self
    }
    /// <p>The event types that MediaTailor emits in logs for interactions with the origin server.</p>
    pub fn get_manifest_service_interaction_log(&self) -> &::std::option::Option<crate::types::ManifestServiceInteractionLog> {
        self.inner.get_manifest_service_interaction_log()
    }
}
