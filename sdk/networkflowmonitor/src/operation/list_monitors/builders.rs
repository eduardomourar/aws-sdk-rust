// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_monitors::_list_monitors_output::ListMonitorsOutputBuilder;

pub use crate::operation::list_monitors::_list_monitors_input::ListMonitorsInputBuilder;

impl crate::operation::list_monitors::builders::ListMonitorsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_monitors::ListMonitorsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_monitors::ListMonitorsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_monitors();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListMonitors`.
///
/// <p>List all monitors in an account. Optionally, you can list only monitors that have a specific status, by using the <code>STATUS</code> parameter.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListMonitorsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_monitors::builders::ListMonitorsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_monitors::ListMonitorsOutput,
        crate::operation::list_monitors::ListMonitorsError,
    > for ListMonitorsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_monitors::ListMonitorsOutput,
            crate::operation::list_monitors::ListMonitorsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListMonitorsFluentBuilder {
    /// Creates a new `ListMonitorsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListMonitors as a reference.
    pub fn as_input(&self) -> &crate::operation::list_monitors::builders::ListMonitorsInputBuilder {
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
        crate::operation::list_monitors::ListMonitorsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_monitors::ListMonitorsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_monitors::ListMonitors::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_monitors::ListMonitors::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_monitors::ListMonitorsOutput,
        crate::operation::list_monitors::ListMonitorsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_monitors::paginator::ListMonitorsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_monitors::paginator::ListMonitorsPaginator {
        crate::operation::list_monitors::paginator::ListMonitorsPaginator::new(self.handle, self.inner)
    }
    /// <p>The token for the next set of results. You receive this token from a previous call.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token for the next set of results. You receive this token from a previous call.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token for the next set of results. You receive this token from a previous call.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The number of query results that you want to return with this call.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The number of query results that you want to return with this call.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The number of query results that you want to return with this call.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>The status of a monitor. The status can be one of the following</p>
    /// <ul>
    /// <li>
    /// <p><code>PENDING</code>: The monitor is in the process of being created.</p></li>
    /// <li>
    /// <p><code>ACTIVE</code>: The monitor is active.</p></li>
    /// <li>
    /// <p><code>INACTIVE</code>: The monitor is inactive.</p></li>
    /// <li>
    /// <p><code>ERROR</code>: Monitor creation failed due to an error.</p></li>
    /// <li>
    /// <p><code>DELETING</code>: The monitor is in the process of being deleted.</p></li>
    /// </ul>
    pub fn monitor_status(mut self, input: crate::types::MonitorStatus) -> Self {
        self.inner = self.inner.monitor_status(input);
        self
    }
    /// <p>The status of a monitor. The status can be one of the following</p>
    /// <ul>
    /// <li>
    /// <p><code>PENDING</code>: The monitor is in the process of being created.</p></li>
    /// <li>
    /// <p><code>ACTIVE</code>: The monitor is active.</p></li>
    /// <li>
    /// <p><code>INACTIVE</code>: The monitor is inactive.</p></li>
    /// <li>
    /// <p><code>ERROR</code>: Monitor creation failed due to an error.</p></li>
    /// <li>
    /// <p><code>DELETING</code>: The monitor is in the process of being deleted.</p></li>
    /// </ul>
    pub fn set_monitor_status(mut self, input: ::std::option::Option<crate::types::MonitorStatus>) -> Self {
        self.inner = self.inner.set_monitor_status(input);
        self
    }
    /// <p>The status of a monitor. The status can be one of the following</p>
    /// <ul>
    /// <li>
    /// <p><code>PENDING</code>: The monitor is in the process of being created.</p></li>
    /// <li>
    /// <p><code>ACTIVE</code>: The monitor is active.</p></li>
    /// <li>
    /// <p><code>INACTIVE</code>: The monitor is inactive.</p></li>
    /// <li>
    /// <p><code>ERROR</code>: Monitor creation failed due to an error.</p></li>
    /// <li>
    /// <p><code>DELETING</code>: The monitor is in the process of being deleted.</p></li>
    /// </ul>
    pub fn get_monitor_status(&self) -> &::std::option::Option<crate::types::MonitorStatus> {
        self.inner.get_monitor_status()
    }
}
