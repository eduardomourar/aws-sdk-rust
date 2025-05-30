// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_service_network_resource_association::_create_service_network_resource_association_output::CreateServiceNetworkResourceAssociationOutputBuilder;

pub use crate::operation::create_service_network_resource_association::_create_service_network_resource_association_input::CreateServiceNetworkResourceAssociationInputBuilder;

impl crate::operation::create_service_network_resource_association::builders::CreateServiceNetworkResourceAssociationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_service_network_resource_association::CreateServiceNetworkResourceAssociationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_service_network_resource_association::CreateServiceNetworkResourceAssociationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_service_network_resource_association();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateServiceNetworkResourceAssociation`.
///
/// <p>Associates the specified service network with the specified resource configuration. This allows the resource configuration to receive connections through the service network, including through a service network VPC endpoint.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateServiceNetworkResourceAssociationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_service_network_resource_association::builders::CreateServiceNetworkResourceAssociationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_service_network_resource_association::CreateServiceNetworkResourceAssociationOutput,
        crate::operation::create_service_network_resource_association::CreateServiceNetworkResourceAssociationError,
    > for CreateServiceNetworkResourceAssociationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_service_network_resource_association::CreateServiceNetworkResourceAssociationOutput,
            crate::operation::create_service_network_resource_association::CreateServiceNetworkResourceAssociationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateServiceNetworkResourceAssociationFluentBuilder {
    /// Creates a new `CreateServiceNetworkResourceAssociationFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateServiceNetworkResourceAssociation as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::create_service_network_resource_association::builders::CreateServiceNetworkResourceAssociationInputBuilder {
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
        crate::operation::create_service_network_resource_association::CreateServiceNetworkResourceAssociationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_service_network_resource_association::CreateServiceNetworkResourceAssociationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::create_service_network_resource_association::CreateServiceNetworkResourceAssociation::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::create_service_network_resource_association::CreateServiceNetworkResourceAssociation::orchestrate(&runtime_plugins, input)
            .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_service_network_resource_association::CreateServiceNetworkResourceAssociationOutput,
        crate::operation::create_service_network_resource_association::CreateServiceNetworkResourceAssociationError,
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
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If you retry a request that completed successfully using the same client token and parameters, the retry succeeds without performing any actions. If the parameters aren't identical, the retry fails.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If you retry a request that completed successfully using the same client token and parameters, the retry succeeds without performing any actions. If the parameters aren't identical, the retry fails.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If you retry a request that completed successfully using the same client token and parameters, the retry succeeds without performing any actions. If the parameters aren't identical, the retry fails.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// <p>The ID of the resource configuration to associate with the service network.</p>
    pub fn resource_configuration_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_configuration_identifier(input.into());
        self
    }
    /// <p>The ID of the resource configuration to associate with the service network.</p>
    pub fn set_resource_configuration_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_configuration_identifier(input);
        self
    }
    /// <p>The ID of the resource configuration to associate with the service network.</p>
    pub fn get_resource_configuration_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resource_configuration_identifier()
    }
    /// <p>The ID of the service network to associate with the resource configuration.</p>
    pub fn service_network_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.service_network_identifier(input.into());
        self
    }
    /// <p>The ID of the service network to associate with the resource configuration.</p>
    pub fn set_service_network_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_service_network_identifier(input);
        self
    }
    /// <p>The ID of the service network to associate with the resource configuration.</p>
    pub fn get_service_network_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_service_network_identifier()
    }
    ///
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags for the association.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>The tags for the association.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The tags for the association.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_tags()
    }
}
