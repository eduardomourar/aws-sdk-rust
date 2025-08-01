// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_cloud_connectors::_list_cloud_connectors_output::ListCloudConnectorsOutputBuilder;

pub use crate::operation::list_cloud_connectors::_list_cloud_connectors_input::ListCloudConnectorsInputBuilder;

impl crate::operation::list_cloud_connectors::builders::ListCloudConnectorsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_cloud_connectors::ListCloudConnectorsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_cloud_connectors::ListCloudConnectorsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_cloud_connectors();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListCloudConnectors`.
///
/// <p>Returns a list of connectors based on permissions.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListCloudConnectorsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_cloud_connectors::builders::ListCloudConnectorsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_cloud_connectors::ListCloudConnectorsOutput,
        crate::operation::list_cloud_connectors::ListCloudConnectorsError,
    > for ListCloudConnectorsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_cloud_connectors::ListCloudConnectorsOutput,
            crate::operation::list_cloud_connectors::ListCloudConnectorsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListCloudConnectorsFluentBuilder {
    /// Creates a new `ListCloudConnectorsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListCloudConnectors as a reference.
    pub fn as_input(&self) -> &crate::operation::list_cloud_connectors::builders::ListCloudConnectorsInputBuilder {
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
        crate::operation::list_cloud_connectors::ListCloudConnectorsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_cloud_connectors::ListCloudConnectorsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_cloud_connectors::ListCloudConnectors::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_cloud_connectors::ListCloudConnectors::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_cloud_connectors::ListCloudConnectorsOutput,
        crate::operation::list_cloud_connectors::ListCloudConnectorsError,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_cloud_connectors::paginator::ListCloudConnectorsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_cloud_connectors::paginator::ListCloudConnectorsPaginator {
        crate::operation::list_cloud_connectors::paginator::ListCloudConnectorsPaginator::new(self.handle, self.inner)
    }
    /// <p>The type of cloud connectors to filter by when listing available connectors.</p>
    pub fn r#type(mut self, input: crate::types::CloudConnectorType) -> Self {
        self.inner = self.inner.r#type(input);
        self
    }
    /// <p>The type of cloud connectors to filter by when listing available connectors.</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::CloudConnectorType>) -> Self {
        self.inner = self.inner.set_type(input);
        self
    }
    /// <p>The type of cloud connectors to filter by when listing available connectors.</p>
    pub fn get_type(&self) -> &::std::option::Option<crate::types::CloudConnectorType> {
        self.inner.get_type()
    }
    /// <p>The Amazon Resource Name (ARN) of the Lambda function to filter cloud connectors by.</p>
    pub fn lambda_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.lambda_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Lambda function to filter cloud connectors by.</p>
    pub fn set_lambda_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_lambda_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Lambda function to filter cloud connectors by.</p>
    pub fn get_lambda_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_lambda_arn()
    }
    /// <p>The maximum number of results to return at one time.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return at one time.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to return at one time.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>A token that can be used to retrieve the next set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>A token that can be used to retrieve the next set of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>A token that can be used to retrieve the next set of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
