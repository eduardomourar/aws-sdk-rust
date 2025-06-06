// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::is_authorized::_is_authorized_output::IsAuthorizedOutputBuilder;

pub use crate::operation::is_authorized::_is_authorized_input::IsAuthorizedInputBuilder;

impl crate::operation::is_authorized::builders::IsAuthorizedInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::is_authorized::IsAuthorizedOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::is_authorized::IsAuthorizedError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.is_authorized();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `IsAuthorized`.
///
/// <p>Makes an authorization decision about a service request described in the parameters. The information in the parameters can also define additional context that Verified Permissions can include in the evaluation. The request is evaluated against all matching policies in the specified policy store. The result of the decision is either <code>Allow</code> or <code>Deny</code>, along with a list of the policies that resulted in the decision.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct IsAuthorizedFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::is_authorized::builders::IsAuthorizedInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::is_authorized::IsAuthorizedOutput,
        crate::operation::is_authorized::IsAuthorizedError,
    > for IsAuthorizedFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::is_authorized::IsAuthorizedOutput,
            crate::operation::is_authorized::IsAuthorizedError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl IsAuthorizedFluentBuilder {
    /// Creates a new `IsAuthorizedFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the IsAuthorized as a reference.
    pub fn as_input(&self) -> &crate::operation::is_authorized::builders::IsAuthorizedInputBuilder {
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
        crate::operation::is_authorized::IsAuthorizedOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::is_authorized::IsAuthorizedError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::is_authorized::IsAuthorized::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::is_authorized::IsAuthorized::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::is_authorized::IsAuthorizedOutput,
        crate::operation::is_authorized::IsAuthorizedError,
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
    /// <p>Specifies the ID of the policy store. Policies in this policy store will be used to make an authorization decision for the input.</p>
    pub fn policy_store_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.policy_store_id(input.into());
        self
    }
    /// <p>Specifies the ID of the policy store. Policies in this policy store will be used to make an authorization decision for the input.</p>
    pub fn set_policy_store_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_policy_store_id(input);
        self
    }
    /// <p>Specifies the ID of the policy store. Policies in this policy store will be used to make an authorization decision for the input.</p>
    pub fn get_policy_store_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_policy_store_id()
    }
    /// <p>Specifies the principal for which the authorization decision is to be made.</p>
    pub fn principal(mut self, input: crate::types::EntityIdentifier) -> Self {
        self.inner = self.inner.principal(input);
        self
    }
    /// <p>Specifies the principal for which the authorization decision is to be made.</p>
    pub fn set_principal(mut self, input: ::std::option::Option<crate::types::EntityIdentifier>) -> Self {
        self.inner = self.inner.set_principal(input);
        self
    }
    /// <p>Specifies the principal for which the authorization decision is to be made.</p>
    pub fn get_principal(&self) -> &::std::option::Option<crate::types::EntityIdentifier> {
        self.inner.get_principal()
    }
    /// <p>Specifies the requested action to be authorized. For example, is the principal authorized to perform this action on the resource?</p>
    pub fn action(mut self, input: crate::types::ActionIdentifier) -> Self {
        self.inner = self.inner.action(input);
        self
    }
    /// <p>Specifies the requested action to be authorized. For example, is the principal authorized to perform this action on the resource?</p>
    pub fn set_action(mut self, input: ::std::option::Option<crate::types::ActionIdentifier>) -> Self {
        self.inner = self.inner.set_action(input);
        self
    }
    /// <p>Specifies the requested action to be authorized. For example, is the principal authorized to perform this action on the resource?</p>
    pub fn get_action(&self) -> &::std::option::Option<crate::types::ActionIdentifier> {
        self.inner.get_action()
    }
    /// <p>Specifies the resource for which the authorization decision is to be made.</p>
    pub fn resource(mut self, input: crate::types::EntityIdentifier) -> Self {
        self.inner = self.inner.resource(input);
        self
    }
    /// <p>Specifies the resource for which the authorization decision is to be made.</p>
    pub fn set_resource(mut self, input: ::std::option::Option<crate::types::EntityIdentifier>) -> Self {
        self.inner = self.inner.set_resource(input);
        self
    }
    /// <p>Specifies the resource for which the authorization decision is to be made.</p>
    pub fn get_resource(&self) -> &::std::option::Option<crate::types::EntityIdentifier> {
        self.inner.get_resource()
    }
    /// <p>Specifies additional context that can be used to make more granular authorization decisions.</p>
    pub fn context(mut self, input: crate::types::ContextDefinition) -> Self {
        self.inner = self.inner.context(input);
        self
    }
    /// <p>Specifies additional context that can be used to make more granular authorization decisions.</p>
    pub fn set_context(mut self, input: ::std::option::Option<crate::types::ContextDefinition>) -> Self {
        self.inner = self.inner.set_context(input);
        self
    }
    /// <p>Specifies additional context that can be used to make more granular authorization decisions.</p>
    pub fn get_context(&self) -> &::std::option::Option<crate::types::ContextDefinition> {
        self.inner.get_context()
    }
    /// <p>(Optional) Specifies the list of resources and principals and their associated attributes that Verified Permissions can examine when evaluating the policies. These additional entities and their attributes can be referenced and checked by conditional elements in the policies in the specified policy store.</p><note>
    /// <p>You can include only principal and resource entities in this parameter; you can't include actions. You must specify actions in the schema.</p>
    /// </note>
    pub fn entities(mut self, input: crate::types::EntitiesDefinition) -> Self {
        self.inner = self.inner.entities(input);
        self
    }
    /// <p>(Optional) Specifies the list of resources and principals and their associated attributes that Verified Permissions can examine when evaluating the policies. These additional entities and their attributes can be referenced and checked by conditional elements in the policies in the specified policy store.</p><note>
    /// <p>You can include only principal and resource entities in this parameter; you can't include actions. You must specify actions in the schema.</p>
    /// </note>
    pub fn set_entities(mut self, input: ::std::option::Option<crate::types::EntitiesDefinition>) -> Self {
        self.inner = self.inner.set_entities(input);
        self
    }
    /// <p>(Optional) Specifies the list of resources and principals and their associated attributes that Verified Permissions can examine when evaluating the policies. These additional entities and their attributes can be referenced and checked by conditional elements in the policies in the specified policy store.</p><note>
    /// <p>You can include only principal and resource entities in this parameter; you can't include actions. You must specify actions in the schema.</p>
    /// </note>
    pub fn get_entities(&self) -> &::std::option::Option<crate::types::EntitiesDefinition> {
        self.inner.get_entities()
    }
}
