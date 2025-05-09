// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_policy_store::_get_policy_store_output::GetPolicyStoreOutputBuilder;

pub use crate::operation::get_policy_store::_get_policy_store_input::GetPolicyStoreInputBuilder;

impl crate::operation::get_policy_store::builders::GetPolicyStoreInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_policy_store::GetPolicyStoreOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_policy_store::GetPolicyStoreError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_policy_store();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetPolicyStore`.
///
/// <p>Retrieves details about a policy store.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetPolicyStoreFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_policy_store::builders::GetPolicyStoreInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_policy_store::GetPolicyStoreOutput,
        crate::operation::get_policy_store::GetPolicyStoreError,
    > for GetPolicyStoreFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_policy_store::GetPolicyStoreOutput,
            crate::operation::get_policy_store::GetPolicyStoreError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetPolicyStoreFluentBuilder {
    /// Creates a new `GetPolicyStoreFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetPolicyStore as a reference.
    pub fn as_input(&self) -> &crate::operation::get_policy_store::builders::GetPolicyStoreInputBuilder {
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
        crate::operation::get_policy_store::GetPolicyStoreOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_policy_store::GetPolicyStoreError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_policy_store::GetPolicyStore::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_policy_store::GetPolicyStore::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_policy_store::GetPolicyStoreOutput,
        crate::operation::get_policy_store::GetPolicyStoreError,
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
    /// <p>Specifies the ID of the policy store that you want information about.</p>
    pub fn policy_store_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.policy_store_id(input.into());
        self
    }
    /// <p>Specifies the ID of the policy store that you want information about.</p>
    pub fn set_policy_store_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_policy_store_id(input);
        self
    }
    /// <p>Specifies the ID of the policy store that you want information about.</p>
    pub fn get_policy_store_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_policy_store_id()
    }
    /// <p>Specifies whether to return the tags that are attached to the policy store. If this parameter is included in the API call, the tags are returned, otherwise they are not returned.</p><note>
    /// <p>If this parameter is included in the API call but there are no tags attached to the policy store, the <code>tags</code> response parameter is omitted from the response.</p>
    /// </note>
    pub fn tags(mut self, input: bool) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>Specifies whether to return the tags that are attached to the policy store. If this parameter is included in the API call, the tags are returned, otherwise they are not returned.</p><note>
    /// <p>If this parameter is included in the API call but there are no tags attached to the policy store, the <code>tags</code> response parameter is omitted from the response.</p>
    /// </note>
    pub fn set_tags(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>Specifies whether to return the tags that are attached to the policy store. If this parameter is included in the API call, the tags are returned, otherwise they are not returned.</p><note>
    /// <p>If this parameter is included in the API call but there are no tags attached to the policy store, the <code>tags</code> response parameter is omitted from the response.</p>
    /// </note>
    pub fn get_tags(&self) -> &::std::option::Option<bool> {
        self.inner.get_tags()
    }
}
