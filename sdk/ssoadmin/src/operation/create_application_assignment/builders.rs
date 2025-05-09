// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_application_assignment::_create_application_assignment_output::CreateApplicationAssignmentOutputBuilder;

pub use crate::operation::create_application_assignment::_create_application_assignment_input::CreateApplicationAssignmentInputBuilder;

impl crate::operation::create_application_assignment::builders::CreateApplicationAssignmentInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_application_assignment::CreateApplicationAssignmentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_application_assignment::CreateApplicationAssignmentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_application_assignment();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateApplicationAssignment`.
///
/// <p>Grant application access to a user or group.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateApplicationAssignmentFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_application_assignment::builders::CreateApplicationAssignmentInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_application_assignment::CreateApplicationAssignmentOutput,
        crate::operation::create_application_assignment::CreateApplicationAssignmentError,
    > for CreateApplicationAssignmentFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_application_assignment::CreateApplicationAssignmentOutput,
            crate::operation::create_application_assignment::CreateApplicationAssignmentError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateApplicationAssignmentFluentBuilder {
    /// Creates a new `CreateApplicationAssignmentFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateApplicationAssignment as a reference.
    pub fn as_input(&self) -> &crate::operation::create_application_assignment::builders::CreateApplicationAssignmentInputBuilder {
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
        crate::operation::create_application_assignment::CreateApplicationAssignmentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_application_assignment::CreateApplicationAssignmentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_application_assignment::CreateApplicationAssignment::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_application_assignment::CreateApplicationAssignment::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_application_assignment::CreateApplicationAssignmentOutput,
        crate::operation::create_application_assignment::CreateApplicationAssignmentError,
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
    /// <p>The ARN of the application for which the assignment is created.</p>
    pub fn application_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.application_arn(input.into());
        self
    }
    /// <p>The ARN of the application for which the assignment is created.</p>
    pub fn set_application_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_application_arn(input);
        self
    }
    /// <p>The ARN of the application for which the assignment is created.</p>
    pub fn get_application_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_application_arn()
    }
    /// <p>An identifier for an object in IAM Identity Center, such as a user or group. PrincipalIds are GUIDs (For example, f81d4fae-7dec-11d0-a765-00a0c91e6bf6). For more information about PrincipalIds in IAM Identity Center, see the <a href="https://docs.aws.amazon.com/singlesignon/latest/IdentityStoreAPIReference/welcome.html">IAM Identity Center Identity Store API Reference</a>.</p>
    pub fn principal_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.principal_id(input.into());
        self
    }
    /// <p>An identifier for an object in IAM Identity Center, such as a user or group. PrincipalIds are GUIDs (For example, f81d4fae-7dec-11d0-a765-00a0c91e6bf6). For more information about PrincipalIds in IAM Identity Center, see the <a href="https://docs.aws.amazon.com/singlesignon/latest/IdentityStoreAPIReference/welcome.html">IAM Identity Center Identity Store API Reference</a>.</p>
    pub fn set_principal_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_principal_id(input);
        self
    }
    /// <p>An identifier for an object in IAM Identity Center, such as a user or group. PrincipalIds are GUIDs (For example, f81d4fae-7dec-11d0-a765-00a0c91e6bf6). For more information about PrincipalIds in IAM Identity Center, see the <a href="https://docs.aws.amazon.com/singlesignon/latest/IdentityStoreAPIReference/welcome.html">IAM Identity Center Identity Store API Reference</a>.</p>
    pub fn get_principal_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_principal_id()
    }
    /// <p>The entity type for which the assignment will be created.</p>
    pub fn principal_type(mut self, input: crate::types::PrincipalType) -> Self {
        self.inner = self.inner.principal_type(input);
        self
    }
    /// <p>The entity type for which the assignment will be created.</p>
    pub fn set_principal_type(mut self, input: ::std::option::Option<crate::types::PrincipalType>) -> Self {
        self.inner = self.inner.set_principal_type(input);
        self
    }
    /// <p>The entity type for which the assignment will be created.</p>
    pub fn get_principal_type(&self) -> &::std::option::Option<crate::types::PrincipalType> {
        self.inner.get_principal_type()
    }
}
