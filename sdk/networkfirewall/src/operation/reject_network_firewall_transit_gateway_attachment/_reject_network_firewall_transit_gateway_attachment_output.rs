// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RejectNetworkFirewallTransitGatewayAttachmentOutput {
    /// <p>The unique identifier of the transit gateway attachment that was rejected.</p>
    pub transit_gateway_attachment_id: ::std::string::String,
    /// <p>The current status of the transit gateway attachment. Valid values are:</p>
    /// <ul>
    /// <li>
    /// <p><code>CREATING</code> - The attachment is being created</p></li>
    /// <li>
    /// <p><code>DELETING</code> - The attachment is being deleted</p></li>
    /// <li>
    /// <p><code>DELETED</code> - The attachment has been deleted</p></li>
    /// <li>
    /// <p><code>FAILED</code> - The attachment creation has failed and cannot be recovered</p></li>
    /// <li>
    /// <p><code>ERROR</code> - The attachment is in an error state that might be recoverable</p></li>
    /// <li>
    /// <p><code>READY</code> - The attachment is active and processing traffic</p></li>
    /// <li>
    /// <p><code>PENDING_ACCEPTANCE</code> - The attachment is waiting to be accepted</p></li>
    /// <li>
    /// <p><code>REJECTING</code> - The attachment is in the process of being rejected</p></li>
    /// <li>
    /// <p><code>REJECTED</code> - The attachment has been rejected</p></li>
    /// </ul>
    /// <p>For information about troubleshooting endpoint failures, see <a href="https://docs.aws.amazon.com/network-firewall/latest/developerguide/firewall-troubleshooting-endpoint-failures.html">Troubleshooting firewall endpoint failures</a> in the <i>Network Firewall Developer Guide</i>.</p>
    pub transit_gateway_attachment_status: crate::types::TransitGatewayAttachmentStatus,
    _request_id: Option<String>,
}
impl RejectNetworkFirewallTransitGatewayAttachmentOutput {
    /// <p>The unique identifier of the transit gateway attachment that was rejected.</p>
    pub fn transit_gateway_attachment_id(&self) -> &str {
        use std::ops::Deref;
        self.transit_gateway_attachment_id.deref()
    }
    /// <p>The current status of the transit gateway attachment. Valid values are:</p>
    /// <ul>
    /// <li>
    /// <p><code>CREATING</code> - The attachment is being created</p></li>
    /// <li>
    /// <p><code>DELETING</code> - The attachment is being deleted</p></li>
    /// <li>
    /// <p><code>DELETED</code> - The attachment has been deleted</p></li>
    /// <li>
    /// <p><code>FAILED</code> - The attachment creation has failed and cannot be recovered</p></li>
    /// <li>
    /// <p><code>ERROR</code> - The attachment is in an error state that might be recoverable</p></li>
    /// <li>
    /// <p><code>READY</code> - The attachment is active and processing traffic</p></li>
    /// <li>
    /// <p><code>PENDING_ACCEPTANCE</code> - The attachment is waiting to be accepted</p></li>
    /// <li>
    /// <p><code>REJECTING</code> - The attachment is in the process of being rejected</p></li>
    /// <li>
    /// <p><code>REJECTED</code> - The attachment has been rejected</p></li>
    /// </ul>
    /// <p>For information about troubleshooting endpoint failures, see <a href="https://docs.aws.amazon.com/network-firewall/latest/developerguide/firewall-troubleshooting-endpoint-failures.html">Troubleshooting firewall endpoint failures</a> in the <i>Network Firewall Developer Guide</i>.</p>
    pub fn transit_gateway_attachment_status(&self) -> &crate::types::TransitGatewayAttachmentStatus {
        &self.transit_gateway_attachment_status
    }
}
impl ::aws_types::request_id::RequestId for RejectNetworkFirewallTransitGatewayAttachmentOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl RejectNetworkFirewallTransitGatewayAttachmentOutput {
    /// Creates a new builder-style object to manufacture [`RejectNetworkFirewallTransitGatewayAttachmentOutput`](crate::operation::reject_network_firewall_transit_gateway_attachment::RejectNetworkFirewallTransitGatewayAttachmentOutput).
    pub fn builder(
    ) -> crate::operation::reject_network_firewall_transit_gateway_attachment::builders::RejectNetworkFirewallTransitGatewayAttachmentOutputBuilder
    {
        crate::operation::reject_network_firewall_transit_gateway_attachment::builders::RejectNetworkFirewallTransitGatewayAttachmentOutputBuilder::default()
    }
}

