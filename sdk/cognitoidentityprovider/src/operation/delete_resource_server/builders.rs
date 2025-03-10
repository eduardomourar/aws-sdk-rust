// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_resource_server::_delete_resource_server_output::DeleteResourceServerOutputBuilder;

pub use crate::operation::delete_resource_server::_delete_resource_server_input::DeleteResourceServerInputBuilder;

impl crate::operation::delete_resource_server::builders::DeleteResourceServerInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_resource_server::DeleteResourceServerOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_resource_server::DeleteResourceServerError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_resource_server();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteResourceServer`.
///
/// <p>Deletes a resource server. After you delete a resource server, users can no longer generate access tokens with scopes that are associate with that resource server.</p>
/// <p>Resource servers are associated with custom scopes and machine-to-machine (M2M) authorization. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-define-resource-servers.html">Access control with resource servers</a>.</p><note>
/// <p>Amazon Cognito evaluates Identity and Access Management (IAM) policies in requests for this API operation. For this operation, you must use IAM credentials to authorize requests, and you must grant yourself the corresponding IAM permission in a policy.</p>
/// <p class="title"><b>Learn more</b></p>
/// <ul>
/// <li>
/// <p><a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_aws-signing.html">Signing Amazon Web Services API Requests</a></p></li>
/// <li>
/// <p><a href="https://docs.aws.amazon.com/cognito/latest/developerguide/user-pools-API-operations.html">Using the Amazon Cognito user pools API and user pool endpoints</a></p></li>
/// </ul>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteResourceServerFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_resource_server::builders::DeleteResourceServerInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_resource_server::DeleteResourceServerOutput,
        crate::operation::delete_resource_server::DeleteResourceServerError,
    > for DeleteResourceServerFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_resource_server::DeleteResourceServerOutput,
            crate::operation::delete_resource_server::DeleteResourceServerError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteResourceServerFluentBuilder {
    /// Creates a new `DeleteResourceServerFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteResourceServer as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_resource_server::builders::DeleteResourceServerInputBuilder {
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
        crate::operation::delete_resource_server::DeleteResourceServerOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_resource_server::DeleteResourceServerError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_resource_server::DeleteResourceServer::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_resource_server::DeleteResourceServer::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_resource_server::DeleteResourceServerOutput,
        crate::operation::delete_resource_server::DeleteResourceServerError,
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
    /// <p>The ID of the user pool where you want to delete the resource server.</p>
    pub fn user_pool_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.user_pool_id(input.into());
        self
    }
    /// <p>The ID of the user pool where you want to delete the resource server.</p>
    pub fn set_user_pool_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_user_pool_id(input);
        self
    }
    /// <p>The ID of the user pool where you want to delete the resource server.</p>
    pub fn get_user_pool_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_user_pool_id()
    }
    /// <p>The identifier of the resource server that you want to delete.</p>
    pub fn identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.identifier(input.into());
        self
    }
    /// <p>The identifier of the resource server that you want to delete.</p>
    pub fn set_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_identifier(input);
        self
    }
    /// <p>The identifier of the resource server that you want to delete.</p>
    pub fn get_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_identifier()
    }
}
