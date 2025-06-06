// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_local_gateway_virtual_interface_group::_create_local_gateway_virtual_interface_group_output::CreateLocalGatewayVirtualInterfaceGroupOutputBuilder;

pub use crate::operation::create_local_gateway_virtual_interface_group::_create_local_gateway_virtual_interface_group_input::CreateLocalGatewayVirtualInterfaceGroupInputBuilder;

impl crate::operation::create_local_gateway_virtual_interface_group::builders::CreateLocalGatewayVirtualInterfaceGroupInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_local_gateway_virtual_interface_group::CreateLocalGatewayVirtualInterfaceGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_local_gateway_virtual_interface_group::CreateLocalGatewayVirtualInterfaceGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_local_gateway_virtual_interface_group();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateLocalGatewayVirtualInterfaceGroup`.
///
/// <p>Create a local gateway virtual interface group.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateLocalGatewayVirtualInterfaceGroupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_local_gateway_virtual_interface_group::builders::CreateLocalGatewayVirtualInterfaceGroupInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_local_gateway_virtual_interface_group::CreateLocalGatewayVirtualInterfaceGroupOutput,
        crate::operation::create_local_gateway_virtual_interface_group::CreateLocalGatewayVirtualInterfaceGroupError,
    > for CreateLocalGatewayVirtualInterfaceGroupFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_local_gateway_virtual_interface_group::CreateLocalGatewayVirtualInterfaceGroupOutput,
            crate::operation::create_local_gateway_virtual_interface_group::CreateLocalGatewayVirtualInterfaceGroupError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateLocalGatewayVirtualInterfaceGroupFluentBuilder {
    /// Creates a new `CreateLocalGatewayVirtualInterfaceGroupFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateLocalGatewayVirtualInterfaceGroup as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::create_local_gateway_virtual_interface_group::builders::CreateLocalGatewayVirtualInterfaceGroupInputBuilder {
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
        crate::operation::create_local_gateway_virtual_interface_group::CreateLocalGatewayVirtualInterfaceGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_local_gateway_virtual_interface_group::CreateLocalGatewayVirtualInterfaceGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::create_local_gateway_virtual_interface_group::CreateLocalGatewayVirtualInterfaceGroup::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::create_local_gateway_virtual_interface_group::CreateLocalGatewayVirtualInterfaceGroup::orchestrate(&runtime_plugins, input)
            .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_local_gateway_virtual_interface_group::CreateLocalGatewayVirtualInterfaceGroupOutput,
        crate::operation::create_local_gateway_virtual_interface_group::CreateLocalGatewayVirtualInterfaceGroupError,
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
    /// <p>The ID of the local gateway.</p>
    pub fn local_gateway_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.local_gateway_id(input.into());
        self
    }
    /// <p>The ID of the local gateway.</p>
    pub fn set_local_gateway_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_local_gateway_id(input);
        self
    }
    /// <p>The ID of the local gateway.</p>
    pub fn get_local_gateway_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_local_gateway_id()
    }
    /// <p>The Autonomous System Number(ASN) for the local Border Gateway Protocol (BGP).</p>
    pub fn local_bgp_asn(mut self, input: i32) -> Self {
        self.inner = self.inner.local_bgp_asn(input);
        self
    }
    /// <p>The Autonomous System Number(ASN) for the local Border Gateway Protocol (BGP).</p>
    pub fn set_local_bgp_asn(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_local_bgp_asn(input);
        self
    }
    /// <p>The Autonomous System Number(ASN) for the local Border Gateway Protocol (BGP).</p>
    pub fn get_local_bgp_asn(&self) -> &::std::option::Option<i32> {
        self.inner.get_local_bgp_asn()
    }
    /// <p>The extended 32-bit ASN for the local BGP configuration.</p>
    pub fn local_bgp_asn_extended(mut self, input: i64) -> Self {
        self.inner = self.inner.local_bgp_asn_extended(input);
        self
    }
    /// <p>The extended 32-bit ASN for the local BGP configuration.</p>
    pub fn set_local_bgp_asn_extended(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_local_bgp_asn_extended(input);
        self
    }
    /// <p>The extended 32-bit ASN for the local BGP configuration.</p>
    pub fn get_local_bgp_asn_extended(&self) -> &::std::option::Option<i64> {
        self.inner.get_local_bgp_asn_extended()
    }
    ///
    /// Appends an item to `TagSpecifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to apply to the local gateway virtual interface group when the resource is being created.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        self.inner = self.inner.tag_specifications(input);
        self
    }
    /// <p>The tags to apply to the local gateway virtual interface group when the resource is being created.</p>
    pub fn set_tag_specifications(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>) -> Self {
        self.inner = self.inner.set_tag_specifications(input);
        self
    }
    /// <p>The tags to apply to the local gateway virtual interface group when the resource is being created.</p>
    pub fn get_tag_specifications(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>> {
        self.inner.get_tag_specifications()
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
