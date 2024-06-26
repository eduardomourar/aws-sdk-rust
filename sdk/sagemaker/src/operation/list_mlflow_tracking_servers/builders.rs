// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_mlflow_tracking_servers::_list_mlflow_tracking_servers_output::ListMlflowTrackingServersOutputBuilder;

pub use crate::operation::list_mlflow_tracking_servers::_list_mlflow_tracking_servers_input::ListMlflowTrackingServersInputBuilder;

impl crate::operation::list_mlflow_tracking_servers::builders::ListMlflowTrackingServersInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_mlflow_tracking_servers::ListMlflowTrackingServersOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_mlflow_tracking_servers::ListMlflowTrackingServersError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_mlflow_tracking_servers();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListMlflowTrackingServers`.
///
/// <p>Lists all MLflow Tracking Servers.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListMlflowTrackingServersFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_mlflow_tracking_servers::builders::ListMlflowTrackingServersInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_mlflow_tracking_servers::ListMlflowTrackingServersOutput,
        crate::operation::list_mlflow_tracking_servers::ListMlflowTrackingServersError,
    > for ListMlflowTrackingServersFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_mlflow_tracking_servers::ListMlflowTrackingServersOutput,
            crate::operation::list_mlflow_tracking_servers::ListMlflowTrackingServersError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListMlflowTrackingServersFluentBuilder {
    /// Creates a new `ListMlflowTrackingServersFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListMlflowTrackingServers as a reference.
    pub fn as_input(&self) -> &crate::operation::list_mlflow_tracking_servers::builders::ListMlflowTrackingServersInputBuilder {
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
        crate::operation::list_mlflow_tracking_servers::ListMlflowTrackingServersOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_mlflow_tracking_servers::ListMlflowTrackingServersError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_mlflow_tracking_servers::ListMlflowTrackingServers::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_mlflow_tracking_servers::ListMlflowTrackingServers::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_mlflow_tracking_servers::ListMlflowTrackingServersOutput,
        crate::operation::list_mlflow_tracking_servers::ListMlflowTrackingServersError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_mlflow_tracking_servers::paginator::ListMlflowTrackingServersPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_mlflow_tracking_servers::paginator::ListMlflowTrackingServersPaginator {
        crate::operation::list_mlflow_tracking_servers::paginator::ListMlflowTrackingServersPaginator::new(self.handle, self.inner)
    }
    /// <p>Use the <code>CreatedAfter</code> filter to only list tracking servers created after a specific date and time. Listed tracking servers are shown with a date and time such as <code>"2024-03-16T01:46:56+00:00"</code>. The <code>CreatedAfter</code> parameter takes in a Unix timestamp. To convert a date and time into a Unix timestamp, see <a href="https://www.epochconverter.com/">EpochConverter</a>.</p>
    pub fn created_after(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.created_after(input);
        self
    }
    /// <p>Use the <code>CreatedAfter</code> filter to only list tracking servers created after a specific date and time. Listed tracking servers are shown with a date and time such as <code>"2024-03-16T01:46:56+00:00"</code>. The <code>CreatedAfter</code> parameter takes in a Unix timestamp. To convert a date and time into a Unix timestamp, see <a href="https://www.epochconverter.com/">EpochConverter</a>.</p>
    pub fn set_created_after(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_created_after(input);
        self
    }
    /// <p>Use the <code>CreatedAfter</code> filter to only list tracking servers created after a specific date and time. Listed tracking servers are shown with a date and time such as <code>"2024-03-16T01:46:56+00:00"</code>. The <code>CreatedAfter</code> parameter takes in a Unix timestamp. To convert a date and time into a Unix timestamp, see <a href="https://www.epochconverter.com/">EpochConverter</a>.</p>
    pub fn get_created_after(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_created_after()
    }
    /// <p>Use the <code>CreatedBefore</code> filter to only list tracking servers created before a specific date and time. Listed tracking servers are shown with a date and time such as <code>"2024-03-16T01:46:56+00:00"</code>. The <code>CreatedBefore</code> parameter takes in a Unix timestamp. To convert a date and time into a Unix timestamp, see <a href="https://www.epochconverter.com/">EpochConverter</a>.</p>
    pub fn created_before(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.created_before(input);
        self
    }
    /// <p>Use the <code>CreatedBefore</code> filter to only list tracking servers created before a specific date and time. Listed tracking servers are shown with a date and time such as <code>"2024-03-16T01:46:56+00:00"</code>. The <code>CreatedBefore</code> parameter takes in a Unix timestamp. To convert a date and time into a Unix timestamp, see <a href="https://www.epochconverter.com/">EpochConverter</a>.</p>
    pub fn set_created_before(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_created_before(input);
        self
    }
    /// <p>Use the <code>CreatedBefore</code> filter to only list tracking servers created before a specific date and time. Listed tracking servers are shown with a date and time such as <code>"2024-03-16T01:46:56+00:00"</code>. The <code>CreatedBefore</code> parameter takes in a Unix timestamp. To convert a date and time into a Unix timestamp, see <a href="https://www.epochconverter.com/">EpochConverter</a>.</p>
    pub fn get_created_before(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_created_before()
    }
    /// <p>Filter for tracking servers with a specified creation status.</p>
    pub fn tracking_server_status(mut self, input: crate::types::TrackingServerStatus) -> Self {
        self.inner = self.inner.tracking_server_status(input);
        self
    }
    /// <p>Filter for tracking servers with a specified creation status.</p>
    pub fn set_tracking_server_status(mut self, input: ::std::option::Option<crate::types::TrackingServerStatus>) -> Self {
        self.inner = self.inner.set_tracking_server_status(input);
        self
    }
    /// <p>Filter for tracking servers with a specified creation status.</p>
    pub fn get_tracking_server_status(&self) -> &::std::option::Option<crate::types::TrackingServerStatus> {
        self.inner.get_tracking_server_status()
    }
    /// <p>Filter for tracking servers using the specified MLflow version.</p>
    pub fn mlflow_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.mlflow_version(input.into());
        self
    }
    /// <p>Filter for tracking servers using the specified MLflow version.</p>
    pub fn set_mlflow_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_mlflow_version(input);
        self
    }
    /// <p>Filter for tracking servers using the specified MLflow version.</p>
    pub fn get_mlflow_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_mlflow_version()
    }
    /// <p>Filter for trackings servers sorting by name, creation time, or creation status.</p>
    pub fn sort_by(mut self, input: crate::types::SortTrackingServerBy) -> Self {
        self.inner = self.inner.sort_by(input);
        self
    }
    /// <p>Filter for trackings servers sorting by name, creation time, or creation status.</p>
    pub fn set_sort_by(mut self, input: ::std::option::Option<crate::types::SortTrackingServerBy>) -> Self {
        self.inner = self.inner.set_sort_by(input);
        self
    }
    /// <p>Filter for trackings servers sorting by name, creation time, or creation status.</p>
    pub fn get_sort_by(&self) -> &::std::option::Option<crate::types::SortTrackingServerBy> {
        self.inner.get_sort_by()
    }
    /// <p>Change the order of the listed tracking servers. By default, tracking servers are listed in <code>Descending</code> order by creation time. To change the list order, you can specify <code>SortOrder</code> to be <code>Ascending</code>.</p>
    pub fn sort_order(mut self, input: crate::types::SortOrder) -> Self {
        self.inner = self.inner.sort_order(input);
        self
    }
    /// <p>Change the order of the listed tracking servers. By default, tracking servers are listed in <code>Descending</code> order by creation time. To change the list order, you can specify <code>SortOrder</code> to be <code>Ascending</code>.</p>
    pub fn set_sort_order(mut self, input: ::std::option::Option<crate::types::SortOrder>) -> Self {
        self.inner = self.inner.set_sort_order(input);
        self
    }
    /// <p>Change the order of the listed tracking servers. By default, tracking servers are listed in <code>Descending</code> order by creation time. To change the list order, you can specify <code>SortOrder</code> to be <code>Ascending</code>.</p>
    pub fn get_sort_order(&self) -> &::std::option::Option<crate::types::SortOrder> {
        self.inner.get_sort_order()
    }
    /// <p>If the previous response was truncated, you will receive this token. Use it in your next request to receive the next set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>If the previous response was truncated, you will receive this token. Use it in your next request to receive the next set of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>If the previous response was truncated, you will receive this token. Use it in your next request to receive the next set of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of tracking servers to list.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of tracking servers to list.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of tracking servers to list.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
}
