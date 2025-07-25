// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_login_profile::_create_login_profile_output::CreateLoginProfileOutputBuilder;

pub use crate::operation::create_login_profile::_create_login_profile_input::CreateLoginProfileInputBuilder;

impl crate::operation::create_login_profile::builders::CreateLoginProfileInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_login_profile::CreateLoginProfileOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_login_profile::CreateLoginProfileError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_login_profile();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateLoginProfile`.
///
/// <p>Creates a password for the specified IAM user. A password allows an IAM user to access Amazon Web Services services through the Amazon Web Services Management Console.</p>
/// <p>You can use the CLI, the Amazon Web Services API, or the <b>Users</b> page in the IAM console to create a password for any IAM user. Use <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_ChangePassword.html">ChangePassword</a> to update your own existing password in the <b>My Security Credentials</b> page in the Amazon Web Services Management Console.</p>
/// <p>For more information about managing passwords, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/Using_ManagingLogins.html">Managing passwords</a> in the <i>IAM User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateLoginProfileFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_login_profile::builders::CreateLoginProfileInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_login_profile::CreateLoginProfileOutput,
        crate::operation::create_login_profile::CreateLoginProfileError,
    > for CreateLoginProfileFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_login_profile::CreateLoginProfileOutput,
            crate::operation::create_login_profile::CreateLoginProfileError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateLoginProfileFluentBuilder {
    /// Creates a new `CreateLoginProfileFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateLoginProfile as a reference.
    pub fn as_input(&self) -> &crate::operation::create_login_profile::builders::CreateLoginProfileInputBuilder {
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
        crate::operation::create_login_profile::CreateLoginProfileOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_login_profile::CreateLoginProfileError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_login_profile::CreateLoginProfile::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_login_profile::CreateLoginProfile::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_login_profile::CreateLoginProfileOutput,
        crate::operation::create_login_profile::CreateLoginProfileError,
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
    /// <p>The name of the IAM user to create a password for. The user must already exist.</p>
    /// <p>This parameter is optional. If no user name is included, it defaults to the principal making the request. When you make this request with root user credentials, you must use an <a href="https://docs.aws.amazon.com/STS/latest/APIReference/API_AssumeRoot.html">AssumeRoot</a> session to omit the user name.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn user_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.user_name(input.into());
        self
    }
    /// <p>The name of the IAM user to create a password for. The user must already exist.</p>
    /// <p>This parameter is optional. If no user name is included, it defaults to the principal making the request. When you make this request with root user credentials, you must use an <a href="https://docs.aws.amazon.com/STS/latest/APIReference/API_AssumeRoot.html">AssumeRoot</a> session to omit the user name.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn set_user_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_user_name(input);
        self
    }
    /// <p>The name of the IAM user to create a password for. The user must already exist.</p>
    /// <p>This parameter is optional. If no user name is included, it defaults to the principal making the request. When you make this request with root user credentials, you must use an <a href="https://docs.aws.amazon.com/STS/latest/APIReference/API_AssumeRoot.html">AssumeRoot</a> session to omit the user name.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn get_user_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_user_name()
    }
    /// <p>The new password for the user.</p>
    /// <p>This parameter must be omitted when you make the request with an <a href="https://docs.aws.amazon.com/STS/latest/APIReference/API_AssumeRoot.html">AssumeRoot</a> session. It is required in all other cases.</p>
    /// <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> that is used to validate this parameter is a string of characters. That string can include almost any printable ASCII character from the space (<code>\u0020</code>) through the end of the ASCII character range (<code>\u00FF</code>). You can also include the tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and carriage return (<code>\u000D</code>) characters. Any of these characters are valid in a password. However, many tools, such as the Amazon Web Services Management Console, might restrict the ability to type certain characters because they have special meaning within that tool.</p>
    pub fn password(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.password(input.into());
        self
    }
    /// <p>The new password for the user.</p>
    /// <p>This parameter must be omitted when you make the request with an <a href="https://docs.aws.amazon.com/STS/latest/APIReference/API_AssumeRoot.html">AssumeRoot</a> session. It is required in all other cases.</p>
    /// <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> that is used to validate this parameter is a string of characters. That string can include almost any printable ASCII character from the space (<code>\u0020</code>) through the end of the ASCII character range (<code>\u00FF</code>). You can also include the tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and carriage return (<code>\u000D</code>) characters. Any of these characters are valid in a password. However, many tools, such as the Amazon Web Services Management Console, might restrict the ability to type certain characters because they have special meaning within that tool.</p>
    pub fn set_password(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_password(input);
        self
    }
    /// <p>The new password for the user.</p>
    /// <p>This parameter must be omitted when you make the request with an <a href="https://docs.aws.amazon.com/STS/latest/APIReference/API_AssumeRoot.html">AssumeRoot</a> session. It is required in all other cases.</p>
    /// <p>The <a href="http://wikipedia.org/wiki/regex">regex pattern</a> that is used to validate this parameter is a string of characters. That string can include almost any printable ASCII character from the space (<code>\u0020</code>) through the end of the ASCII character range (<code>\u00FF</code>). You can also include the tab (<code>\u0009</code>), line feed (<code>\u000A</code>), and carriage return (<code>\u000D</code>) characters. Any of these characters are valid in a password. However, many tools, such as the Amazon Web Services Management Console, might restrict the ability to type certain characters because they have special meaning within that tool.</p>
    pub fn get_password(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_password()
    }
    /// <p>Specifies whether the user is required to set a new password on next sign-in.</p>
    pub fn password_reset_required(mut self, input: bool) -> Self {
        self.inner = self.inner.password_reset_required(input);
        self
    }
    /// <p>Specifies whether the user is required to set a new password on next sign-in.</p>
    pub fn set_password_reset_required(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_password_reset_required(input);
        self
    }
    /// <p>Specifies whether the user is required to set a new password on next sign-in.</p>
    pub fn get_password_reset_required(&self) -> &::std::option::Option<bool> {
        self.inner.get_password_reset_required()
    }
}
