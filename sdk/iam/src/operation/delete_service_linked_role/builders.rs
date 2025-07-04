// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_service_linked_role::_delete_service_linked_role_output::DeleteServiceLinkedRoleOutputBuilder;

pub use crate::operation::delete_service_linked_role::_delete_service_linked_role_input::DeleteServiceLinkedRoleInputBuilder;

impl crate::operation::delete_service_linked_role::builders::DeleteServiceLinkedRoleInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_service_linked_role::DeleteServiceLinkedRoleOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_service_linked_role::DeleteServiceLinkedRoleError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_service_linked_role();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteServiceLinkedRole`.
///
/// <p>Submits a service-linked role deletion request and returns a <code>DeletionTaskId</code>, which you can use to check the status of the deletion. Before you call this operation, confirm that the role has no active sessions and that any resources used by the role in the linked service are deleted. If you call this operation more than once for the same service-linked role and an earlier deletion task is not complete, then the <code>DeletionTaskId</code> of the earlier request is returned.</p>
/// <p>If you submit a deletion request for a service-linked role whose linked service is still accessing a resource, then the deletion task fails. If it fails, the <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_GetServiceLinkedRoleDeletionStatus.html">GetServiceLinkedRoleDeletionStatus</a> operation returns the reason for the failure, usually including the resources that must be deleted. To delete the service-linked role, you must first remove those resources from the linked service and then submit the deletion request again. Resources are specific to the service that is linked to the role. For more information about removing resources from a service, see the <a href="http://docs.aws.amazon.com/">Amazon Web Services documentation</a> for your service.</p>
/// <p>For more information about service-linked roles, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles_terms-and-concepts.html#iam-term-service-linked-role">Roles terms and concepts: Amazon Web Services service-linked role</a> in the <i>IAM User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteServiceLinkedRoleFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_service_linked_role::builders::DeleteServiceLinkedRoleInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_service_linked_role::DeleteServiceLinkedRoleOutput,
        crate::operation::delete_service_linked_role::DeleteServiceLinkedRoleError,
    > for DeleteServiceLinkedRoleFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_service_linked_role::DeleteServiceLinkedRoleOutput,
            crate::operation::delete_service_linked_role::DeleteServiceLinkedRoleError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteServiceLinkedRoleFluentBuilder {
    /// Creates a new `DeleteServiceLinkedRoleFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteServiceLinkedRole as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_service_linked_role::builders::DeleteServiceLinkedRoleInputBuilder {
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
        crate::operation::delete_service_linked_role::DeleteServiceLinkedRoleOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_service_linked_role::DeleteServiceLinkedRoleError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_service_linked_role::DeleteServiceLinkedRole::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_service_linked_role::DeleteServiceLinkedRole::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_service_linked_role::DeleteServiceLinkedRoleOutput,
        crate::operation::delete_service_linked_role::DeleteServiceLinkedRoleError,
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
    /// <p>The name of the service-linked role to be deleted.</p>
    pub fn role_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.role_name(input.into());
        self
    }
    /// <p>The name of the service-linked role to be deleted.</p>
    pub fn set_role_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_role_name(input);
        self
    }
    /// <p>The name of the service-linked role to be deleted.</p>
    pub fn get_role_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_role_name()
    }
}
