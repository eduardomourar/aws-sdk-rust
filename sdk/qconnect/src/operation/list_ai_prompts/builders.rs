// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_ai_prompts::_list_ai_prompts_output::ListAiPromptsOutputBuilder;

pub use crate::operation::list_ai_prompts::_list_ai_prompts_input::ListAiPromptsInputBuilder;

impl crate::operation::list_ai_prompts::builders::ListAiPromptsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_ai_prompts::ListAiPromptsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_ai_prompts::ListAIPromptsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_ai_prompts();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListAIPrompts`.
///
/// <p>Lists the AI Prompts available on the Amazon Q in Connect assistant.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListAIPromptsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_ai_prompts::builders::ListAiPromptsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_ai_prompts::ListAiPromptsOutput,
        crate::operation::list_ai_prompts::ListAIPromptsError,
    > for ListAIPromptsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_ai_prompts::ListAiPromptsOutput,
            crate::operation::list_ai_prompts::ListAIPromptsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListAIPromptsFluentBuilder {
    /// Creates a new `ListAIPromptsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListAIPrompts as a reference.
    pub fn as_input(&self) -> &crate::operation::list_ai_prompts::builders::ListAiPromptsInputBuilder {
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
        crate::operation::list_ai_prompts::ListAiPromptsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_ai_prompts::ListAIPromptsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_ai_prompts::ListAIPrompts::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_ai_prompts::ListAIPrompts::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_ai_prompts::ListAiPromptsOutput,
        crate::operation::list_ai_prompts::ListAIPromptsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_ai_prompts::paginator::ListAiPromptsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_ai_prompts::paginator::ListAiPromptsPaginator {
        crate::operation::list_ai_prompts::paginator::ListAiPromptsPaginator::new(self.handle, self.inner)
    }
    /// <p>The identifier of the Amazon Q in Connect assistant. Can be either the ID or the ARN. URLs cannot contain the ARN.</p>
    pub fn assistant_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.assistant_id(input.into());
        self
    }
    /// <p>The identifier of the Amazon Q in Connect assistant. Can be either the ID or the ARN. URLs cannot contain the ARN.</p>
    pub fn set_assistant_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_assistant_id(input);
        self
    }
    /// <p>The identifier of the Amazon Q in Connect assistant. Can be either the ID or the ARN. URLs cannot contain the ARN.</p>
    pub fn get_assistant_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_assistant_id()
    }
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of results to return per page.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return per page.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to return per page.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>The origin of the AI Prompts to be listed. <code>SYSTEM</code> for a default AI Agent created by Q in Connect or <code>CUSTOMER</code> for an AI Agent created by calling AI Agent creation APIs.</p>
    pub fn origin(mut self, input: crate::types::Origin) -> Self {
        self.inner = self.inner.origin(input);
        self
    }
    /// <p>The origin of the AI Prompts to be listed. <code>SYSTEM</code> for a default AI Agent created by Q in Connect or <code>CUSTOMER</code> for an AI Agent created by calling AI Agent creation APIs.</p>
    pub fn set_origin(mut self, input: ::std::option::Option<crate::types::Origin>) -> Self {
        self.inner = self.inner.set_origin(input);
        self
    }
    /// <p>The origin of the AI Prompts to be listed. <code>SYSTEM</code> for a default AI Agent created by Q in Connect or <code>CUSTOMER</code> for an AI Agent created by calling AI Agent creation APIs.</p>
    pub fn get_origin(&self) -> &::std::option::Option<crate::types::Origin> {
        self.inner.get_origin()
    }
}
