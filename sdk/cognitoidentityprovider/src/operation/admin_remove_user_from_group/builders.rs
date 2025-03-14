// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::admin_remove_user_from_group::_admin_remove_user_from_group_output::AdminRemoveUserFromGroupOutputBuilder;

pub use crate::operation::admin_remove_user_from_group::_admin_remove_user_from_group_input::AdminRemoveUserFromGroupInputBuilder;

impl crate::operation::admin_remove_user_from_group::builders::AdminRemoveUserFromGroupInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::admin_remove_user_from_group::AdminRemoveUserFromGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::admin_remove_user_from_group::AdminRemoveUserFromGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.admin_remove_user_from_group();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AdminRemoveUserFromGroup`.
///
/// <p>Given a username and a group name, removes them from the group. User pool groups are identifiers that you can reference from the contents of ID and access tokens, and set preferred IAM roles for identity-pool authentication. For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pools-user-groups.html">Adding groups to a user pool</a>.</p><note>
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
pub struct AdminRemoveUserFromGroupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::admin_remove_user_from_group::builders::AdminRemoveUserFromGroupInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::admin_remove_user_from_group::AdminRemoveUserFromGroupOutput,
        crate::operation::admin_remove_user_from_group::AdminRemoveUserFromGroupError,
    > for AdminRemoveUserFromGroupFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::admin_remove_user_from_group::AdminRemoveUserFromGroupOutput,
            crate::operation::admin_remove_user_from_group::AdminRemoveUserFromGroupError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AdminRemoveUserFromGroupFluentBuilder {
    /// Creates a new `AdminRemoveUserFromGroupFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AdminRemoveUserFromGroup as a reference.
    pub fn as_input(&self) -> &crate::operation::admin_remove_user_from_group::builders::AdminRemoveUserFromGroupInputBuilder {
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
        crate::operation::admin_remove_user_from_group::AdminRemoveUserFromGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::admin_remove_user_from_group::AdminRemoveUserFromGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::admin_remove_user_from_group::AdminRemoveUserFromGroup::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::admin_remove_user_from_group::AdminRemoveUserFromGroup::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::admin_remove_user_from_group::AdminRemoveUserFromGroupOutput,
        crate::operation::admin_remove_user_from_group::AdminRemoveUserFromGroupError,
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
    /// <p>The ID of the user pool that contains the group and the user that you want to remove.</p>
    pub fn user_pool_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.user_pool_id(input.into());
        self
    }
    /// <p>The ID of the user pool that contains the group and the user that you want to remove.</p>
    pub fn set_user_pool_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_user_pool_id(input);
        self
    }
    /// <p>The ID of the user pool that contains the group and the user that you want to remove.</p>
    pub fn get_user_pool_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_user_pool_id()
    }
    /// <p>The name of the user that you want to query or modify. The value of this parameter is typically your user's username, but it can be any of their alias attributes. If <code>username</code> isn't an alias attribute in your user pool, this value must be the <code>sub</code> of a local user or the username of a user from a third-party IdP.</p>
    pub fn username(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.username(input.into());
        self
    }
    /// <p>The name of the user that you want to query or modify. The value of this parameter is typically your user's username, but it can be any of their alias attributes. If <code>username</code> isn't an alias attribute in your user pool, this value must be the <code>sub</code> of a local user or the username of a user from a third-party IdP.</p>
    pub fn set_username(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_username(input);
        self
    }
    /// <p>The name of the user that you want to query or modify. The value of this parameter is typically your user's username, but it can be any of their alias attributes. If <code>username</code> isn't an alias attribute in your user pool, this value must be the <code>sub</code> of a local user or the username of a user from a third-party IdP.</p>
    pub fn get_username(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_username()
    }
    /// <p>The name of the group that you want to remove the user from, for example <code>MyTestGroup</code>.</p>
    pub fn group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.group_name(input.into());
        self
    }
    /// <p>The name of the group that you want to remove the user from, for example <code>MyTestGroup</code>.</p>
    pub fn set_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_group_name(input);
        self
    }
    /// <p>The name of the group that you want to remove the user from, for example <code>MyTestGroup</code>.</p>
    pub fn get_group_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_group_name()
    }
}
