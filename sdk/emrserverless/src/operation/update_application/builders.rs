// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_application::_update_application_output::UpdateApplicationOutputBuilder;

pub use crate::operation::update_application::_update_application_input::UpdateApplicationInputBuilder;

impl crate::operation::update_application::builders::UpdateApplicationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_application::UpdateApplicationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_application::UpdateApplicationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_application();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateApplication`.
///
/// <p>Updates a specified application. An application has to be in a stopped or created state in order to be updated.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateApplicationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_application::builders::UpdateApplicationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_application::UpdateApplicationOutput,
        crate::operation::update_application::UpdateApplicationError,
    > for UpdateApplicationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_application::UpdateApplicationOutput,
            crate::operation::update_application::UpdateApplicationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateApplicationFluentBuilder {
    /// Creates a new `UpdateApplicationFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateApplication as a reference.
    pub fn as_input(&self) -> &crate::operation::update_application::builders::UpdateApplicationInputBuilder {
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
        crate::operation::update_application::UpdateApplicationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_application::UpdateApplicationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_application::UpdateApplication::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_application::UpdateApplication::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_application::UpdateApplicationOutput,
        crate::operation::update_application::UpdateApplicationError,
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
    /// <p>The ID of the application to update.</p>
    pub fn application_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.application_id(input.into());
        self
    }
    /// <p>The ID of the application to update.</p>
    pub fn set_application_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_application_id(input);
        self
    }
    /// <p>The ID of the application to update.</p>
    pub fn get_application_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_application_id()
    }
    /// <p>The client idempotency token of the application to update. Its value must be unique for each request.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>The client idempotency token of the application to update. Its value must be unique for each request.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>The client idempotency token of the application to update. Its value must be unique for each request.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    ///
    /// Adds a key-value pair to `initialCapacity`.
    ///
    /// To override the contents of this collection use [`set_initial_capacity`](Self::set_initial_capacity).
    ///
    /// <p>The capacity to initialize when the application is updated.</p>
    pub fn initial_capacity(mut self, k: impl ::std::convert::Into<::std::string::String>, v: crate::types::InitialCapacityConfig) -> Self {
        self.inner = self.inner.initial_capacity(k.into(), v);
        self
    }
    /// <p>The capacity to initialize when the application is updated.</p>
    pub fn set_initial_capacity(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::InitialCapacityConfig>>,
    ) -> Self {
        self.inner = self.inner.set_initial_capacity(input);
        self
    }
    /// <p>The capacity to initialize when the application is updated.</p>
    pub fn get_initial_capacity(
        &self,
    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::InitialCapacityConfig>> {
        self.inner.get_initial_capacity()
    }
    /// <p>The maximum capacity to allocate when the application is updated. This is cumulative across all workers at any given point in time during the lifespan of the application. No new resources will be created once any one of the defined limits is hit.</p>
    pub fn maximum_capacity(mut self, input: crate::types::MaximumAllowedResources) -> Self {
        self.inner = self.inner.maximum_capacity(input);
        self
    }
    /// <p>The maximum capacity to allocate when the application is updated. This is cumulative across all workers at any given point in time during the lifespan of the application. No new resources will be created once any one of the defined limits is hit.</p>
    pub fn set_maximum_capacity(mut self, input: ::std::option::Option<crate::types::MaximumAllowedResources>) -> Self {
        self.inner = self.inner.set_maximum_capacity(input);
        self
    }
    /// <p>The maximum capacity to allocate when the application is updated. This is cumulative across all workers at any given point in time during the lifespan of the application. No new resources will be created once any one of the defined limits is hit.</p>
    pub fn get_maximum_capacity(&self) -> &::std::option::Option<crate::types::MaximumAllowedResources> {
        self.inner.get_maximum_capacity()
    }
    /// <p>The configuration for an application to automatically start on job submission.</p>
    pub fn auto_start_configuration(mut self, input: crate::types::AutoStartConfig) -> Self {
        self.inner = self.inner.auto_start_configuration(input);
        self
    }
    /// <p>The configuration for an application to automatically start on job submission.</p>
    pub fn set_auto_start_configuration(mut self, input: ::std::option::Option<crate::types::AutoStartConfig>) -> Self {
        self.inner = self.inner.set_auto_start_configuration(input);
        self
    }
    /// <p>The configuration for an application to automatically start on job submission.</p>
    pub fn get_auto_start_configuration(&self) -> &::std::option::Option<crate::types::AutoStartConfig> {
        self.inner.get_auto_start_configuration()
    }
    /// <p>The configuration for an application to automatically stop after a certain amount of time being idle.</p>
    pub fn auto_stop_configuration(mut self, input: crate::types::AutoStopConfig) -> Self {
        self.inner = self.inner.auto_stop_configuration(input);
        self
    }
    /// <p>The configuration for an application to automatically stop after a certain amount of time being idle.</p>
    pub fn set_auto_stop_configuration(mut self, input: ::std::option::Option<crate::types::AutoStopConfig>) -> Self {
        self.inner = self.inner.set_auto_stop_configuration(input);
        self
    }
    /// <p>The configuration for an application to automatically stop after a certain amount of time being idle.</p>
    pub fn get_auto_stop_configuration(&self) -> &::std::option::Option<crate::types::AutoStopConfig> {
        self.inner.get_auto_stop_configuration()
    }
    /// <p>The network configuration for customer VPC connectivity.</p>
    pub fn network_configuration(mut self, input: crate::types::NetworkConfiguration) -> Self {
        self.inner = self.inner.network_configuration(input);
        self
    }
    /// <p>The network configuration for customer VPC connectivity.</p>
    pub fn set_network_configuration(mut self, input: ::std::option::Option<crate::types::NetworkConfiguration>) -> Self {
        self.inner = self.inner.set_network_configuration(input);
        self
    }
    /// <p>The network configuration for customer VPC connectivity.</p>
    pub fn get_network_configuration(&self) -> &::std::option::Option<crate::types::NetworkConfiguration> {
        self.inner.get_network_configuration()
    }
    /// <p>The CPU architecture of an application.</p>
    pub fn architecture(mut self, input: crate::types::Architecture) -> Self {
        self.inner = self.inner.architecture(input);
        self
    }
    /// <p>The CPU architecture of an application.</p>
    pub fn set_architecture(mut self, input: ::std::option::Option<crate::types::Architecture>) -> Self {
        self.inner = self.inner.set_architecture(input);
        self
    }
    /// <p>The CPU architecture of an application.</p>
    pub fn get_architecture(&self) -> &::std::option::Option<crate::types::Architecture> {
        self.inner.get_architecture()
    }
    /// <p>The image configuration to be used for all worker types. You can either set this parameter or <code>imageConfiguration</code> for each worker type in <code>WorkerTypeSpecificationInput</code>.</p>
    pub fn image_configuration(mut self, input: crate::types::ImageConfigurationInput) -> Self {
        self.inner = self.inner.image_configuration(input);
        self
    }
    /// <p>The image configuration to be used for all worker types. You can either set this parameter or <code>imageConfiguration</code> for each worker type in <code>WorkerTypeSpecificationInput</code>.</p>
    pub fn set_image_configuration(mut self, input: ::std::option::Option<crate::types::ImageConfigurationInput>) -> Self {
        self.inner = self.inner.set_image_configuration(input);
        self
    }
    /// <p>The image configuration to be used for all worker types. You can either set this parameter or <code>imageConfiguration</code> for each worker type in <code>WorkerTypeSpecificationInput</code>.</p>
    pub fn get_image_configuration(&self) -> &::std::option::Option<crate::types::ImageConfigurationInput> {
        self.inner.get_image_configuration()
    }
    ///
    /// Adds a key-value pair to `workerTypeSpecifications`.
    ///
    /// To override the contents of this collection use [`set_worker_type_specifications`](Self::set_worker_type_specifications).
    ///
    /// <p>The key-value pairs that specify worker type to <code>WorkerTypeSpecificationInput</code>. This parameter must contain all valid worker types for a Spark or Hive application. Valid worker types include <code>Driver</code> and <code>Executor</code> for Spark applications and <code>HiveDriver</code> and <code>TezTask</code> for Hive applications. You can either set image details in this parameter for each worker type, or in <code>imageConfiguration</code> for all worker types.</p>
    pub fn worker_type_specifications(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: crate::types::WorkerTypeSpecificationInput,
    ) -> Self {
        self.inner = self.inner.worker_type_specifications(k.into(), v);
        self
    }
    /// <p>The key-value pairs that specify worker type to <code>WorkerTypeSpecificationInput</code>. This parameter must contain all valid worker types for a Spark or Hive application. Valid worker types include <code>Driver</code> and <code>Executor</code> for Spark applications and <code>HiveDriver</code> and <code>TezTask</code> for Hive applications. You can either set image details in this parameter for each worker type, or in <code>imageConfiguration</code> for all worker types.</p>
    pub fn set_worker_type_specifications(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::WorkerTypeSpecificationInput>>,
    ) -> Self {
        self.inner = self.inner.set_worker_type_specifications(input);
        self
    }
    /// <p>The key-value pairs that specify worker type to <code>WorkerTypeSpecificationInput</code>. This parameter must contain all valid worker types for a Spark or Hive application. Valid worker types include <code>Driver</code> and <code>Executor</code> for Spark applications and <code>HiveDriver</code> and <code>TezTask</code> for Hive applications. You can either set image details in this parameter for each worker type, or in <code>imageConfiguration</code> for all worker types.</p>
    pub fn get_worker_type_specifications(
        &self,
    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::WorkerTypeSpecificationInput>> {
        self.inner.get_worker_type_specifications()
    }
    /// <p>The interactive configuration object that contains new interactive use cases when the application is updated.</p>
    pub fn interactive_configuration(mut self, input: crate::types::InteractiveConfiguration) -> Self {
        self.inner = self.inner.interactive_configuration(input);
        self
    }
    /// <p>The interactive configuration object that contains new interactive use cases when the application is updated.</p>
    pub fn set_interactive_configuration(mut self, input: ::std::option::Option<crate::types::InteractiveConfiguration>) -> Self {
        self.inner = self.inner.set_interactive_configuration(input);
        self
    }
    /// <p>The interactive configuration object that contains new interactive use cases when the application is updated.</p>
    pub fn get_interactive_configuration(&self) -> &::std::option::Option<crate::types::InteractiveConfiguration> {
        self.inner.get_interactive_configuration()
    }
    /// <p>The Amazon EMR release label for the application. You can change the release label to use a different release of Amazon EMR.</p>
    pub fn release_label(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.release_label(input.into());
        self
    }
    /// <p>The Amazon EMR release label for the application. You can change the release label to use a different release of Amazon EMR.</p>
    pub fn set_release_label(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_release_label(input);
        self
    }
    /// <p>The Amazon EMR release label for the application. You can change the release label to use a different release of Amazon EMR.</p>
    pub fn get_release_label(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_release_label()
    }
    ///
    /// Appends an item to `runtimeConfiguration`.
    ///
    /// To override the contents of this collection use [`set_runtime_configuration`](Self::set_runtime_configuration).
    ///
    /// <p>The <a href="https://docs.aws.amazon.com/emr-serverless/latest/APIReference/API_Configuration.html">Configuration</a> specifications to use when updating an application. Each configuration consists of a classification and properties. This configuration is applied across all the job runs submitted under the application.</p>
    pub fn runtime_configuration(mut self, input: crate::types::Configuration) -> Self {
        self.inner = self.inner.runtime_configuration(input);
        self
    }
    /// <p>The <a href="https://docs.aws.amazon.com/emr-serverless/latest/APIReference/API_Configuration.html">Configuration</a> specifications to use when updating an application. Each configuration consists of a classification and properties. This configuration is applied across all the job runs submitted under the application.</p>
    pub fn set_runtime_configuration(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Configuration>>) -> Self {
        self.inner = self.inner.set_runtime_configuration(input);
        self
    }
    /// <p>The <a href="https://docs.aws.amazon.com/emr-serverless/latest/APIReference/API_Configuration.html">Configuration</a> specifications to use when updating an application. Each configuration consists of a classification and properties. This configuration is applied across all the job runs submitted under the application.</p>
    pub fn get_runtime_configuration(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Configuration>> {
        self.inner.get_runtime_configuration()
    }
    /// <p>The configuration setting for monitoring.</p>
    pub fn monitoring_configuration(mut self, input: crate::types::MonitoringConfiguration) -> Self {
        self.inner = self.inner.monitoring_configuration(input);
        self
    }
    /// <p>The configuration setting for monitoring.</p>
    pub fn set_monitoring_configuration(mut self, input: ::std::option::Option<crate::types::MonitoringConfiguration>) -> Self {
        self.inner = self.inner.set_monitoring_configuration(input);
        self
    }
    /// <p>The configuration setting for monitoring.</p>
    pub fn get_monitoring_configuration(&self) -> &::std::option::Option<crate::types::MonitoringConfiguration> {
        self.inner.get_monitoring_configuration()
    }
    /// <p>The scheduler configuration for batch and streaming jobs running on this application. Supported with release labels emr-7.0.0 and above.</p>
    pub fn scheduler_configuration(mut self, input: crate::types::SchedulerConfiguration) -> Self {
        self.inner = self.inner.scheduler_configuration(input);
        self
    }
    /// <p>The scheduler configuration for batch and streaming jobs running on this application. Supported with release labels emr-7.0.0 and above.</p>
    pub fn set_scheduler_configuration(mut self, input: ::std::option::Option<crate::types::SchedulerConfiguration>) -> Self {
        self.inner = self.inner.set_scheduler_configuration(input);
        self
    }
    /// <p>The scheduler configuration for batch and streaming jobs running on this application. Supported with release labels emr-7.0.0 and above.</p>
    pub fn get_scheduler_configuration(&self) -> &::std::option::Option<crate::types::SchedulerConfiguration> {
        self.inner.get_scheduler_configuration()
    }
    /// <p>Specifies the IAM Identity Center configuration used to enable or disable trusted identity propagation. When provided, this configuration determines how the application interacts with IAM Identity Center for user authentication and access control.</p>
    pub fn identity_center_configuration(mut self, input: crate::types::IdentityCenterConfigurationInput) -> Self {
        self.inner = self.inner.identity_center_configuration(input);
        self
    }
    /// <p>Specifies the IAM Identity Center configuration used to enable or disable trusted identity propagation. When provided, this configuration determines how the application interacts with IAM Identity Center for user authentication and access control.</p>
    pub fn set_identity_center_configuration(mut self, input: ::std::option::Option<crate::types::IdentityCenterConfigurationInput>) -> Self {
        self.inner = self.inner.set_identity_center_configuration(input);
        self
    }
    /// <p>Specifies the IAM Identity Center configuration used to enable or disable trusted identity propagation. When provided, this configuration determines how the application interacts with IAM Identity Center for user authentication and access control.</p>
    pub fn get_identity_center_configuration(&self) -> &::std::option::Option<crate::types::IdentityCenterConfigurationInput> {
        self.inner.get_identity_center_configuration()
    }
}