/// A builder for [`RejectNetworkFirewallTransitGatewayAttachmentOutput`](crate::operation::reject_network_firewall_transit_gateway_attachment::RejectNetworkFirewallTransitGatewayAttachmentOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct RejectNetworkFirewallTransitGatewayAttachmentOutputBuilder {
    pub(crate) transit_gateway_attachment_id: ::std::option::Option<::std::string::String>,
    pub(crate) transit_gateway_attachment_status: ::std::option::Option<crate::types::TransitGatewayAttachmentStatus>,
    _request_id: Option<String>,
}
impl RejectNetworkFirewallTransitGatewayAttachmentOutputBuilder {
    /// <p>The unique identifier of the transit gateway attachment that was rejected.</p>
    /// This field is required.
    pub fn transit_gateway_attachment_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.transit_gateway_attachment_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier of the transit gateway attachment that was rejected.</p>
    pub fn set_transit_gateway_attachment_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.transit_gateway_attachment_id = input;
        self
    }
    /// <p>The unique identifier of the transit gateway attachment that was rejected.</p>
    pub fn get_transit_gateway_attachment_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.transit_gateway_attachment_id
    }
    /// <p>The current status of the transit gateway attachment. Valid values are:</p>
    /// <ul>
    /// <li>
    /// <p><code>CREATING</code> - The attachment is being created</p></li>
    /// <li>
    /// <p><code>DELETING</code> - The attachment is being deleted</p></li>
    /// <li>
    /// <p><code>DELETED</code> - The attachment has been deleted</p></li>
    /// <li>
    /// <p><code>FAILED</code> - The attachment creation has failed and cannot be recovered</p></li>
    /// <li>
    /// <p><code>ERROR</code> - The attachment is in an error state that might be recoverable</p></li>
    /// <li>
    /// <p><code>READY</code> - The attachment is active and processing traffic</p></li>
    /// <li>
    /// <p><code>PENDING_ACCEPTANCE</code> - The attachment is waiting to be accepted</p></li>
    /// <li>
    /// <p><code>REJECTING</code> - The attachment is in the process of being rejected</p></li>
    /// <li>
    /// <p><code>REJECTED</code> - The attachment has been rejected</p></li>
    /// </ul>
    /// <p>For information about troubleshooting endpoint failures, see <a href="https://docs.aws.amazon.com/network-firewall/latest/developerguide/firewall-troubleshooting-endpoint-failures.html">Troubleshooting firewall endpoint failures</a> in the <i>Network Firewall Developer Guide</i>.</p>
    /// This field is required.
    pub fn transit_gateway_attachment_status(mut self, input: crate::types::TransitGatewayAttachmentStatus) -> Self {
        self.transit_gateway_attachment_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The current status of the transit gateway attachment. Valid values are:</p>
    /// <ul>
    /// <li>
    /// <p><code>CREATING</code> - The attachment is being created</p></li>
    /// <li>
    /// <p><code>DELETING</code> - The attachment is being deleted</p></li>
    /// <li>
    /// <p><code>DELETED</code> - The attachment has been deleted</p></li>
    /// <li>
    /// <p><code>FAILED</code> - The attachment creation has failed and cannot be recovered</p></li>
    /// <li>
    /// <p><code>ERROR</code> - The attachment is in an error state that might be recoverable</p></li>
    /// <li>
    /// <p><code>READY</code> - The attachment is active and processing traffic</p></li>
    /// <li>
    /// <p><code>PENDING_ACCEPTANCE</code> - The attachment is waiting to be accepted</p></li>
    /// <li>
    /// <p><code>REJECTING</code> - The attachment is in the process of being rejected</p></li>
    /// <li>
    /// <p><code>REJECTED</code> - The attachment has been rejected</p></li>
    /// </ul>
    /// <p>For information about troubleshooting endpoint failures, see <a href="https://docs.aws.amazon.com/network-firewall/latest/developerguide/firewall-troubleshooting-endpoint-failures.html">Troubleshooting firewall endpoint failures</a> in the <i>Network Firewall Developer Guide</i>.</p>
    pub fn set_transit_gateway_attachment_status(mut self, input: ::std::option::Option<crate::types::TransitGatewayAttachmentStatus>) -> Self {
        self.transit_gateway_attachment_status = input;
        self
    }
    /// <p>The current status of the transit gateway attachment. Valid values are:</p>
    /// <ul>
    /// <li>
    /// <p><code>CREATING</code> - The attachment is being created</p></li>
    /// <li>
    /// <p><code>DELETING</code> - The attachment is being deleted</p></li>
    /// <li>
    /// <p><code>DELETED</code> - The attachment has been deleted</p></li>
    /// <li>
    /// <p><code>FAILED</code> - The attachment creation has failed and cannot be recovered</p></li>
    /// <li>
    /// <p><code>ERROR</code> - The attachment is in an error state that might be recoverable</p></li>
    /// <li>
    /// <p><code>READY</code> - The attachment is active and processing traffic</p></li>
    /// <li>
    /// <p><code>PENDING_ACCEPTANCE</code> - The attachment is waiting to be accepted</p></li>
    /// <li>
    /// <p><code>REJECTING</code> - The attachment is in the process of being rejected</p></li>
    /// <li>
    /// <p><code>REJECTED</code> - The attachment has been rejected</p></li>
    /// </ul>
    /// <p>For information about troubleshooting endpoint failures, see <a href="https://docs.aws.amazon.com/network-firewall/latest/developerguide/firewall-troubleshooting-endpoint-failures.html">Troubleshooting firewall endpoint failures</a> in the <i>Network Firewall Developer Guide</i>.</p>
    pub fn get_transit_gateway_attachment_status(&self) -> &::std::option::Option<crate::types::TransitGatewayAttachmentStatus> {
        &self.transit_gateway_attachment_status
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`RejectNetworkFirewallTransitGatewayAttachmentOutput`](crate::operation::reject_network_firewall_transit_gateway_attachment::RejectNetworkFirewallTransitGatewayAttachmentOutput).
    /// This method will fail if any of the following fields are not set:
    /// - [`transit_gateway_attachment_id`](crate::operation::reject_network_firewall_transit_gateway_attachment::builders::RejectNetworkFirewallTransitGatewayAttachmentOutputBuilder::transit_gateway_attachment_id)
    /// - [`transit_gateway_attachment_status`](crate::operation::reject_network_firewall_transit_gateway_attachment::builders::RejectNetworkFirewallTransitGatewayAttachmentOutputBuilder::transit_gateway_attachment_status)
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::reject_network_firewall_transit_gateway_attachment::RejectNetworkFirewallTransitGatewayAttachmentOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::reject_network_firewall_transit_gateway_attachment::RejectNetworkFirewallTransitGatewayAttachmentOutput {
                transit_gateway_attachment_id: self.transit_gateway_attachment_id
                    .ok_or_else(||
                        ::aws_smithy_types::error::operation::BuildError::missing_field("transit_gateway_attachment_id", "transit_gateway_attachment_id was not specified but it is required when building RejectNetworkFirewallTransitGatewayAttachmentOutput")
                    )?
                ,
                transit_gateway_attachment_status: self.transit_gateway_attachment_status
                    .ok_or_else(||
                        ::aws_smithy_types::error::operation::BuildError::missing_field("transit_gateway_attachment_status", "transit_gateway_attachment_status was not specified but it is required when building RejectNetworkFirewallTransitGatewayAttachmentOutput")
                    )?
                ,
                _request_id: self._request_id,
            }
        )
    }
}
