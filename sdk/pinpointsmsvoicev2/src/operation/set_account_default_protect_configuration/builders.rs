// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::set_account_default_protect_configuration::_set_account_default_protect_configuration_output::SetAccountDefaultProtectConfigurationOutputBuilder;

pub use crate::operation::set_account_default_protect_configuration::_set_account_default_protect_configuration_input::SetAccountDefaultProtectConfigurationInputBuilder;

impl crate::operation::set_account_default_protect_configuration::builders::SetAccountDefaultProtectConfigurationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::set_account_default_protect_configuration::SetAccountDefaultProtectConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::set_account_default_protect_configuration::SetAccountDefaultProtectConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.set_account_default_protect_configuration();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `SetAccountDefaultProtectConfiguration`.
///
/// <p>Set a protect configuration as your account default. You can only have one account default protect configuration at a time. The current account default protect configuration is replaced with the provided protect configuration.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct SetAccountDefaultProtectConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::set_account_default_protect_configuration::builders::SetAccountDefaultProtectConfigurationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::set_account_default_protect_configuration::SetAccountDefaultProtectConfigurationOutput,
        crate::operation::set_account_default_protect_configuration::SetAccountDefaultProtectConfigurationError,
    > for SetAccountDefaultProtectConfigurationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::set_account_default_protect_configuration::SetAccountDefaultProtectConfigurationOutput,
            crate::operation::set_account_default_protect_configuration::SetAccountDefaultProtectConfigurationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl SetAccountDefaultProtectConfigurationFluentBuilder {
    /// Creates a new `SetAccountDefaultProtectConfigurationFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the SetAccountDefaultProtectConfiguration as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::set_account_default_protect_configuration::builders::SetAccountDefaultProtectConfigurationInputBuilder {
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
        crate::operation::set_account_default_protect_configuration::SetAccountDefaultProtectConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::set_account_default_protect_configuration::SetAccountDefaultProtectConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::set_account_default_protect_configuration::SetAccountDefaultProtectConfiguration::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::set_account_default_protect_configuration::SetAccountDefaultProtectConfiguration::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::set_account_default_protect_configuration::SetAccountDefaultProtectConfigurationOutput,
        crate::operation::set_account_default_protect_configuration::SetAccountDefaultProtectConfigurationError,
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
    /// <p>The unique identifier for the protect configuration.</p>
    pub fn protect_configuration_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.protect_configuration_id(input.into());
        self
    }
    /// <p>The unique identifier for the protect configuration.</p>
    pub fn set_protect_configuration_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_protect_configuration_id(input);
        self
    }
    /// <p>The unique identifier for the protect configuration.</p>
    pub fn get_protect_configuration_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_protect_configuration_id()
    }
}
