// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_chat_response_configuration::_create_chat_response_configuration_output::CreateChatResponseConfigurationOutputBuilder;

pub use crate::operation::create_chat_response_configuration::_create_chat_response_configuration_input::CreateChatResponseConfigurationInputBuilder;

impl crate::operation::create_chat_response_configuration::builders::CreateChatResponseConfigurationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_chat_response_configuration::CreateChatResponseConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_chat_response_configuration::CreateChatResponseConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_chat_response_configuration();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateChatResponseConfiguration`.
///
/// <p>Creates a new chat response configuration for an Amazon Q Business application. This operation establishes a set of parameters that define how the system generates and formats responses to user queries in chat interactions.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateChatResponseConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_chat_response_configuration::builders::CreateChatResponseConfigurationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_chat_response_configuration::CreateChatResponseConfigurationOutput,
        crate::operation::create_chat_response_configuration::CreateChatResponseConfigurationError,
    > for CreateChatResponseConfigurationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_chat_response_configuration::CreateChatResponseConfigurationOutput,
            crate::operation::create_chat_response_configuration::CreateChatResponseConfigurationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateChatResponseConfigurationFluentBuilder {
    /// Creates a new `CreateChatResponseConfigurationFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateChatResponseConfiguration as a reference.
    pub fn as_input(&self) -> &crate::operation::create_chat_response_configuration::builders::CreateChatResponseConfigurationInputBuilder {
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
        crate::operation::create_chat_response_configuration::CreateChatResponseConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_chat_response_configuration::CreateChatResponseConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_chat_response_configuration::CreateChatResponseConfiguration::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_chat_response_configuration::CreateChatResponseConfiguration::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_chat_response_configuration::CreateChatResponseConfigurationOutput,
        crate::operation::create_chat_response_configuration::CreateChatResponseConfigurationError,
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
    /// <p>The unique identifier of the Amazon Q Business application for which to create the new chat response configuration.</p>
    pub fn application_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.application_id(input.into());
        self
    }
    /// <p>The unique identifier of the Amazon Q Business application for which to create the new chat response configuration.</p>
    pub fn set_application_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_application_id(input);
        self
    }
    /// <p>The unique identifier of the Amazon Q Business application for which to create the new chat response configuration.</p>
    pub fn get_application_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_application_id()
    }
    /// <p>A human-readable name for the new chat response configuration, making it easier to identify and manage among multiple configurations.</p>
    pub fn display_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.display_name(input.into());
        self
    }
    /// <p>A human-readable name for the new chat response configuration, making it easier to identify and manage among multiple configurations.</p>
    pub fn set_display_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_display_name(input);
        self
    }
    /// <p>A human-readable name for the new chat response configuration, making it easier to identify and manage among multiple configurations.</p>
    pub fn get_display_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_display_name()
    }
    /// <p>A unique, case-sensitive identifier to ensure idempotency of the request. This helps prevent the same configuration from being created multiple times if retries occur.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A unique, case-sensitive identifier to ensure idempotency of the request. This helps prevent the same configuration from being created multiple times if retries occur.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>A unique, case-sensitive identifier to ensure idempotency of the request. This helps prevent the same configuration from being created multiple times if retries occur.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    ///
    /// Adds a key-value pair to `responseConfigurations`.
    ///
    /// To override the contents of this collection use [`set_response_configurations`](Self::set_response_configurations).
    ///
    /// <p>A collection of response configuration settings that define how Amazon Q Business will generate and format responses to user queries in chat interactions.</p>
    pub fn response_configurations(mut self, k: crate::types::ResponseConfigurationType, v: crate::types::ResponseConfiguration) -> Self {
        self.inner = self.inner.response_configurations(k, v);
        self
    }
    /// <p>A collection of response configuration settings that define how Amazon Q Business will generate and format responses to user queries in chat interactions.</p>
    pub fn set_response_configurations(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<crate::types::ResponseConfigurationType, crate::types::ResponseConfiguration>>,
    ) -> Self {
        self.inner = self.inner.set_response_configurations(input);
        self
    }
    /// <p>A collection of response configuration settings that define how Amazon Q Business will generate and format responses to user queries in chat interactions.</p>
    pub fn get_response_configurations(
        &self,
    ) -> &::std::option::Option<::std::collections::HashMap<crate::types::ResponseConfigurationType, crate::types::ResponseConfiguration>> {
        self.inner.get_response_configurations()
    }
    ///
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A list of key-value pairs to apply as tags to the new chat response configuration, enabling categorization and management of resources across Amazon Web Services services.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>A list of key-value pairs to apply as tags to the new chat response configuration, enabling categorization and management of resources across Amazon Web Services services.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>A list of key-value pairs to apply as tags to the new chat response configuration, enabling categorization and management of resources across Amazon Web Services services.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
}
