// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_log_patterns::_list_log_patterns_output::ListLogPatternsOutputBuilder;

pub use crate::operation::list_log_patterns::_list_log_patterns_input::ListLogPatternsInputBuilder;

impl crate::operation::list_log_patterns::builders::ListLogPatternsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_log_patterns::ListLogPatternsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_log_patterns::ListLogPatternsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_log_patterns();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListLogPatterns`.
///
/// <p>Lists the log patterns in the specific log <code>LogPatternSet</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListLogPatternsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_log_patterns::builders::ListLogPatternsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_log_patterns::ListLogPatternsOutput,
        crate::operation::list_log_patterns::ListLogPatternsError,
    > for ListLogPatternsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_log_patterns::ListLogPatternsOutput,
            crate::operation::list_log_patterns::ListLogPatternsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListLogPatternsFluentBuilder {
    /// Creates a new `ListLogPatternsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListLogPatterns as a reference.
    pub fn as_input(&self) -> &crate::operation::list_log_patterns::builders::ListLogPatternsInputBuilder {
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
        crate::operation::list_log_patterns::ListLogPatternsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_log_patterns::ListLogPatternsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_log_patterns::ListLogPatterns::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_log_patterns::ListLogPatterns::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_log_patterns::ListLogPatternsOutput,
        crate::operation::list_log_patterns::ListLogPatternsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_log_patterns::paginator::ListLogPatternsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_log_patterns::paginator::ListLogPatternsPaginator {
        crate::operation::list_log_patterns::paginator::ListLogPatternsPaginator::new(self.handle, self.inner)
    }
    /// <p>The name of the resource group.</p>
    pub fn resource_group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_group_name(input.into());
        self
    }
    /// <p>The name of the resource group.</p>
    pub fn set_resource_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_group_name(input);
        self
    }
    /// <p>The name of the resource group.</p>
    pub fn get_resource_group_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resource_group_name()
    }
    /// <p>The name of the log pattern set.</p>
    pub fn pattern_set_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.pattern_set_name(input.into());
        self
    }
    /// <p>The name of the log pattern set.</p>
    pub fn set_pattern_set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_pattern_set_name(input);
        self
    }
    /// <p>The name of the log pattern set.</p>
    pub fn get_pattern_set_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_pattern_set_name()
    }
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>The token to request the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token to request the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token to request the next page of results.</p>
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
