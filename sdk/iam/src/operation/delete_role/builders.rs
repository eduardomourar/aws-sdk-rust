// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_role::_delete_role_output::DeleteRoleOutputBuilder;

pub use crate::operation::delete_role::_delete_role_input::DeleteRoleInputBuilder;

impl crate::operation::delete_role::builders::DeleteRoleInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_role::DeleteRoleOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_role::DeleteRoleError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_role();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteRole`.
///
/// <p>Deletes the specified role. Unlike the Amazon Web Services Management Console, when you delete a role programmatically, you must delete the items attached to the role manually, or the deletion fails. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_manage_delete.html#roles-managingrole-deleting-cli">Deleting an IAM role</a>. Before attempting to delete a role, remove the following attached items:</p>
/// <ul>
/// <li>
/// <p>Inline policies (<a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_DeleteRolePolicy.html">DeleteRolePolicy</a>)</p></li>
/// <li>
/// <p>Attached managed policies (<a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_DetachRolePolicy.html">DetachRolePolicy</a>)</p></li>
/// <li>
/// <p>Instance profile (<a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_RemoveRoleFromInstanceProfile.html">RemoveRoleFromInstanceProfile</a>)</p></li>
/// <li>
/// <p>Optional – Delete instance profile after detaching from role for resource clean up (<a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_DeleteInstanceProfile.html">DeleteInstanceProfile</a>)</p></li>
/// </ul><important>
/// <p>Make sure that you do not have any Amazon EC2 instances running with the role you are about to delete. Deleting a role or instance profile that is associated with a running instance will break any applications running on the instance.</p>
/// </important>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteRoleFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_role::builders::DeleteRoleInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_role::DeleteRoleOutput,
        crate::operation::delete_role::DeleteRoleError,
    > for DeleteRoleFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_role::DeleteRoleOutput,
            crate::operation::delete_role::DeleteRoleError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteRoleFluentBuilder {
    /// Creates a new `DeleteRoleFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteRole as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_role::builders::DeleteRoleInputBuilder {
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
        crate::operation::delete_role::DeleteRoleOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_role::DeleteRoleError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_role::DeleteRole::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_role::DeleteRole::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_role::DeleteRoleOutput,
        crate::operation::delete_role::DeleteRoleError,
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
    /// <p>The name of the role to delete.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn role_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.role_name(input.into());
        self
    }
    /// <p>The name of the role to delete.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn set_role_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_role_name(input);
        self
    }
    /// <p>The name of the role to delete.</p>
    /// <p>This parameter allows (through its <a href="http://wikipedia.org/wiki/regex">regex pattern</a>) a string of characters consisting of upper and lowercase alphanumeric characters with no spaces. You can also include any of the following characters: _+=,.@-</p>
    pub fn get_role_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_role_name()
    }
}
