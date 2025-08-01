// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::associate_security_group_vpc::_associate_security_group_vpc_output::AssociateSecurityGroupVpcOutputBuilder;

pub use crate::operation::associate_security_group_vpc::_associate_security_group_vpc_input::AssociateSecurityGroupVpcInputBuilder;

impl crate::operation::associate_security_group_vpc::builders::AssociateSecurityGroupVpcInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::associate_security_group_vpc::AssociateSecurityGroupVpcOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::associate_security_group_vpc::AssociateSecurityGroupVpcError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.associate_security_group_vpc();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AssociateSecurityGroupVpc`.
///
/// <p>Associates a security group with another VPC in the same Region. This enables you to use the same security group with network interfaces and instances in the specified VPC.</p><note>
/// <ul>
/// <li>
/// <p>The VPC you want to associate the security group with must be in the same Region.</p></li>
/// <li>
/// <p>You can associate the security group with another VPC if your account owns the VPC or if the VPC was shared with you.</p></li>
/// <li>
/// <p>You must own the security group.</p></li>
/// <li>
/// <p>You cannot use this feature with default security groups.</p></li>
/// <li>
/// <p>You cannot use this feature with the default VPC.</p></li>
/// </ul>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AssociateSecurityGroupVpcFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::associate_security_group_vpc::builders::AssociateSecurityGroupVpcInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::associate_security_group_vpc::AssociateSecurityGroupVpcOutput,
        crate::operation::associate_security_group_vpc::AssociateSecurityGroupVpcError,
    > for AssociateSecurityGroupVpcFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::associate_security_group_vpc::AssociateSecurityGroupVpcOutput,
            crate::operation::associate_security_group_vpc::AssociateSecurityGroupVpcError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AssociateSecurityGroupVpcFluentBuilder {
    /// Creates a new `AssociateSecurityGroupVpcFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AssociateSecurityGroupVpc as a reference.
    pub fn as_input(&self) -> &crate::operation::associate_security_group_vpc::builders::AssociateSecurityGroupVpcInputBuilder {
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
        crate::operation::associate_security_group_vpc::AssociateSecurityGroupVpcOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::associate_security_group_vpc::AssociateSecurityGroupVpcError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::associate_security_group_vpc::AssociateSecurityGroupVpc::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::associate_security_group_vpc::AssociateSecurityGroupVpc::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::associate_security_group_vpc::AssociateSecurityGroupVpcOutput,
        crate::operation::associate_security_group_vpc::AssociateSecurityGroupVpcError,
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
    /// <p>A security group ID.</p>
    pub fn group_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.group_id(input.into());
        self
    }
    /// <p>A security group ID.</p>
    pub fn set_group_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_group_id(input);
        self
    }
    /// <p>A security group ID.</p>
    pub fn get_group_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_group_id()
    }
    /// <p>A VPC ID.</p>
    pub fn vpc_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.vpc_id(input.into());
        self
    }
    /// <p>A VPC ID.</p>
    pub fn set_vpc_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_vpc_id(input);
        self
    }
    /// <p>A VPC ID.</p>
    pub fn get_vpc_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_vpc_id()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
}
