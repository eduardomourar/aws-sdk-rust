// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::provision_byoip_cidr::_provision_byoip_cidr_output::ProvisionByoipCidrOutputBuilder;

pub use crate::operation::provision_byoip_cidr::_provision_byoip_cidr_input::ProvisionByoipCidrInputBuilder;

impl crate::operation::provision_byoip_cidr::builders::ProvisionByoipCidrInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::provision_byoip_cidr::ProvisionByoipCidrOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::provision_byoip_cidr::ProvisionByoipCidrError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.provision_byoip_cidr();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ProvisionByoipCidr`.
///
/// <p>Provisions an IPv4 or IPv6 address range for use with your Amazon Web Services resources through bring your own IP addresses (BYOIP) and creates a corresponding address pool. After the address range is provisioned, it is ready to be advertised.</p>
/// <p>Amazon Web Services verifies that you own the address range and are authorized to advertise it. You must ensure that the address range is registered to you and that you created an RPKI ROA to authorize Amazon ASNs 16509 and 14618 to advertise the address range. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-byoip.html">Bring your own IP addresses (BYOIP)</a> in the <i>Amazon EC2 User Guide</i>.</p>
/// <p>Provisioning an address range is an asynchronous operation, so the call returns immediately, but the address range is not ready to use until its status changes from <code>pending-provision</code> to <code>provisioned</code>. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/byoip-onboard.html">Onboard your address range</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ProvisionByoipCidrFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::provision_byoip_cidr::builders::ProvisionByoipCidrInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::provision_byoip_cidr::ProvisionByoipCidrOutput,
        crate::operation::provision_byoip_cidr::ProvisionByoipCidrError,
    > for ProvisionByoipCidrFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::provision_byoip_cidr::ProvisionByoipCidrOutput,
            crate::operation::provision_byoip_cidr::ProvisionByoipCidrError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ProvisionByoipCidrFluentBuilder {
    /// Creates a new `ProvisionByoipCidrFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ProvisionByoipCidr as a reference.
    pub fn as_input(&self) -> &crate::operation::provision_byoip_cidr::builders::ProvisionByoipCidrInputBuilder {
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
        crate::operation::provision_byoip_cidr::ProvisionByoipCidrOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::provision_byoip_cidr::ProvisionByoipCidrError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::provision_byoip_cidr::ProvisionByoipCidr::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::provision_byoip_cidr::ProvisionByoipCidr::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::provision_byoip_cidr::ProvisionByoipCidrOutput,
        crate::operation::provision_byoip_cidr::ProvisionByoipCidrError,
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
    /// <p>The public IPv4 or IPv6 address range, in CIDR notation. The most specific IPv4 prefix that you can specify is /24. The most specific IPv6 address range that you can bring is /48 for CIDRs that are publicly advertisable and /56 for CIDRs that are not publicly advertisable. The address range cannot overlap with another address range that you've brought to this or another Region.</p>
    pub fn cidr(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.cidr(input.into());
        self
    }
    /// <p>The public IPv4 or IPv6 address range, in CIDR notation. The most specific IPv4 prefix that you can specify is /24. The most specific IPv6 address range that you can bring is /48 for CIDRs that are publicly advertisable and /56 for CIDRs that are not publicly advertisable. The address range cannot overlap with another address range that you've brought to this or another Region.</p>
    pub fn set_cidr(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_cidr(input);
        self
    }
    /// <p>The public IPv4 or IPv6 address range, in CIDR notation. The most specific IPv4 prefix that you can specify is /24. The most specific IPv6 address range that you can bring is /48 for CIDRs that are publicly advertisable and /56 for CIDRs that are not publicly advertisable. The address range cannot overlap with another address range that you've brought to this or another Region.</p>
    pub fn get_cidr(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_cidr()
    }
    /// <p>A signed document that proves that you are authorized to bring the specified IP address range to Amazon using BYOIP.</p>
    pub fn cidr_authorization_context(mut self, input: crate::types::CidrAuthorizationContext) -> Self {
        self.inner = self.inner.cidr_authorization_context(input);
        self
    }
    /// <p>A signed document that proves that you are authorized to bring the specified IP address range to Amazon using BYOIP.</p>
    pub fn set_cidr_authorization_context(mut self, input: ::std::option::Option<crate::types::CidrAuthorizationContext>) -> Self {
        self.inner = self.inner.set_cidr_authorization_context(input);
        self
    }
    /// <p>A signed document that proves that you are authorized to bring the specified IP address range to Amazon using BYOIP.</p>
    pub fn get_cidr_authorization_context(&self) -> &::std::option::Option<crate::types::CidrAuthorizationContext> {
        self.inner.get_cidr_authorization_context()
    }
    /// <p>(IPv6 only) Indicate whether the address range will be publicly advertised to the internet.</p>
    /// <p>Default: true</p>
    pub fn publicly_advertisable(mut self, input: bool) -> Self {
        self.inner = self.inner.publicly_advertisable(input);
        self
    }
    /// <p>(IPv6 only) Indicate whether the address range will be publicly advertised to the internet.</p>
    /// <p>Default: true</p>
    pub fn set_publicly_advertisable(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_publicly_advertisable(input);
        self
    }
    /// <p>(IPv6 only) Indicate whether the address range will be publicly advertised to the internet.</p>
    /// <p>Default: true</p>
    pub fn get_publicly_advertisable(&self) -> &::std::option::Option<bool> {
        self.inner.get_publicly_advertisable()
    }
    /// <p>A description for the address range and the address pool.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description for the address range and the address pool.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>A description for the address range and the address pool.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
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
    ///
    /// Appends an item to `PoolTagSpecifications`.
    ///
    /// To override the contents of this collection use [`set_pool_tag_specifications`](Self::set_pool_tag_specifications).
    ///
    /// <p>The tags to apply to the address pool.</p>
    pub fn pool_tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        self.inner = self.inner.pool_tag_specifications(input);
        self
    }
    /// <p>The tags to apply to the address pool.</p>
    pub fn set_pool_tag_specifications(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>) -> Self {
        self.inner = self.inner.set_pool_tag_specifications(input);
        self
    }
    /// <p>The tags to apply to the address pool.</p>
    pub fn get_pool_tag_specifications(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>> {
        self.inner.get_pool_tag_specifications()
    }
    /// <p>Reserved.</p>
    pub fn multi_region(mut self, input: bool) -> Self {
        self.inner = self.inner.multi_region(input);
        self
    }
    /// <p>Reserved.</p>
    pub fn set_multi_region(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_multi_region(input);
        self
    }
    /// <p>Reserved.</p>
    pub fn get_multi_region(&self) -> &::std::option::Option<bool> {
        self.inner.get_multi_region()
    }
    /// <p>If you have <a href="https://docs.aws.amazon.com/local-zones/latest/ug/how-local-zones-work.html">Local Zones</a> enabled, you can choose a network border group for Local Zones when you provision and advertise a BYOIPv4 CIDR. Choose the network border group carefully as the EIP and the Amazon Web Services resource it is associated with must reside in the same network border group.</p>
    /// <p>You can provision BYOIP address ranges to and advertise them in the following Local Zone network border groups:</p>
    /// <ul>
    /// <li>
    /// <p>us-east-1-dfw-2</p></li>
    /// <li>
    /// <p>us-west-2-lax-1</p></li>
    /// <li>
    /// <p>us-west-2-phx-2</p></li>
    /// </ul><note>
    /// <p>You cannot provision or advertise BYOIPv6 address ranges in Local Zones at this time.</p>
    /// </note>
    pub fn network_border_group(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.network_border_group(input.into());
        self
    }
    /// <p>If you have <a href="https://docs.aws.amazon.com/local-zones/latest/ug/how-local-zones-work.html">Local Zones</a> enabled, you can choose a network border group for Local Zones when you provision and advertise a BYOIPv4 CIDR. Choose the network border group carefully as the EIP and the Amazon Web Services resource it is associated with must reside in the same network border group.</p>
    /// <p>You can provision BYOIP address ranges to and advertise them in the following Local Zone network border groups:</p>
    /// <ul>
    /// <li>
    /// <p>us-east-1-dfw-2</p></li>
    /// <li>
    /// <p>us-west-2-lax-1</p></li>
    /// <li>
    /// <p>us-west-2-phx-2</p></li>
    /// </ul><note>
    /// <p>You cannot provision or advertise BYOIPv6 address ranges in Local Zones at this time.</p>
    /// </note>
    pub fn set_network_border_group(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_network_border_group(input);
        self
    }
    /// <p>If you have <a href="https://docs.aws.amazon.com/local-zones/latest/ug/how-local-zones-work.html">Local Zones</a> enabled, you can choose a network border group for Local Zones when you provision and advertise a BYOIPv4 CIDR. Choose the network border group carefully as the EIP and the Amazon Web Services resource it is associated with must reside in the same network border group.</p>
    /// <p>You can provision BYOIP address ranges to and advertise them in the following Local Zone network border groups:</p>
    /// <ul>
    /// <li>
    /// <p>us-east-1-dfw-2</p></li>
    /// <li>
    /// <p>us-west-2-lax-1</p></li>
    /// <li>
    /// <p>us-west-2-phx-2</p></li>
    /// </ul><note>
    /// <p>You cannot provision or advertise BYOIPv6 address ranges in Local Zones at this time.</p>
    /// </note>
    pub fn get_network_border_group(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_network_border_group()
    }
}
