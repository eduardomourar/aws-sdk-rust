// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_flow_execution::_get_flow_execution_output::GetFlowExecutionOutputBuilder;

pub use crate::operation::get_flow_execution::_get_flow_execution_input::GetFlowExecutionInputBuilder;

impl crate::operation::get_flow_execution::builders::GetFlowExecutionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_flow_execution::GetFlowExecutionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_flow_execution::GetFlowExecutionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_flow_execution();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetFlowExecution`.
///
/// <p>Retrieves details about a specific flow execution, including its status, start and end times, and any errors that occurred during execution.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetFlowExecutionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_flow_execution::builders::GetFlowExecutionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_flow_execution::GetFlowExecutionOutput,
        crate::operation::get_flow_execution::GetFlowExecutionError,
    > for GetFlowExecutionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_flow_execution::GetFlowExecutionOutput,
            crate::operation::get_flow_execution::GetFlowExecutionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetFlowExecutionFluentBuilder {
    /// Creates a new `GetFlowExecutionFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetFlowExecution as a reference.
    pub fn as_input(&self) -> &crate::operation::get_flow_execution::builders::GetFlowExecutionInputBuilder {
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
        crate::operation::get_flow_execution::GetFlowExecutionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_flow_execution::GetFlowExecutionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_flow_execution::GetFlowExecution::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_flow_execution::GetFlowExecution::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_flow_execution::GetFlowExecutionOutput,
        crate::operation::get_flow_execution::GetFlowExecutionError,
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
    /// <p>The unique identifier of the flow.</p>
    pub fn flow_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.flow_identifier(input.into());
        self
    }
    /// <p>The unique identifier of the flow.</p>
    pub fn set_flow_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_flow_identifier(input);
        self
    }
    /// <p>The unique identifier of the flow.</p>
    pub fn get_flow_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_flow_identifier()
    }
    /// <p>The unique identifier of the flow alias used for the execution.</p>
    pub fn flow_alias_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.flow_alias_identifier(input.into());
        self
    }
    /// <p>The unique identifier of the flow alias used for the execution.</p>
    pub fn set_flow_alias_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_flow_alias_identifier(input);
        self
    }
    /// <p>The unique identifier of the flow alias used for the execution.</p>
    pub fn get_flow_alias_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_flow_alias_identifier()
    }
    /// <p>The unique identifier of the flow execution to retrieve.</p>
    pub fn execution_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.execution_identifier(input.into());
        self
    }
    /// <p>The unique identifier of the flow execution to retrieve.</p>
    pub fn set_execution_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_execution_identifier(input);
        self
    }
    /// <p>The unique identifier of the flow execution to retrieve.</p>
    pub fn get_execution_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_execution_identifier()
    }
}
