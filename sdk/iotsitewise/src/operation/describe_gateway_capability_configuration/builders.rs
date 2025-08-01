// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_gateway_capability_configuration::_describe_gateway_capability_configuration_output::DescribeGatewayCapabilityConfigurationOutputBuilder;

pub use crate::operation::describe_gateway_capability_configuration::_describe_gateway_capability_configuration_input::DescribeGatewayCapabilityConfigurationInputBuilder;

impl crate::operation::describe_gateway_capability_configuration::builders::DescribeGatewayCapabilityConfigurationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_gateway_capability_configuration::DescribeGatewayCapabilityConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_gateway_capability_configuration::DescribeGatewayCapabilityConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_gateway_capability_configuration();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeGatewayCapabilityConfiguration`.
///
/// <p>Each gateway capability defines data sources for a gateway. This is the namespace of the gateway capability.</p>
/// <p>. The namespace follows the format <code>service:capability:version</code>, where:</p>
/// <ul>
/// <li>
/// <p><code>service</code> - The service providing the capability, or <code>iotsitewise</code>.</p></li>
/// <li>
/// <p><code>capability</code> - The specific capability type. Options include: <code>opcuacollector</code> for the OPC UA data source collector, or <code>publisher</code> for data publisher capability.</p></li>
/// <li>
/// <p><code>version</code> - The version number of the capability. Option include <code>2</code> for Classic streams, V2 gateways, and <code>3</code> for MQTT-enabled, V3 gateways.</p></li>
/// </ul>
/// <p>After updating a capability configuration, the sync status becomes <code>OUT_OF_SYNC</code> until the gateway processes the configuration.Use <code>DescribeGatewayCapabilityConfiguration</code> to check the sync status and verify the configuration was applied.</p>
/// <p>A gateway can have multiple capability configurations with different namespaces.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeGatewayCapabilityConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_gateway_capability_configuration::builders::DescribeGatewayCapabilityConfigurationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_gateway_capability_configuration::DescribeGatewayCapabilityConfigurationOutput,
        crate::operation::describe_gateway_capability_configuration::DescribeGatewayCapabilityConfigurationError,
    > for DescribeGatewayCapabilityConfigurationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_gateway_capability_configuration::DescribeGatewayCapabilityConfigurationOutput,
            crate::operation::describe_gateway_capability_configuration::DescribeGatewayCapabilityConfigurationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeGatewayCapabilityConfigurationFluentBuilder {
    /// Creates a new `DescribeGatewayCapabilityConfigurationFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeGatewayCapabilityConfiguration as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::describe_gateway_capability_configuration::builders::DescribeGatewayCapabilityConfigurationInputBuilder {
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
        crate::operation::describe_gateway_capability_configuration::DescribeGatewayCapabilityConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_gateway_capability_configuration::DescribeGatewayCapabilityConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::describe_gateway_capability_configuration::DescribeGatewayCapabilityConfiguration::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::describe_gateway_capability_configuration::DescribeGatewayCapabilityConfiguration::orchestrate(&runtime_plugins, input)
            .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_gateway_capability_configuration::DescribeGatewayCapabilityConfigurationOutput,
        crate::operation::describe_gateway_capability_configuration::DescribeGatewayCapabilityConfigurationError,
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
    /// <p>The ID of the gateway that defines the capability configuration.</p>
    pub fn gateway_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.gateway_id(input.into());
        self
    }
    /// <p>The ID of the gateway that defines the capability configuration.</p>
    pub fn set_gateway_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_gateway_id(input);
        self
    }
    /// <p>The ID of the gateway that defines the capability configuration.</p>
    pub fn get_gateway_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_gateway_id()
    }
    /// <p>The namespace of the capability configuration. For example, if you configure OPC UA sources for an MQTT-enabled gateway, your OPC-UA capability configuration has the namespace <code>iotsitewise:opcuacollector:3</code>.</p>
    pub fn capability_namespace(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.capability_namespace(input.into());
        self
    }
    /// <p>The namespace of the capability configuration. For example, if you configure OPC UA sources for an MQTT-enabled gateway, your OPC-UA capability configuration has the namespace <code>iotsitewise:opcuacollector:3</code>.</p>
    pub fn set_capability_namespace(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_capability_namespace(input);
        self
    }
    /// <p>The namespace of the capability configuration. For example, if you configure OPC UA sources for an MQTT-enabled gateway, your OPC-UA capability configuration has the namespace <code>iotsitewise:opcuacollector:3</code>.</p>
    pub fn get_capability_namespace(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_capability_namespace()
    }
}
