// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_configuration_history::_list_configuration_history_output::ListConfigurationHistoryOutputBuilder;

pub use crate::operation::list_configuration_history::_list_configuration_history_input::ListConfigurationHistoryInputBuilder;

impl crate::operation::list_configuration_history::builders::ListConfigurationHistoryInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_configuration_history::ListConfigurationHistoryOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_configuration_history::ListConfigurationHistoryError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_configuration_history();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListConfigurationHistory`.
///
/// <p>Lists the INFO, WARN, and ERROR events for periodic configuration updates performed by Application Insights. Examples of events represented are:</p>
/// <ul>
/// <li>
/// <p>INFO: creating a new alarm or updating an alarm threshold.</p></li>
/// <li>
/// <p>WARN: alarm not created due to insufficient data points used to predict thresholds.</p></li>
/// <li>
/// <p>ERROR: alarm not created due to permission errors or exceeding quotas.</p></li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListConfigurationHistoryFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_configuration_history::builders::ListConfigurationHistoryInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_configuration_history::ListConfigurationHistoryOutput,
        crate::operation::list_configuration_history::ListConfigurationHistoryError,
    > for ListConfigurationHistoryFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_configuration_history::ListConfigurationHistoryOutput,
            crate::operation::list_configuration_history::ListConfigurationHistoryError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListConfigurationHistoryFluentBuilder {
    /// Creates a new `ListConfigurationHistoryFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListConfigurationHistory as a reference.
    pub fn as_input(&self) -> &crate::operation::list_configuration_history::builders::ListConfigurationHistoryInputBuilder {
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
        crate::operation::list_configuration_history::ListConfigurationHistoryOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_configuration_history::ListConfigurationHistoryError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_configuration_history::ListConfigurationHistory::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_configuration_history::ListConfigurationHistory::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_configuration_history::ListConfigurationHistoryOutput,
        crate::operation::list_configuration_history::ListConfigurationHistoryError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_configuration_history::paginator::ListConfigurationHistoryPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_configuration_history::paginator::ListConfigurationHistoryPaginator {
        crate::operation::list_configuration_history::paginator::ListConfigurationHistoryPaginator::new(self.handle, self.inner)
    }
    /// <p>Resource group to which the application belongs.</p>
    pub fn resource_group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_group_name(input.into());
        self
    }
    /// <p>Resource group to which the application belongs.</p>
    pub fn set_resource_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_group_name(input);
        self
    }
    /// <p>Resource group to which the application belongs.</p>
    pub fn get_resource_group_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resource_group_name()
    }
    /// <p>The start time of the event.</p>
    pub fn start_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.start_time(input);
        self
    }
    /// <p>The start time of the event.</p>
    pub fn set_start_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_start_time(input);
        self
    }
    /// <p>The start time of the event.</p>
    pub fn get_start_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_start_time()
    }
    /// <p>The end time of the event.</p>
    pub fn end_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.end_time(input);
        self
    }
    /// <p>The end time of the event.</p>
    pub fn set_end_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_end_time(input);
        self
    }
    /// <p>The end time of the event.</p>
    pub fn get_end_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_end_time()
    }
    /// <p>The status of the configuration update event. Possible values include INFO, WARN, and ERROR.</p>
    pub fn event_status(mut self, input: crate::types::ConfigurationEventStatus) -> Self {
        self.inner = self.inner.event_status(input);
        self
    }
    /// <p>The status of the configuration update event. Possible values include INFO, WARN, and ERROR.</p>
    pub fn set_event_status(mut self, input: ::std::option::Option<crate::types::ConfigurationEventStatus>) -> Self {
        self.inner = self.inner.set_event_status(input);
        self
    }
    /// <p>The status of the configuration update event. Possible values include INFO, WARN, and ERROR.</p>
    pub fn get_event_status(&self) -> &::std::option::Option<crate::types::ConfigurationEventStatus> {
        self.inner.get_event_status()
    }
    /// <p>The maximum number of results returned by <code>ListConfigurationHistory</code> in paginated output. When this parameter is used, <code>ListConfigurationHistory</code> returns only <code>MaxResults</code> in a single page along with a <code>NextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListConfigurationHistory</code> request with the returned <code>NextToken</code> value. If this parameter is not used, then <code>ListConfigurationHistory</code> returns all results.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results returned by <code>ListConfigurationHistory</code> in paginated output. When this parameter is used, <code>ListConfigurationHistory</code> returns only <code>MaxResults</code> in a single page along with a <code>NextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListConfigurationHistory</code> request with the returned <code>NextToken</code> value. If this parameter is not used, then <code>ListConfigurationHistory</code> returns all results.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results returned by <code>ListConfigurationHistory</code> in paginated output. When this parameter is used, <code>ListConfigurationHistory</code> returns only <code>MaxResults</code> in a single page along with a <code>NextToken</code> response element. The remaining results of the initial request can be seen by sending another <code>ListConfigurationHistory</code> request with the returned <code>NextToken</code> value. If this parameter is not used, then <code>ListConfigurationHistory</code> returns all results.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>The <code>NextToken</code> value returned from a previous paginated <code>ListConfigurationHistory</code> request where <code>MaxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>NextToken</code> value. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The <code>NextToken</code> value returned from a previous paginated <code>ListConfigurationHistory</code> request where <code>MaxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>NextToken</code> value. This value is <code>null</code> when there are no more results to return.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The <code>NextToken</code> value returned from a previous paginated <code>ListConfigurationHistory</code> request where <code>MaxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>NextToken</code> value. This value is <code>null</code> when there are no more results to return.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The Amazon Web Services account ID for the resource group owner.</p>
    pub fn account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.account_id(input.into());
        self
    }
    /// <p>The Amazon Web Services account ID for the resource group owner.</p>
    pub fn set_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_account_id(input);
        self
    }
    /// <p>The Amazon Web Services account ID for the resource group owner.</p>
    pub fn get_account_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_account_id()
    }
}
