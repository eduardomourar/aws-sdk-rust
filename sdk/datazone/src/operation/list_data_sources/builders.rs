// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_data_sources::_list_data_sources_output::ListDataSourcesOutputBuilder;

pub use crate::operation::list_data_sources::_list_data_sources_input::ListDataSourcesInputBuilder;

impl crate::operation::list_data_sources::builders::ListDataSourcesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_data_sources::ListDataSourcesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_data_sources::ListDataSourcesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_data_sources();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListDataSources`.
///
/// <p>Lists data sources in Amazon DataZone.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListDataSourcesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_data_sources::builders::ListDataSourcesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_data_sources::ListDataSourcesOutput,
        crate::operation::list_data_sources::ListDataSourcesError,
    > for ListDataSourcesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_data_sources::ListDataSourcesOutput,
            crate::operation::list_data_sources::ListDataSourcesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListDataSourcesFluentBuilder {
    /// Creates a new `ListDataSourcesFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListDataSources as a reference.
    pub fn as_input(&self) -> &crate::operation::list_data_sources::builders::ListDataSourcesInputBuilder {
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
        crate::operation::list_data_sources::ListDataSourcesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_data_sources::ListDataSourcesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_data_sources::ListDataSources::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_data_sources::ListDataSources::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_data_sources::ListDataSourcesOutput,
        crate::operation::list_data_sources::ListDataSourcesError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_data_sources::paginator::ListDataSourcesPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_data_sources::paginator::ListDataSourcesPaginator {
        crate::operation::list_data_sources::paginator::ListDataSourcesPaginator::new(self.handle, self.inner)
    }
    /// <p>The identifier of the Amazon DataZone domain in which to list the data sources.</p>
    pub fn domain_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_identifier(input.into());
        self
    }
    /// <p>The identifier of the Amazon DataZone domain in which to list the data sources.</p>
    pub fn set_domain_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_identifier(input);
        self
    }
    /// <p>The identifier of the Amazon DataZone domain in which to list the data sources.</p>
    pub fn get_domain_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_domain_identifier()
    }
    /// <p>The identifier of the project in which to list data sources.</p>
    pub fn project_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.project_identifier(input.into());
        self
    }
    /// <p>The identifier of the project in which to list data sources.</p>
    pub fn set_project_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_project_identifier(input);
        self
    }
    /// <p>The identifier of the project in which to list data sources.</p>
    pub fn get_project_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_project_identifier()
    }
    /// <p>The identifier of the environment in which to list the data sources.</p>
    pub fn environment_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.environment_identifier(input.into());
        self
    }
    /// <p>The identifier of the environment in which to list the data sources.</p>
    pub fn set_environment_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_environment_identifier(input);
        self
    }
    /// <p>The identifier of the environment in which to list the data sources.</p>
    pub fn get_environment_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_environment_identifier()
    }
    /// <p>The ID of the connection.</p>
    pub fn connection_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.connection_identifier(input.into());
        self
    }
    /// <p>The ID of the connection.</p>
    pub fn set_connection_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_connection_identifier(input);
        self
    }
    /// <p>The ID of the connection.</p>
    pub fn get_connection_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_connection_identifier()
    }
    /// <p>The type of the data source.</p>
    pub fn r#type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.r#type(input.into());
        self
    }
    /// <p>The type of the data source.</p>
    pub fn set_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_type(input);
        self
    }
    /// <p>The type of the data source.</p>
    pub fn get_type(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_type()
    }
    /// <p>The status of the data source.</p>
    pub fn status(mut self, input: crate::types::DataSourceStatus) -> Self {
        self.inner = self.inner.status(input);
        self
    }
    /// <p>The status of the data source.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::DataSourceStatus>) -> Self {
        self.inner = self.inner.set_status(input);
        self
    }
    /// <p>The status of the data source.</p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::DataSourceStatus> {
        self.inner.get_status()
    }
    /// <p>The name of the data source.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the data source.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the data source.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>When the number of data sources is greater than the default value for the <code>MaxResults</code> parameter, or if you explicitly specify a value for <code>MaxResults</code> that is less than the number of data sources, the response includes a pagination token named <code>NextToken</code>. You can specify this <code>NextToken</code> value in a subsequent call to <code>ListDataSources</code> to list the next set of data sources.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>When the number of data sources is greater than the default value for the <code>MaxResults</code> parameter, or if you explicitly specify a value for <code>MaxResults</code> that is less than the number of data sources, the response includes a pagination token named <code>NextToken</code>. You can specify this <code>NextToken</code> value in a subsequent call to <code>ListDataSources</code> to list the next set of data sources.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>When the number of data sources is greater than the default value for the <code>MaxResults</code> parameter, or if you explicitly specify a value for <code>MaxResults</code> that is less than the number of data sources, the response includes a pagination token named <code>NextToken</code>. You can specify this <code>NextToken</code> value in a subsequent call to <code>ListDataSources</code> to list the next set of data sources.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of data sources to return in a single call to <code>ListDataSources</code>. When the number of data sources to be listed is greater than the value of <code>MaxResults</code>, the response contains a <code>NextToken</code> value that you can use in a subsequent call to <code>ListDataSources</code> to list the next set of data sources.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of data sources to return in a single call to <code>ListDataSources</code>. When the number of data sources to be listed is greater than the value of <code>MaxResults</code>, the response contains a <code>NextToken</code> value that you can use in a subsequent call to <code>ListDataSources</code> to list the next set of data sources.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of data sources to return in a single call to <code>ListDataSources</code>. When the number of data sources to be listed is greater than the value of <code>MaxResults</code>, the response contains a <code>NextToken</code> value that you can use in a subsequent call to <code>ListDataSources</code> to list the next set of data sources.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
}
