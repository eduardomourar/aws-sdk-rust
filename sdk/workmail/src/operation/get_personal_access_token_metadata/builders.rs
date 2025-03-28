// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_personal_access_token_metadata::_get_personal_access_token_metadata_output::GetPersonalAccessTokenMetadataOutputBuilder;

pub use crate::operation::get_personal_access_token_metadata::_get_personal_access_token_metadata_input::GetPersonalAccessTokenMetadataInputBuilder;

impl crate::operation::get_personal_access_token_metadata::builders::GetPersonalAccessTokenMetadataInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_personal_access_token_metadata::GetPersonalAccessTokenMetadataOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_personal_access_token_metadata::GetPersonalAccessTokenMetadataError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_personal_access_token_metadata();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetPersonalAccessTokenMetadata`.
///
/// <p>Requests details of a specific Personal Access Token within the WorkMail organization.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetPersonalAccessTokenMetadataFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_personal_access_token_metadata::builders::GetPersonalAccessTokenMetadataInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_personal_access_token_metadata::GetPersonalAccessTokenMetadataOutput,
        crate::operation::get_personal_access_token_metadata::GetPersonalAccessTokenMetadataError,
    > for GetPersonalAccessTokenMetadataFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_personal_access_token_metadata::GetPersonalAccessTokenMetadataOutput,
            crate::operation::get_personal_access_token_metadata::GetPersonalAccessTokenMetadataError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetPersonalAccessTokenMetadataFluentBuilder {
    /// Creates a new `GetPersonalAccessTokenMetadataFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetPersonalAccessTokenMetadata as a reference.
    pub fn as_input(&self) -> &crate::operation::get_personal_access_token_metadata::builders::GetPersonalAccessTokenMetadataInputBuilder {
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
        crate::operation::get_personal_access_token_metadata::GetPersonalAccessTokenMetadataOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_personal_access_token_metadata::GetPersonalAccessTokenMetadataError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_personal_access_token_metadata::GetPersonalAccessTokenMetadata::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_personal_access_token_metadata::GetPersonalAccessTokenMetadata::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_personal_access_token_metadata::GetPersonalAccessTokenMetadataOutput,
        crate::operation::get_personal_access_token_metadata::GetPersonalAccessTokenMetadataError,
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
    /// <p>The Organization ID.</p>
    pub fn organization_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.organization_id(input.into());
        self
    }
    /// <p>The Organization ID.</p>
    pub fn set_organization_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_organization_id(input);
        self
    }
    /// <p>The Organization ID.</p>
    pub fn get_organization_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_organization_id()
    }
    /// <p>The Personal Access Token ID.</p>
    pub fn personal_access_token_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.personal_access_token_id(input.into());
        self
    }
    /// <p>The Personal Access Token ID.</p>
    pub fn set_personal_access_token_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_personal_access_token_id(input);
        self
    }
    /// <p>The Personal Access Token ID.</p>
    pub fn get_personal_access_token_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_personal_access_token_id()
    }
}
