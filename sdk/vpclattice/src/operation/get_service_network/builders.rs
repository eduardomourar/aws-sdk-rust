// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_service_network::_get_service_network_output::GetServiceNetworkOutputBuilder;

pub use crate::operation::get_service_network::_get_service_network_input::GetServiceNetworkInputBuilder;

impl crate::operation::get_service_network::builders::GetServiceNetworkInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_service_network::GetServiceNetworkOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_service_network::GetServiceNetworkError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_service_network();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetServiceNetwork`.
///
/// <p>Retrieves information about the specified service network.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetServiceNetworkFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_service_network::builders::GetServiceNetworkInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_service_network::GetServiceNetworkOutput,
        crate::operation::get_service_network::GetServiceNetworkError,
    > for GetServiceNetworkFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_service_network::GetServiceNetworkOutput,
            crate::operation::get_service_network::GetServiceNetworkError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetServiceNetworkFluentBuilder {
    /// Creates a new `GetServiceNetworkFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetServiceNetwork as a reference.
    pub fn as_input(&self) -> &crate::operation::get_service_network::builders::GetServiceNetworkInputBuilder {
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
        crate::operation::get_service_network::GetServiceNetworkOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_service_network::GetServiceNetworkError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_service_network::GetServiceNetwork::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_service_network::GetServiceNetwork::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_service_network::GetServiceNetworkOutput,
        crate::operation::get_service_network::GetServiceNetworkError,
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
    /// <p>The ID or ARN of the service network.</p>
    pub fn service_network_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.service_network_identifier(input.into());
        self
    }
    /// <p>The ID or ARN of the service network.</p>
    pub fn set_service_network_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_service_network_identifier(input);
        self
    }
    /// <p>The ID or ARN of the service network.</p>
    pub fn get_service_network_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_service_network_identifier()
    }
}
