// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_service_job::_describe_service_job_output::DescribeServiceJobOutputBuilder;

pub use crate::operation::describe_service_job::_describe_service_job_input::DescribeServiceJobInputBuilder;

impl crate::operation::describe_service_job::builders::DescribeServiceJobInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_service_job::DescribeServiceJobOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_service_job::DescribeServiceJobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_service_job();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeServiceJob`.
///
/// <p>The details of a service job.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeServiceJobFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_service_job::builders::DescribeServiceJobInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_service_job::DescribeServiceJobOutput,
        crate::operation::describe_service_job::DescribeServiceJobError,
    > for DescribeServiceJobFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_service_job::DescribeServiceJobOutput,
            crate::operation::describe_service_job::DescribeServiceJobError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeServiceJobFluentBuilder {
    /// Creates a new `DescribeServiceJobFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeServiceJob as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_service_job::builders::DescribeServiceJobInputBuilder {
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
        crate::operation::describe_service_job::DescribeServiceJobOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_service_job::DescribeServiceJobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_service_job::DescribeServiceJob::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_service_job::DescribeServiceJob::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_service_job::DescribeServiceJobOutput,
        crate::operation::describe_service_job::DescribeServiceJobError,
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
    /// <p>The job ID for the service job to describe.</p>
    pub fn job_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.job_id(input.into());
        self
    }
    /// <p>The job ID for the service job to describe.</p>
    pub fn set_job_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_job_id(input);
        self
    }
    /// <p>The job ID for the service job to describe.</p>
    pub fn get_job_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_job_id()
    }
}
