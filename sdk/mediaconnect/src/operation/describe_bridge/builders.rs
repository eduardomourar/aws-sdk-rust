// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_bridge::_describe_bridge_output::DescribeBridgeOutputBuilder;

pub use crate::operation::describe_bridge::_describe_bridge_input::DescribeBridgeInputBuilder;

impl crate::operation::describe_bridge::builders::DescribeBridgeInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_bridge::DescribeBridgeOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_bridge::DescribeBridgeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_bridge();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeBridge`.
///
/// <p>Displays the details of a bridge.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeBridgeFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_bridge::builders::DescribeBridgeInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_bridge::DescribeBridgeOutput,
        crate::operation::describe_bridge::DescribeBridgeError,
    > for DescribeBridgeFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_bridge::DescribeBridgeOutput,
            crate::operation::describe_bridge::DescribeBridgeError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeBridgeFluentBuilder {
    /// Creates a new `DescribeBridgeFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeBridge as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_bridge::builders::DescribeBridgeInputBuilder {
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
        crate::operation::describe_bridge::DescribeBridgeOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_bridge::DescribeBridgeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_bridge::DescribeBridge::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_bridge::DescribeBridge::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_bridge::DescribeBridgeOutput,
        crate::operation::describe_bridge::DescribeBridgeError,
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
    /// <p>The Amazon Resource Name (ARN) of the bridge that you want to describe.</p>
    pub fn bridge_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bridge_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the bridge that you want to describe.</p>
    pub fn set_bridge_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bridge_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the bridge that you want to describe.</p>
    pub fn get_bridge_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bridge_arn()
    }
}
