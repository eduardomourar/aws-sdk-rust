// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_query_results::_get_query_results_output::GetQueryResultsOutputBuilder;

pub use crate::operation::get_query_results::_get_query_results_input::GetQueryResultsInputBuilder;

impl crate::operation::get_query_results::builders::GetQueryResultsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_query_results::GetQueryResultsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_query_results::GetQueryResultsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_query_results();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetQueryResults`.
///
/// <p>Gets event data results of a query. You must specify the <code>QueryID</code> value returned by the <code>StartQuery</code> operation.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetQueryResultsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_query_results::builders::GetQueryResultsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_query_results::GetQueryResultsOutput,
        crate::operation::get_query_results::GetQueryResultsError,
    > for GetQueryResultsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_query_results::GetQueryResultsOutput,
            crate::operation::get_query_results::GetQueryResultsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetQueryResultsFluentBuilder {
    /// Creates a new `GetQueryResultsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetQueryResults as a reference.
    pub fn as_input(&self) -> &crate::operation::get_query_results::builders::GetQueryResultsInputBuilder {
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
        crate::operation::get_query_results::GetQueryResultsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_query_results::GetQueryResultsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_query_results::GetQueryResults::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_query_results::GetQueryResults::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_query_results::GetQueryResultsOutput,
        crate::operation::get_query_results::GetQueryResultsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::get_query_results::paginator::GetQueryResultsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::get_query_results::paginator::GetQueryResultsPaginator {
        crate::operation::get_query_results::paginator::GetQueryResultsPaginator::new(self.handle, self.inner)
    }
    /// <p>The ARN (or ID suffix of the ARN) of the event data store against which the query was run.</p>
    #[deprecated(note = "EventDataStore is no longer required by GetQueryResultsRequest")]
    pub fn event_data_store(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.event_data_store(input.into());
        self
    }
    /// <p>The ARN (or ID suffix of the ARN) of the event data store against which the query was run.</p>
    #[deprecated(note = "EventDataStore is no longer required by GetQueryResultsRequest")]
    pub fn set_event_data_store(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_event_data_store(input);
        self
    }
    /// <p>The ARN (or ID suffix of the ARN) of the event data store against which the query was run.</p>
    #[deprecated(note = "EventDataStore is no longer required by GetQueryResultsRequest")]
    pub fn get_event_data_store(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_event_data_store()
    }
    /// <p>The ID of the query for which you want to get results.</p>
    pub fn query_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.query_id(input.into());
        self
    }
    /// <p>The ID of the query for which you want to get results.</p>
    pub fn set_query_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_query_id(input);
        self
    }
    /// <p>The ID of the query for which you want to get results.</p>
    pub fn get_query_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_query_id()
    }
    /// <p>A token you can use to get the next page of query results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>A token you can use to get the next page of query results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>A token you can use to get the next page of query results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of query results to display on a single page.</p>
    pub fn max_query_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_query_results(input);
        self
    }
    /// <p>The maximum number of query results to display on a single page.</p>
    pub fn set_max_query_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_query_results(input);
        self
    }
    /// <p>The maximum number of query results to display on a single page.</p>
    pub fn get_max_query_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_query_results()
    }
    /// <p>The account ID of the event data store owner.</p>
    pub fn event_data_store_owner_account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.event_data_store_owner_account_id(input.into());
        self
    }
    /// <p>The account ID of the event data store owner.</p>
    pub fn set_event_data_store_owner_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_event_data_store_owner_account_id(input);
        self
    }
    /// <p>The account ID of the event data store owner.</p>
    pub fn get_event_data_store_owner_account_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_event_data_store_owner_account_id()
    }
}
