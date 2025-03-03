// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_connector_operation::_describe_connector_operation_output::DescribeConnectorOperationOutputBuilder;

pub use crate::operation::describe_connector_operation::_describe_connector_operation_input::DescribeConnectorOperationInputBuilder;

impl crate::operation::describe_connector_operation::builders::DescribeConnectorOperationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_connector_operation::DescribeConnectorOperationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_connector_operation::DescribeConnectorOperationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_connector_operation();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeConnectorOperation`.
///
/// <p>Returns information about the specified connector's operations.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeConnectorOperationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_connector_operation::builders::DescribeConnectorOperationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_connector_operation::DescribeConnectorOperationOutput,
        crate::operation::describe_connector_operation::DescribeConnectorOperationError,
    > for DescribeConnectorOperationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_connector_operation::DescribeConnectorOperationOutput,
            crate::operation::describe_connector_operation::DescribeConnectorOperationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeConnectorOperationFluentBuilder {
    /// Creates a new `DescribeConnectorOperationFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeConnectorOperation as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_connector_operation::builders::DescribeConnectorOperationInputBuilder {
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
        crate::operation::describe_connector_operation::DescribeConnectorOperationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_connector_operation::DescribeConnectorOperationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_connector_operation::DescribeConnectorOperation::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_connector_operation::DescribeConnectorOperation::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_connector_operation::DescribeConnectorOperationOutput,
        crate::operation::describe_connector_operation::DescribeConnectorOperationError,
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
    /// <p>ARN of the connector operation to be described.</p>
    pub fn connector_operation_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.connector_operation_arn(input.into());
        self
    }
    /// <p>ARN of the connector operation to be described.</p>
    pub fn set_connector_operation_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_connector_operation_arn(input);
        self
    }
    /// <p>ARN of the connector operation to be described.</p>
    pub fn get_connector_operation_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_connector_operation_arn()
    }
}
