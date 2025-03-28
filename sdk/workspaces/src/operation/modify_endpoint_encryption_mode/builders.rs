// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_endpoint_encryption_mode::_modify_endpoint_encryption_mode_output::ModifyEndpointEncryptionModeOutputBuilder;

pub use crate::operation::modify_endpoint_encryption_mode::_modify_endpoint_encryption_mode_input::ModifyEndpointEncryptionModeInputBuilder;

impl crate::operation::modify_endpoint_encryption_mode::builders::ModifyEndpointEncryptionModeInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::modify_endpoint_encryption_mode::ModifyEndpointEncryptionModeOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_endpoint_encryption_mode::ModifyEndpointEncryptionModeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.modify_endpoint_encryption_mode();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ModifyEndpointEncryptionMode`.
///
/// <p>Modifies the endpoint encryption mode that allows you to configure the specified directory between Standard TLS and FIPS 140-2 validated mode.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifyEndpointEncryptionModeFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::modify_endpoint_encryption_mode::builders::ModifyEndpointEncryptionModeInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::modify_endpoint_encryption_mode::ModifyEndpointEncryptionModeOutput,
        crate::operation::modify_endpoint_encryption_mode::ModifyEndpointEncryptionModeError,
    > for ModifyEndpointEncryptionModeFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::modify_endpoint_encryption_mode::ModifyEndpointEncryptionModeOutput,
            crate::operation::modify_endpoint_encryption_mode::ModifyEndpointEncryptionModeError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ModifyEndpointEncryptionModeFluentBuilder {
    /// Creates a new `ModifyEndpointEncryptionModeFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ModifyEndpointEncryptionMode as a reference.
    pub fn as_input(&self) -> &crate::operation::modify_endpoint_encryption_mode::builders::ModifyEndpointEncryptionModeInputBuilder {
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
        crate::operation::modify_endpoint_encryption_mode::ModifyEndpointEncryptionModeOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_endpoint_encryption_mode::ModifyEndpointEncryptionModeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::modify_endpoint_encryption_mode::ModifyEndpointEncryptionMode::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::modify_endpoint_encryption_mode::ModifyEndpointEncryptionMode::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::modify_endpoint_encryption_mode::ModifyEndpointEncryptionModeOutput,
        crate::operation::modify_endpoint_encryption_mode::ModifyEndpointEncryptionModeError,
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
    /// <p>The identifier of the directory.</p>
    pub fn directory_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.directory_id(input.into());
        self
    }
    /// <p>The identifier of the directory.</p>
    pub fn set_directory_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_directory_id(input);
        self
    }
    /// <p>The identifier of the directory.</p>
    pub fn get_directory_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_directory_id()
    }
    /// <p>The encryption mode used for endpoint connections when streaming to WorkSpaces Personal or WorkSpace Pools.</p>
    pub fn endpoint_encryption_mode(mut self, input: crate::types::EndpointEncryptionMode) -> Self {
        self.inner = self.inner.endpoint_encryption_mode(input);
        self
    }
    /// <p>The encryption mode used for endpoint connections when streaming to WorkSpaces Personal or WorkSpace Pools.</p>
    pub fn set_endpoint_encryption_mode(mut self, input: ::std::option::Option<crate::types::EndpointEncryptionMode>) -> Self {
        self.inner = self.inner.set_endpoint_encryption_mode(input);
        self
    }
    /// <p>The encryption mode used for endpoint connections when streaming to WorkSpaces Personal or WorkSpace Pools.</p>
    pub fn get_endpoint_encryption_mode(&self) -> &::std::option::Option<crate::types::EndpointEncryptionMode> {
        self.inner.get_endpoint_encryption_mode()
    }
}
