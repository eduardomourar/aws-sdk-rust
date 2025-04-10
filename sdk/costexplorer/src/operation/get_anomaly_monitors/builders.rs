// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_anomaly_monitors::_get_anomaly_monitors_output::GetAnomalyMonitorsOutputBuilder;

pub use crate::operation::get_anomaly_monitors::_get_anomaly_monitors_input::GetAnomalyMonitorsInputBuilder;

impl crate::operation::get_anomaly_monitors::builders::GetAnomalyMonitorsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_anomaly_monitors::GetAnomalyMonitorsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_anomaly_monitors::GetAnomalyMonitorsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_anomaly_monitors();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetAnomalyMonitors`.
///
/// <p>Retrieves the cost anomaly monitor definitions for your account. You can filter using a list of cost anomaly monitor Amazon Resource Names (ARNs).</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetAnomalyMonitorsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_anomaly_monitors::builders::GetAnomalyMonitorsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_anomaly_monitors::GetAnomalyMonitorsOutput,
        crate::operation::get_anomaly_monitors::GetAnomalyMonitorsError,
    > for GetAnomalyMonitorsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_anomaly_monitors::GetAnomalyMonitorsOutput,
            crate::operation::get_anomaly_monitors::GetAnomalyMonitorsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetAnomalyMonitorsFluentBuilder {
    /// Creates a new `GetAnomalyMonitorsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetAnomalyMonitors as a reference.
    pub fn as_input(&self) -> &crate::operation::get_anomaly_monitors::builders::GetAnomalyMonitorsInputBuilder {
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
        crate::operation::get_anomaly_monitors::GetAnomalyMonitorsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_anomaly_monitors::GetAnomalyMonitorsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_anomaly_monitors::GetAnomalyMonitors::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_anomaly_monitors::GetAnomalyMonitors::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_anomaly_monitors::GetAnomalyMonitorsOutput,
        crate::operation::get_anomaly_monitors::GetAnomalyMonitorsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::get_anomaly_monitors::paginator::GetAnomalyMonitorsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::get_anomaly_monitors::paginator::GetAnomalyMonitorsPaginator {
        crate::operation::get_anomaly_monitors::paginator::GetAnomalyMonitorsPaginator::new(self.handle, self.inner)
    }
    ///
    /// Appends an item to `MonitorArnList`.
    ///
    /// To override the contents of this collection use [`set_monitor_arn_list`](Self::set_monitor_arn_list).
    ///
    /// <p>A list of cost anomaly monitor ARNs.</p>
    pub fn monitor_arn_list(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.monitor_arn_list(input.into());
        self
    }
    /// <p>A list of cost anomaly monitor ARNs.</p>
    pub fn set_monitor_arn_list(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_monitor_arn_list(input);
        self
    }
    /// <p>A list of cost anomaly monitor ARNs.</p>
    pub fn get_monitor_arn_list(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_monitor_arn_list()
    }
    /// <p>The token to retrieve the next set of results. Amazon Web Services provides the token when the response from a previous call has more results than the maximum page size.</p>
    pub fn next_page_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_page_token(input.into());
        self
    }
    /// <p>The token to retrieve the next set of results. Amazon Web Services provides the token when the response from a previous call has more results than the maximum page size.</p>
    pub fn set_next_page_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_page_token(input);
        self
    }
    /// <p>The token to retrieve the next set of results. Amazon Web Services provides the token when the response from a previous call has more results than the maximum page size.</p>
    pub fn get_next_page_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_page_token()
    }
    /// <p>The number of entries that a paginated response contains.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The number of entries that a paginated response contains.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The number of entries that a paginated response contains.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
}
