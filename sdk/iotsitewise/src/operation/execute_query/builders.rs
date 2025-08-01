// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::execute_query::_execute_query_output::ExecuteQueryOutputBuilder;

pub use crate::operation::execute_query::_execute_query_input::ExecuteQueryInputBuilder;

impl crate::operation::execute_query::builders::ExecuteQueryInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::execute_query::ExecuteQueryOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::execute_query::ExecuteQueryError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.execute_query();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ExecuteQuery`.
///
/// <p>Run SQL queries to retrieve metadata and time-series data from asset models, assets, measurements, metrics, transforms, and aggregates.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ExecuteQueryFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::execute_query::builders::ExecuteQueryInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::execute_query::ExecuteQueryOutput,
        crate::operation::execute_query::ExecuteQueryError,
    > for ExecuteQueryFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::execute_query::ExecuteQueryOutput,
            crate::operation::execute_query::ExecuteQueryError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ExecuteQueryFluentBuilder {
    /// Creates a new `ExecuteQueryFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ExecuteQuery as a reference.
    pub fn as_input(&self) -> &crate::operation::execute_query::builders::ExecuteQueryInputBuilder {
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
        crate::operation::execute_query::ExecuteQueryOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::execute_query::ExecuteQueryError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::execute_query::ExecuteQuery::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::execute_query::ExecuteQuery::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::execute_query::ExecuteQueryOutput,
        crate::operation::execute_query::ExecuteQueryError,
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
    /// <p>The IoT SiteWise query statement.</p>
    pub fn query_statement(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.query_statement(input.into());
        self
    }
    /// <p>The IoT SiteWise query statement.</p>
    pub fn set_query_statement(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_query_statement(input);
        self
    }
    /// <p>The IoT SiteWise query statement.</p>
    pub fn get_query_statement(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_query_statement()
    }
    /// <p>The string that specifies the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The string that specifies the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The string that specifies the next page of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of results to return at one time.</p>
    /// <ul>
    /// <li>
    /// <p>Minimum is 1</p></li>
    /// <li>
    /// <p>Maximum is 20000</p></li>
    /// <li>
    /// <p>Default is 250</p></li>
    /// </ul>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return at one time.</p>
    /// <ul>
    /// <li>
    /// <p>Minimum is 1</p></li>
    /// <li>
    /// <p>Maximum is 20000</p></li>
    /// <li>
    /// <p>Default is 250</p></li>
    /// </ul>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to return at one time.</p>
    /// <ul>
    /// <li>
    /// <p>Minimum is 1</p></li>
    /// <li>
    /// <p>Maximum is 20000</p></li>
    /// <li>
    /// <p>Default is 250</p></li>
    /// </ul>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. Don't reuse this client token if a new idempotent request is required.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. Don't reuse this client token if a new idempotent request is required.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. Don't reuse this client token if a new idempotent request is required.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
}
