// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_bridge_output::_update_bridge_output_output::UpdateBridgeOutputOutputBuilder;

pub use crate::operation::update_bridge_output::_update_bridge_output_input::UpdateBridgeOutputInputBuilder;

impl crate::operation::update_bridge_output::builders::UpdateBridgeOutputInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_bridge_output::UpdateBridgeOutputOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_bridge_output::UpdateBridgeOutputError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_bridge_output();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateBridgeOutput`.
///
/// <p>Updates an existing bridge output.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateBridgeOutputFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_bridge_output::builders::UpdateBridgeOutputInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_bridge_output::UpdateBridgeOutputOutput,
        crate::operation::update_bridge_output::UpdateBridgeOutputError,
    > for UpdateBridgeOutputFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_bridge_output::UpdateBridgeOutputOutput,
            crate::operation::update_bridge_output::UpdateBridgeOutputError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateBridgeOutputFluentBuilder {
    /// Creates a new `UpdateBridgeOutputFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateBridgeOutput as a reference.
    pub fn as_input(&self) -> &crate::operation::update_bridge_output::builders::UpdateBridgeOutputInputBuilder {
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
        crate::operation::update_bridge_output::UpdateBridgeOutputOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_bridge_output::UpdateBridgeOutputError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_bridge_output::UpdateBridgeOutput::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_bridge_output::UpdateBridgeOutput::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_bridge_output::UpdateBridgeOutputOutput,
        crate::operation::update_bridge_output::UpdateBridgeOutputError,
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
    /// <p>The Amazon Resource Name (ARN) of the bridge that you want to update.</p>
    pub fn bridge_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bridge_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the bridge that you want to update.</p>
    pub fn set_bridge_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bridge_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the bridge that you want to update.</p>
    pub fn get_bridge_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bridge_arn()
    }
    /// <p>The network of the bridge output.</p>
    pub fn network_output(mut self, input: crate::types::UpdateBridgeNetworkOutputRequest) -> Self {
        self.inner = self.inner.network_output(input);
        self
    }
    /// <p>The network of the bridge output.</p>
    pub fn set_network_output(mut self, input: ::std::option::Option<crate::types::UpdateBridgeNetworkOutputRequest>) -> Self {
        self.inner = self.inner.set_network_output(input);
        self
    }
    /// <p>The network of the bridge output.</p>
    pub fn get_network_output(&self) -> &::std::option::Option<crate::types::UpdateBridgeNetworkOutputRequest> {
        self.inner.get_network_output()
    }
    /// <p>Tname of the output that you want to update.</p>
    pub fn output_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.output_name(input.into());
        self
    }
    /// <p>Tname of the output that you want to update.</p>
    pub fn set_output_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_output_name(input);
        self
    }
    /// <p>Tname of the output that you want to update.</p>
    pub fn get_output_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_output_name()
    }
}
