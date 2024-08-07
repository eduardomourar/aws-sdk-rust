// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the options for a VPC attachment.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateTransitGatewayVpcAttachmentRequestOptions {
    /// <p>Enable or disable DNS support. The default is <code>enable</code>.</p>
    pub dns_support: ::std::option::Option<crate::types::DnsSupportValue>,
    /// <note>
    /// <p>This parameter is in preview and may not be available for your account.</p>
    /// </note>
    /// <p>Enables you to reference a security group across VPCs attached to a transit gateway. Use this option to simplify security group management and control of instance-to-instance traffic across VPCs that are connected by transit gateway. You can also use this option to migrate from VPC peering (which was the only option that supported security group referencing) to transit gateways (which now also support security group referencing). This option is disabled by default and there are no additional costs to use this feature.</p>
    /// <p>If you don't enable or disable SecurityGroupReferencingSupport in the request, the attachment will inherit the security group referencing support setting on the transit gateway.</p>
    pub security_group_referencing_support: ::std::option::Option<crate::types::SecurityGroupReferencingSupportValue>,
    /// <p>Enable or disable IPv6 support. The default is <code>disable</code>.</p>
    pub ipv6_support: ::std::option::Option<crate::types::Ipv6SupportValue>,
    /// <p>Enable or disable support for appliance mode. If enabled, a traffic flow between a source and destination uses the same Availability Zone for the VPC attachment for the lifetime of that flow. The default is <code>disable</code>.</p>
    pub appliance_mode_support: ::std::option::Option<crate::types::ApplianceModeSupportValue>,
}
impl CreateTransitGatewayVpcAttachmentRequestOptions {
    /// <p>Enable or disable DNS support. The default is <code>enable</code>.</p>
    pub fn dns_support(&self) -> ::std::option::Option<&crate::types::DnsSupportValue> {
        self.dns_support.as_ref()
    }
    /// <note>
    /// <p>This parameter is in preview and may not be available for your account.</p>
    /// </note>
    /// <p>Enables you to reference a security group across VPCs attached to a transit gateway. Use this option to simplify security group management and control of instance-to-instance traffic across VPCs that are connected by transit gateway. You can also use this option to migrate from VPC peering (which was the only option that supported security group referencing) to transit gateways (which now also support security group referencing). This option is disabled by default and there are no additional costs to use this feature.</p>
    /// <p>If you don't enable or disable SecurityGroupReferencingSupport in the request, the attachment will inherit the security group referencing support setting on the transit gateway.</p>
    pub fn security_group_referencing_support(&self) -> ::std::option::Option<&crate::types::SecurityGroupReferencingSupportValue> {
        self.security_group_referencing_support.as_ref()
    }
    /// <p>Enable or disable IPv6 support. The default is <code>disable</code>.</p>
    pub fn ipv6_support(&self) -> ::std::option::Option<&crate::types::Ipv6SupportValue> {
        self.ipv6_support.as_ref()
    }
    /// <p>Enable or disable support for appliance mode. If enabled, a traffic flow between a source and destination uses the same Availability Zone for the VPC attachment for the lifetime of that flow. The default is <code>disable</code>.</p>
    pub fn appliance_mode_support(&self) -> ::std::option::Option<&crate::types::ApplianceModeSupportValue> {
        self.appliance_mode_support.as_ref()
    }
}
impl CreateTransitGatewayVpcAttachmentRequestOptions {
    /// Creates a new builder-style object to manufacture [`CreateTransitGatewayVpcAttachmentRequestOptions`](crate::types::CreateTransitGatewayVpcAttachmentRequestOptions).
    pub fn builder() -> crate::types::builders::CreateTransitGatewayVpcAttachmentRequestOptionsBuilder {
        crate::types::builders::CreateTransitGatewayVpcAttachmentRequestOptionsBuilder::default()
    }
}

