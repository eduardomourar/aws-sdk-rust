// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_map_style_descriptor::_get_map_style_descriptor_output::GetMapStyleDescriptorOutputBuilder;

pub use crate::operation::get_map_style_descriptor::_get_map_style_descriptor_input::GetMapStyleDescriptorInputBuilder;

impl crate::operation::get_map_style_descriptor::builders::GetMapStyleDescriptorInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_map_style_descriptor::GetMapStyleDescriptorOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_map_style_descriptor::GetMapStyleDescriptorError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_map_style_descriptor();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetMapStyleDescriptor`.
///
/// <p>Retrieves the map style descriptor from a map resource.</p>
/// <p>The style descriptor contains speciﬁcations on how features render on a map. For example, what data to display, what order to display the data in, and the style for the data. Style descriptors follow the Mapbox Style Specification.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetMapStyleDescriptorFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_map_style_descriptor::builders::GetMapStyleDescriptorInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_map_style_descriptor::GetMapStyleDescriptorOutput,
        crate::operation::get_map_style_descriptor::GetMapStyleDescriptorError,
    > for GetMapStyleDescriptorFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_map_style_descriptor::GetMapStyleDescriptorOutput,
            crate::operation::get_map_style_descriptor::GetMapStyleDescriptorError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetMapStyleDescriptorFluentBuilder {
    /// Creates a new `GetMapStyleDescriptorFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetMapStyleDescriptor as a reference.
    pub fn as_input(&self) -> &crate::operation::get_map_style_descriptor::builders::GetMapStyleDescriptorInputBuilder {
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
        crate::operation::get_map_style_descriptor::GetMapStyleDescriptorOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_map_style_descriptor::GetMapStyleDescriptorError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_map_style_descriptor::GetMapStyleDescriptor::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_map_style_descriptor::GetMapStyleDescriptor::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_map_style_descriptor::GetMapStyleDescriptorOutput,
        crate::operation::get_map_style_descriptor::GetMapStyleDescriptorError,
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
    /// <p>The map resource to retrieve the style descriptor from.</p>
    pub fn map_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.map_name(input.into());
        self
    }
    /// <p>The map resource to retrieve the style descriptor from.</p>
    pub fn set_map_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_map_name(input);
        self
    }
    /// <p>The map resource to retrieve the style descriptor from.</p>
    pub fn get_map_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_map_name()
    }
    /// <p>The optional <a href="https://docs.aws.amazon.com/location/previous/developerguide/using-apikeys.html">API key</a> to authorize the request.</p>
    pub fn key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.key(input.into());
        self
    }
    /// <p>The optional <a href="https://docs.aws.amazon.com/location/previous/developerguide/using-apikeys.html">API key</a> to authorize the request.</p>
    pub fn set_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_key(input);
        self
    }
    /// <p>The optional <a href="https://docs.aws.amazon.com/location/previous/developerguide/using-apikeys.html">API key</a> to authorize the request.</p>
    pub fn get_key(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_key()
    }
}
