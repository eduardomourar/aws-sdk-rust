// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_ipam_resource_discovery::_modify_ipam_resource_discovery_output::ModifyIpamResourceDiscoveryOutputBuilder;

pub use crate::operation::modify_ipam_resource_discovery::_modify_ipam_resource_discovery_input::ModifyIpamResourceDiscoveryInputBuilder;

impl crate::operation::modify_ipam_resource_discovery::builders::ModifyIpamResourceDiscoveryInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::modify_ipam_resource_discovery::ModifyIpamResourceDiscoveryOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_ipam_resource_discovery::ModifyIpamResourceDiscoveryError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.modify_ipam_resource_discovery();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ModifyIpamResourceDiscovery`.
///
/// <p>Modifies a resource discovery. A resource discovery is an IPAM component that enables IPAM to manage and monitor resources that belong to the owning account.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifyIpamResourceDiscoveryFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::modify_ipam_resource_discovery::builders::ModifyIpamResourceDiscoveryInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::modify_ipam_resource_discovery::ModifyIpamResourceDiscoveryOutput,
        crate::operation::modify_ipam_resource_discovery::ModifyIpamResourceDiscoveryError,
    > for ModifyIpamResourceDiscoveryFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::modify_ipam_resource_discovery::ModifyIpamResourceDiscoveryOutput,
            crate::operation::modify_ipam_resource_discovery::ModifyIpamResourceDiscoveryError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ModifyIpamResourceDiscoveryFluentBuilder {
    /// Creates a new `ModifyIpamResourceDiscoveryFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ModifyIpamResourceDiscovery as a reference.
    pub fn as_input(&self) -> &crate::operation::modify_ipam_resource_discovery::builders::ModifyIpamResourceDiscoveryInputBuilder {
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
        crate::operation::modify_ipam_resource_discovery::ModifyIpamResourceDiscoveryOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_ipam_resource_discovery::ModifyIpamResourceDiscoveryError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::modify_ipam_resource_discovery::ModifyIpamResourceDiscovery::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::modify_ipam_resource_discovery::ModifyIpamResourceDiscovery::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::modify_ipam_resource_discovery::ModifyIpamResourceDiscoveryOutput,
        crate::operation::modify_ipam_resource_discovery::ModifyIpamResourceDiscoveryError,
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
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
    /// <p>A resource discovery ID.</p>
    pub fn ipam_resource_discovery_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.ipam_resource_discovery_id(input.into());
        self
    }
    /// <p>A resource discovery ID.</p>
    pub fn set_ipam_resource_discovery_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_ipam_resource_discovery_id(input);
        self
    }
    /// <p>A resource discovery ID.</p>
    pub fn get_ipam_resource_discovery_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_ipam_resource_discovery_id()
    }
    /// <p>A resource discovery description.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A resource discovery description.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>A resource discovery description.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    ///
    /// Appends an item to `AddOperatingRegions`.
    ///
    /// To override the contents of this collection use [`set_add_operating_regions`](Self::set_add_operating_regions).
    ///
    /// <p>Add operating Regions to the resource discovery. Operating Regions are Amazon Web Services Regions where the IPAM is allowed to manage IP address CIDRs. IPAM only discovers and monitors resources in the Amazon Web Services Regions you select as operating Regions.</p>
    pub fn add_operating_regions(mut self, input: crate::types::AddIpamOperatingRegion) -> Self {
        self.inner = self.inner.add_operating_regions(input);
        self
    }
    /// <p>Add operating Regions to the resource discovery. Operating Regions are Amazon Web Services Regions where the IPAM is allowed to manage IP address CIDRs. IPAM only discovers and monitors resources in the Amazon Web Services Regions you select as operating Regions.</p>
    pub fn set_add_operating_regions(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::AddIpamOperatingRegion>>) -> Self {
        self.inner = self.inner.set_add_operating_regions(input);
        self
    }
    /// <p>Add operating Regions to the resource discovery. Operating Regions are Amazon Web Services Regions where the IPAM is allowed to manage IP address CIDRs. IPAM only discovers and monitors resources in the Amazon Web Services Regions you select as operating Regions.</p>
    pub fn get_add_operating_regions(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::AddIpamOperatingRegion>> {
        self.inner.get_add_operating_regions()
    }
    ///
    /// Appends an item to `RemoveOperatingRegions`.
    ///
    /// To override the contents of this collection use [`set_remove_operating_regions`](Self::set_remove_operating_regions).
    ///
    /// <p>Remove operating Regions.</p>
    pub fn remove_operating_regions(mut self, input: crate::types::RemoveIpamOperatingRegion) -> Self {
        self.inner = self.inner.remove_operating_regions(input);
        self
    }
    /// <p>Remove operating Regions.</p>
    pub fn set_remove_operating_regions(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::RemoveIpamOperatingRegion>>) -> Self {
        self.inner = self.inner.set_remove_operating_regions(input);
        self
    }
    /// <p>Remove operating Regions.</p>
    pub fn get_remove_operating_regions(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::RemoveIpamOperatingRegion>> {
        self.inner.get_remove_operating_regions()
    }
    ///
    /// Appends an item to `AddOrganizationalUnitExclusions`.
    ///
    /// To override the contents of this collection use [`set_add_organizational_unit_exclusions`](Self::set_add_organizational_unit_exclusions).
    ///
    /// <p>Add an Organizational Unit (OU) exclusion to your IPAM. If your IPAM is integrated with Amazon Web Services Organizations and you add an organizational unit (OU) exclusion, IPAM will not manage the IP addresses in accounts in that OU exclusion. There is a limit on the number of exclusions you can create. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/quotas-ipam.html">Quotas for your IPAM</a> in the <i>Amazon VPC IPAM User Guide</i>.</p><note>
    /// <p>The resulting set of exclusions must not result in "overlap", meaning two or more OU exclusions must not exclude the same OU. For more information and examples, see the Amazon Web Services CLI request process in <a href="https://docs.aws.amazon.com/vpc/latest/ipam/exclude-ous.html#exclude-ous-create-delete">Add or remove OU exclusions </a> in the <i>Amazon VPC User Guide</i>.</p>
    /// </note>
    pub fn add_organizational_unit_exclusions(mut self, input: crate::types::AddIpamOrganizationalUnitExclusion) -> Self {
        self.inner = self.inner.add_organizational_unit_exclusions(input);
        self
    }
    /// <p>Add an Organizational Unit (OU) exclusion to your IPAM. If your IPAM is integrated with Amazon Web Services Organizations and you add an organizational unit (OU) exclusion, IPAM will not manage the IP addresses in accounts in that OU exclusion. There is a limit on the number of exclusions you can create. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/quotas-ipam.html">Quotas for your IPAM</a> in the <i>Amazon VPC IPAM User Guide</i>.</p><note>
    /// <p>The resulting set of exclusions must not result in "overlap", meaning two or more OU exclusions must not exclude the same OU. For more information and examples, see the Amazon Web Services CLI request process in <a href="https://docs.aws.amazon.com/vpc/latest/ipam/exclude-ous.html#exclude-ous-create-delete">Add or remove OU exclusions </a> in the <i>Amazon VPC User Guide</i>.</p>
    /// </note>
    pub fn set_add_organizational_unit_exclusions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::AddIpamOrganizationalUnitExclusion>>,
    ) -> Self {
        self.inner = self.inner.set_add_organizational_unit_exclusions(input);
        self
    }
    /// <p>Add an Organizational Unit (OU) exclusion to your IPAM. If your IPAM is integrated with Amazon Web Services Organizations and you add an organizational unit (OU) exclusion, IPAM will not manage the IP addresses in accounts in that OU exclusion. There is a limit on the number of exclusions you can create. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/quotas-ipam.html">Quotas for your IPAM</a> in the <i>Amazon VPC IPAM User Guide</i>.</p><note>
    /// <p>The resulting set of exclusions must not result in "overlap", meaning two or more OU exclusions must not exclude the same OU. For more information and examples, see the Amazon Web Services CLI request process in <a href="https://docs.aws.amazon.com/vpc/latest/ipam/exclude-ous.html#exclude-ous-create-delete">Add or remove OU exclusions </a> in the <i>Amazon VPC User Guide</i>.</p>
    /// </note>
    pub fn get_add_organizational_unit_exclusions(
        &self,
    ) -> &::std::option::Option<::std::vec::Vec<crate::types::AddIpamOrganizationalUnitExclusion>> {
        self.inner.get_add_organizational_unit_exclusions()
    }
    ///
    /// Appends an item to `RemoveOrganizationalUnitExclusions`.
    ///
    /// To override the contents of this collection use [`set_remove_organizational_unit_exclusions`](Self::set_remove_organizational_unit_exclusions).
    ///
    /// <p>Remove an Organizational Unit (OU) exclusion to your IPAM. If your IPAM is integrated with Amazon Web Services Organizations and you add an organizational unit (OU) exclusion, IPAM will not manage the IP addresses in accounts in that OU exclusion. There is a limit on the number of exclusions you can create. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/quotas-ipam.html">Quotas for your IPAM</a> in the <i>Amazon VPC IPAM User Guide</i>.</p><note>
    /// <p>The resulting set of exclusions must not result in "overlap", meaning two or more OU exclusions must not exclude the same OU. For more information and examples, see the Amazon Web Services CLI request process in <a href="https://docs.aws.amazon.com/vpc/latest/ipam/exclude-ous.html#exclude-ous-create-delete">Add or remove OU exclusions </a> in the <i>Amazon VPC User Guide</i>.</p>
    /// </note>
    pub fn remove_organizational_unit_exclusions(mut self, input: crate::types::RemoveIpamOrganizationalUnitExclusion) -> Self {
        self.inner = self.inner.remove_organizational_unit_exclusions(input);
        self
    }
    /// <p>Remove an Organizational Unit (OU) exclusion to your IPAM. If your IPAM is integrated with Amazon Web Services Organizations and you add an organizational unit (OU) exclusion, IPAM will not manage the IP addresses in accounts in that OU exclusion. There is a limit on the number of exclusions you can create. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/quotas-ipam.html">Quotas for your IPAM</a> in the <i>Amazon VPC IPAM User Guide</i>.</p><note>
    /// <p>The resulting set of exclusions must not result in "overlap", meaning two or more OU exclusions must not exclude the same OU. For more information and examples, see the Amazon Web Services CLI request process in <a href="https://docs.aws.amazon.com/vpc/latest/ipam/exclude-ous.html#exclude-ous-create-delete">Add or remove OU exclusions </a> in the <i>Amazon VPC User Guide</i>.</p>
    /// </note>
    pub fn set_remove_organizational_unit_exclusions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::RemoveIpamOrganizationalUnitExclusion>>,
    ) -> Self {
        self.inner = self.inner.set_remove_organizational_unit_exclusions(input);
        self
    }
    /// <p>Remove an Organizational Unit (OU) exclusion to your IPAM. If your IPAM is integrated with Amazon Web Services Organizations and you add an organizational unit (OU) exclusion, IPAM will not manage the IP addresses in accounts in that OU exclusion. There is a limit on the number of exclusions you can create. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/quotas-ipam.html">Quotas for your IPAM</a> in the <i>Amazon VPC IPAM User Guide</i>.</p><note>
    /// <p>The resulting set of exclusions must not result in "overlap", meaning two or more OU exclusions must not exclude the same OU. For more information and examples, see the Amazon Web Services CLI request process in <a href="https://docs.aws.amazon.com/vpc/latest/ipam/exclude-ous.html#exclude-ous-create-delete">Add or remove OU exclusions </a> in the <i>Amazon VPC User Guide</i>.</p>
    /// </note>
    pub fn get_remove_organizational_unit_exclusions(
        &self,
    ) -> &::std::option::Option<::std::vec::Vec<crate::types::RemoveIpamOrganizationalUnitExclusion>> {
        self.inner.get_remove_organizational_unit_exclusions()
    }
}
