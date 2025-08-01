// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_service_environment::_create_service_environment_output::CreateServiceEnvironmentOutputBuilder;

pub use crate::operation::create_service_environment::_create_service_environment_input::CreateServiceEnvironmentInputBuilder;

impl crate::operation::create_service_environment::builders::CreateServiceEnvironmentInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_service_environment::CreateServiceEnvironmentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_service_environment::CreateServiceEnvironmentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_service_environment();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateServiceEnvironment`.
///
/// <p>Creates a service environment for running service jobs. Service environments define capacity limits for specific service types such as SageMaker Training jobs.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateServiceEnvironmentFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_service_environment::builders::CreateServiceEnvironmentInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_service_environment::CreateServiceEnvironmentOutput,
        crate::operation::create_service_environment::CreateServiceEnvironmentError,
    > for CreateServiceEnvironmentFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_service_environment::CreateServiceEnvironmentOutput,
            crate::operation::create_service_environment::CreateServiceEnvironmentError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateServiceEnvironmentFluentBuilder {
    /// Creates a new `CreateServiceEnvironmentFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateServiceEnvironment as a reference.
    pub fn as_input(&self) -> &crate::operation::create_service_environment::builders::CreateServiceEnvironmentInputBuilder {
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
        crate::operation::create_service_environment::CreateServiceEnvironmentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_service_environment::CreateServiceEnvironmentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_service_environment::CreateServiceEnvironment::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_service_environment::CreateServiceEnvironment::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_service_environment::CreateServiceEnvironmentOutput,
        crate::operation::create_service_environment::CreateServiceEnvironmentError,
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
    /// <p>The name for the service environment. It can be up to 128 characters long and can contain letters, numbers, hyphens (-), and underscores (_).</p>
    pub fn service_environment_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.service_environment_name(input.into());
        self
    }
    /// <p>The name for the service environment. It can be up to 128 characters long and can contain letters, numbers, hyphens (-), and underscores (_).</p>
    pub fn set_service_environment_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_service_environment_name(input);
        self
    }
    /// <p>The name for the service environment. It can be up to 128 characters long and can contain letters, numbers, hyphens (-), and underscores (_).</p>
    pub fn get_service_environment_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_service_environment_name()
    }
    /// <p>The type of service environment. For SageMaker Training jobs, specify <code>SAGEMAKER_TRAINING</code>.</p>
    pub fn service_environment_type(mut self, input: crate::types::ServiceEnvironmentType) -> Self {
        self.inner = self.inner.service_environment_type(input);
        self
    }
    /// <p>The type of service environment. For SageMaker Training jobs, specify <code>SAGEMAKER_TRAINING</code>.</p>
    pub fn set_service_environment_type(mut self, input: ::std::option::Option<crate::types::ServiceEnvironmentType>) -> Self {
        self.inner = self.inner.set_service_environment_type(input);
        self
    }
    /// <p>The type of service environment. For SageMaker Training jobs, specify <code>SAGEMAKER_TRAINING</code>.</p>
    pub fn get_service_environment_type(&self) -> &::std::option::Option<crate::types::ServiceEnvironmentType> {
        self.inner.get_service_environment_type()
    }
    /// <p>The state of the service environment. Valid values are <code>ENABLED</code> and <code>DISABLED</code>. The default value is <code>ENABLED</code>.</p>
    pub fn state(mut self, input: crate::types::ServiceEnvironmentState) -> Self {
        self.inner = self.inner.state(input);
        self
    }
    /// <p>The state of the service environment. Valid values are <code>ENABLED</code> and <code>DISABLED</code>. The default value is <code>ENABLED</code>.</p>
    pub fn set_state(mut self, input: ::std::option::Option<crate::types::ServiceEnvironmentState>) -> Self {
        self.inner = self.inner.set_state(input);
        self
    }
    /// <p>The state of the service environment. Valid values are <code>ENABLED</code> and <code>DISABLED</code>. The default value is <code>ENABLED</code>.</p>
    pub fn get_state(&self) -> &::std::option::Option<crate::types::ServiceEnvironmentState> {
        self.inner.get_state()
    }
    ///
    /// Appends an item to `capacityLimits`.
    ///
    /// To override the contents of this collection use [`set_capacity_limits`](Self::set_capacity_limits).
    ///
    /// <p>The capacity limits for the service environment. The number of instances a job consumes is the total number of instances requested in the submit training job request resource configuration.</p>
    pub fn capacity_limits(mut self, input: crate::types::CapacityLimit) -> Self {
        self.inner = self.inner.capacity_limits(input);
        self
    }
    /// <p>The capacity limits for the service environment. The number of instances a job consumes is the total number of instances requested in the submit training job request resource configuration.</p>
    pub fn set_capacity_limits(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::CapacityLimit>>) -> Self {
        self.inner = self.inner.set_capacity_limits(input);
        self
    }
    /// <p>The capacity limits for the service environment. The number of instances a job consumes is the total number of instances requested in the submit training job request resource configuration.</p>
    pub fn get_capacity_limits(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::CapacityLimit>> {
        self.inner.get_capacity_limits()
    }
    ///
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags that you apply to the service environment to help you categorize and organize your resources. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your Batch resources</a>.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>The tags that you apply to the service environment to help you categorize and organize your resources. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your Batch resources</a>.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The tags that you apply to the service environment to help you categorize and organize your resources. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your Batch resources</a>.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_tags()
    }
}