/// A builder for [`CreateTransitGatewayVpcAttachmentRequestOptions`](crate::types::CreateTransitGatewayVpcAttachmentRequestOptions).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct CreateTransitGatewayVpcAttachmentRequestOptionsBuilder {
    pub(crate) dns_support: ::std::option::Option<crate::types::DnsSupportValue>,
    pub(crate) security_group_referencing_support: ::std::option::Option<crate::types::SecurityGroupReferencingSupportValue>,
    pub(crate) ipv6_support: ::std::option::Option<crate::types::Ipv6SupportValue>,
    pub(crate) appliance_mode_support: ::std::option::Option<crate::types::ApplianceModeSupportValue>,
}
impl CreateTransitGatewayVpcAttachmentRequestOptionsBuilder {
    /// <p>Enable or disable DNS support. The default is <code>enable</code>.</p>
    pub fn dns_support(mut self, input: crate::types::DnsSupportValue) -> Self {
        self.dns_support = ::std::option::Option::Some(input);
        self
    }
    /// <p>Enable or disable DNS support. The default is <code>enable</code>.</p>
    pub fn set_dns_support(mut self, input: ::std::option::Option<crate::types::DnsSupportValue>) -> Self {
        self.dns_support = input;
        self
    }
    /// <p>Enable or disable DNS support. The default is <code>enable</code>.</p>
    pub fn get_dns_support(&self) -> &::std::option::Option<crate::types::DnsSupportValue> {
        &self.dns_support
    }
    /// <note>
    /// <p>This parameter is in preview and may not be available for your account.</p>
    /// </note>
    /// <p>Enables you to reference a security group across VPCs attached to a transit gateway. Use this option to simplify security group management and control of instance-to-instance traffic across VPCs that are connected by transit gateway. You can also use this option to migrate from VPC peering (which was the only option that supported security group referencing) to transit gateways (which now also support security group referencing). This option is disabled by default and there are no additional costs to use this feature.</p>
    /// <p>If you don't enable or disable SecurityGroupReferencingSupport in the request, the attachment will inherit the security group referencing support setting on the transit gateway.</p>
    pub fn security_group_referencing_support(mut self, input: crate::types::SecurityGroupReferencingSupportValue) -> Self {
        self.security_group_referencing_support = ::std::option::Option::Some(input);
        self
    }
    /// <note>
    /// <p>This parameter is in preview and may not be available for your account.</p>
    /// </note>
    /// <p>Enables you to reference a security group across VPCs attached to a transit gateway. Use this option to simplify security group management and control of instance-to-instance traffic across VPCs that are connected by transit gateway. You can also use this option to migrate from VPC peering (which was the only option that supported security group referencing) to transit gateways (which now also support security group referencing). This option is disabled by default and there are no additional costs to use this feature.</p>
    /// <p>If you don't enable or disable SecurityGroupReferencingSupport in the request, the attachment will inherit the security group referencing support setting on the transit gateway.</p>
    pub fn set_security_group_referencing_support(
        mut self,
        input: ::std::option::Option<crate::types::SecurityGroupReferencingSupportValue>,
    ) -> Self {
        self.security_group_referencing_support = input;
        self
    }
    /// <note>
    /// <p>This parameter is in preview and may not be available for your account.</p>
    /// </note>
    /// <p>Enables you to reference a security group across VPCs attached to a transit gateway. Use this option to simplify security group management and control of instance-to-instance traffic across VPCs that are connected by transit gateway. You can also use this option to migrate from VPC peering (which was the only option that supported security group referencing) to transit gateways (which now also support security group referencing). This option is disabled by default and there are no additional costs to use this feature.</p>
    /// <p>If you don't enable or disable SecurityGroupReferencingSupport in the request, the attachment will inherit the security group referencing support setting on the transit gateway.</p>
    pub fn get_security_group_referencing_support(&self) -> &::std::option::Option<crate::types::SecurityGroupReferencingSupportValue> {
        &self.security_group_referencing_support
    }
    /// <p>Enable or disable IPv6 support. The default is <code>disable</code>.</p>
    pub fn ipv6_support(mut self, input: crate::types::Ipv6SupportValue) -> Self {
        self.ipv6_support = ::std::option::Option::Some(input);
        self
    }
    /// <p>Enable or disable IPv6 support. The default is <code>disable</code>.</p>
    pub fn set_ipv6_support(mut self, input: ::std::option::Option<crate::types::Ipv6SupportValue>) -> Self {
        self.ipv6_support = input;
        self
    }
    /// <p>Enable or disable IPv6 support. The default is <code>disable</code>.</p>
    pub fn get_ipv6_support(&self) -> &::std::option::Option<crate::types::Ipv6SupportValue> {
        &self.ipv6_support
    }
    /// <p>Enable or disable support for appliance mode. If enabled, a traffic flow between a source and destination uses the same Availability Zone for the VPC attachment for the lifetime of that flow. The default is <code>disable</code>.</p>
    pub fn appliance_mode_support(mut self, input: crate::types::ApplianceModeSupportValue) -> Self {
        self.appliance_mode_support = ::std::option::Option::Some(input);
        self
    }
    /// <p>Enable or disable support for appliance mode. If enabled, a traffic flow between a source and destination uses the same Availability Zone for the VPC attachment for the lifetime of that flow. The default is <code>disable</code>.</p>
    pub fn set_appliance_mode_support(mut self, input: ::std::option::Option<crate::types::ApplianceModeSupportValue>) -> Self {
        self.appliance_mode_support = input;
        self
    }
    /// <p>Enable or disable support for appliance mode. If enabled, a traffic flow between a source and destination uses the same Availability Zone for the VPC attachment for the lifetime of that flow. The default is <code>disable</code>.</p>
    pub fn get_appliance_mode_support(&self) -> &::std::option::Option<crate::types::ApplianceModeSupportValue> {
        &self.appliance_mode_support
    }
    /// Consumes the builder and constructs a [`CreateTransitGatewayVpcAttachmentRequestOptions`](crate::types::CreateTransitGatewayVpcAttachmentRequestOptions).
    pub fn build(self) -> crate::types::CreateTransitGatewayVpcAttachmentRequestOptions {
        crate::types::CreateTransitGatewayVpcAttachmentRequestOptions {
            dns_support: self.dns_support,
            security_group_referencing_support: self.security_group_referencing_support,
            ipv6_support: self.ipv6_support,
            appliance_mode_support: self.appliance_mode_support,
        }
    }
}
