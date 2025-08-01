// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_distributions_by_connection_mode::_list_distributions_by_connection_mode_output::ListDistributionsByConnectionModeOutputBuilder;

pub use crate::operation::list_distributions_by_connection_mode::_list_distributions_by_connection_mode_input::ListDistributionsByConnectionModeInputBuilder;

impl crate::operation::list_distributions_by_connection_mode::builders::ListDistributionsByConnectionModeInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_distributions_by_connection_mode::ListDistributionsByConnectionModeOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_distributions_by_connection_mode::ListDistributionsByConnectionModeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_distributions_by_connection_mode();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListDistributionsByConnectionMode`.
///
/// <p>Lists the distributions by the connection mode that you specify.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListDistributionsByConnectionModeFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_distributions_by_connection_mode::builders::ListDistributionsByConnectionModeInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_distributions_by_connection_mode::ListDistributionsByConnectionModeOutput,
        crate::operation::list_distributions_by_connection_mode::ListDistributionsByConnectionModeError,
    > for ListDistributionsByConnectionModeFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_distributions_by_connection_mode::ListDistributionsByConnectionModeOutput,
            crate::operation::list_distributions_by_connection_mode::ListDistributionsByConnectionModeError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListDistributionsByConnectionModeFluentBuilder {
    /// Creates a new `ListDistributionsByConnectionModeFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListDistributionsByConnectionMode as a reference.
    pub fn as_input(&self) -> &crate::operation::list_distributions_by_connection_mode::builders::ListDistributionsByConnectionModeInputBuilder {
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
        crate::operation::list_distributions_by_connection_mode::ListDistributionsByConnectionModeOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_distributions_by_connection_mode::ListDistributionsByConnectionModeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_distributions_by_connection_mode::ListDistributionsByConnectionMode::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_distributions_by_connection_mode::ListDistributionsByConnectionMode::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_distributions_by_connection_mode::ListDistributionsByConnectionModeOutput,
        crate::operation::list_distributions_by_connection_mode::ListDistributionsByConnectionModeError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_distributions_by_connection_mode::paginator::ListDistributionsByConnectionModePaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_distributions_by_connection_mode::paginator::ListDistributionsByConnectionModePaginator {
        crate::operation::list_distributions_by_connection_mode::paginator::ListDistributionsByConnectionModePaginator::new(self.handle, self.inner)
    }
    /// <p>The marker for the next set of distributions to retrieve.</p>
    pub fn marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.marker(input.into());
        self
    }
    /// <p>The marker for the next set of distributions to retrieve.</p>
    pub fn set_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_marker(input);
        self
    }
    /// <p>The marker for the next set of distributions to retrieve.</p>
    pub fn get_marker(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_marker()
    }
    /// <p>The maximum number of distributions to return.</p>
    pub fn max_items(mut self, input: i32) -> Self {
        self.inner = self.inner.max_items(input);
        self
    }
    /// <p>The maximum number of distributions to return.</p>
    pub fn set_max_items(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_items(input);
        self
    }
    /// <p>The maximum number of distributions to return.</p>
    pub fn get_max_items(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_items()
    }
    /// <p>This field specifies whether the connection mode is through a standard distribution (direct) or a multi-tenant distribution with distribution tenants (tenant-only).</p>
    pub fn connection_mode(mut self, input: crate::types::ConnectionMode) -> Self {
        self.inner = self.inner.connection_mode(input);
        self
    }
    /// <p>This field specifies whether the connection mode is through a standard distribution (direct) or a multi-tenant distribution with distribution tenants (tenant-only).</p>
    pub fn set_connection_mode(mut self, input: ::std::option::Option<crate::types::ConnectionMode>) -> Self {
        self.inner = self.inner.set_connection_mode(input);
        self
    }
    /// <p>This field specifies whether the connection mode is through a standard distribution (direct) or a multi-tenant distribution with distribution tenants (tenant-only).</p>
    pub fn get_connection_mode(&self) -> &::std::option::Option<crate::types::ConnectionMode> {
        self.inner.get_connection_mode()
    }
}
