// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_computation_model_execution_summary::_describe_computation_model_execution_summary_output::DescribeComputationModelExecutionSummaryOutputBuilder;

pub use crate::operation::describe_computation_model_execution_summary::_describe_computation_model_execution_summary_input::DescribeComputationModelExecutionSummaryInputBuilder;

impl crate::operation::describe_computation_model_execution_summary::builders::DescribeComputationModelExecutionSummaryInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_computation_model_execution_summary::DescribeComputationModelExecutionSummaryOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_computation_model_execution_summary::DescribeComputationModelExecutionSummaryError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_computation_model_execution_summary();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeComputationModelExecutionSummary`.
///
/// <p>Retrieves information about the execution summary of a computation model.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeComputationModelExecutionSummaryFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_computation_model_execution_summary::builders::DescribeComputationModelExecutionSummaryInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_computation_model_execution_summary::DescribeComputationModelExecutionSummaryOutput,
        crate::operation::describe_computation_model_execution_summary::DescribeComputationModelExecutionSummaryError,
    > for DescribeComputationModelExecutionSummaryFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_computation_model_execution_summary::DescribeComputationModelExecutionSummaryOutput,
            crate::operation::describe_computation_model_execution_summary::DescribeComputationModelExecutionSummaryError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeComputationModelExecutionSummaryFluentBuilder {
    /// Creates a new `DescribeComputationModelExecutionSummaryFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeComputationModelExecutionSummary as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::describe_computation_model_execution_summary::builders::DescribeComputationModelExecutionSummaryInputBuilder {
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
        crate::operation::describe_computation_model_execution_summary::DescribeComputationModelExecutionSummaryOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_computation_model_execution_summary::DescribeComputationModelExecutionSummaryError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::describe_computation_model_execution_summary::DescribeComputationModelExecutionSummary::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::describe_computation_model_execution_summary::DescribeComputationModelExecutionSummary::orchestrate(&runtime_plugins, input)
            .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_computation_model_execution_summary::DescribeComputationModelExecutionSummaryOutput,
        crate::operation::describe_computation_model_execution_summary::DescribeComputationModelExecutionSummaryError,
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
    /// <p>The ID of the computation model.</p>
    pub fn computation_model_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.computation_model_id(input.into());
        self
    }
    /// <p>The ID of the computation model.</p>
    pub fn set_computation_model_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_computation_model_id(input);
        self
    }
    /// <p>The ID of the computation model.</p>
    pub fn get_computation_model_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_computation_model_id()
    }
    /// <p>The type of the resolved resource.</p>
    pub fn resolve_to_resource_type(mut self, input: crate::types::ResolveToResourceType) -> Self {
        self.inner = self.inner.resolve_to_resource_type(input);
        self
    }
    /// <p>The type of the resolved resource.</p>
    pub fn set_resolve_to_resource_type(mut self, input: ::std::option::Option<crate::types::ResolveToResourceType>) -> Self {
        self.inner = self.inner.set_resolve_to_resource_type(input);
        self
    }
    /// <p>The type of the resolved resource.</p>
    pub fn get_resolve_to_resource_type(&self) -> &::std::option::Option<crate::types::ResolveToResourceType> {
        self.inner.get_resolve_to_resource_type()
    }
    /// <p>The ID of the resolved resource.</p>
    pub fn resolve_to_resource_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resolve_to_resource_id(input.into());
        self
    }
    /// <p>The ID of the resolved resource.</p>
    pub fn set_resolve_to_resource_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resolve_to_resource_id(input);
        self
    }
    /// <p>The ID of the resolved resource.</p>
    pub fn get_resolve_to_resource_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resolve_to_resource_id()
    }
}
