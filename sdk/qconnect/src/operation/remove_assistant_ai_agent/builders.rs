// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::remove_assistant_ai_agent::_remove_assistant_ai_agent_output::RemoveAssistantAiAgentOutputBuilder;

pub use crate::operation::remove_assistant_ai_agent::_remove_assistant_ai_agent_input::RemoveAssistantAiAgentInputBuilder;

impl crate::operation::remove_assistant_ai_agent::builders::RemoveAssistantAiAgentInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::remove_assistant_ai_agent::RemoveAssistantAiAgentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::remove_assistant_ai_agent::RemoveAssistantAIAgentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.remove_assistant_ai_agent();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `RemoveAssistantAIAgent`.
///
/// <p>Removes the AI Agent that is set for use by default on an Amazon Q in Connect Assistant.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RemoveAssistantAIAgentFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::remove_assistant_ai_agent::builders::RemoveAssistantAiAgentInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::remove_assistant_ai_agent::RemoveAssistantAiAgentOutput,
        crate::operation::remove_assistant_ai_agent::RemoveAssistantAIAgentError,
    > for RemoveAssistantAIAgentFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::remove_assistant_ai_agent::RemoveAssistantAiAgentOutput,
            crate::operation::remove_assistant_ai_agent::RemoveAssistantAIAgentError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl RemoveAssistantAIAgentFluentBuilder {
    /// Creates a new `RemoveAssistantAIAgentFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the RemoveAssistantAIAgent as a reference.
    pub fn as_input(&self) -> &crate::operation::remove_assistant_ai_agent::builders::RemoveAssistantAiAgentInputBuilder {
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
        crate::operation::remove_assistant_ai_agent::RemoveAssistantAiAgentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::remove_assistant_ai_agent::RemoveAssistantAIAgentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::remove_assistant_ai_agent::RemoveAssistantAIAgent::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::remove_assistant_ai_agent::RemoveAssistantAIAgent::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::remove_assistant_ai_agent::RemoveAssistantAiAgentOutput,
        crate::operation::remove_assistant_ai_agent::RemoveAssistantAIAgentError,
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
    /// <p>The type of the AI Agent being removed for use by default from the Amazon Q in Connect Assistant.</p>
    pub fn ai_agent_type(mut self, input: crate::types::AiAgentType) -> Self {
        self.inner = self.inner.ai_agent_type(input);
        self
    }
    /// <p>The type of the AI Agent being removed for use by default from the Amazon Q in Connect Assistant.</p>
    pub fn set_ai_agent_type(mut self, input: ::std::option::Option<crate::types::AiAgentType>) -> Self {
        self.inner = self.inner.set_ai_agent_type(input);
        self
    }
    /// <p>The type of the AI Agent being removed for use by default from the Amazon Q in Connect Assistant.</p>
    pub fn get_ai_agent_type(&self) -> &::std::option::Option<crate::types::AiAgentType> {
        self.inner.get_ai_agent_type()
    }
}
