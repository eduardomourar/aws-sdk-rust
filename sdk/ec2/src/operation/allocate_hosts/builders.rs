// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::allocate_hosts::_allocate_hosts_output::AllocateHostsOutputBuilder;

pub use crate::operation::allocate_hosts::_allocate_hosts_input::AllocateHostsInputBuilder;

impl crate::operation::allocate_hosts::builders::AllocateHostsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::allocate_hosts::AllocateHostsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::allocate_hosts::AllocateHostsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.allocate_hosts();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AllocateHosts`.
///
/// <p>Allocates a Dedicated Host to your account. At a minimum, specify the supported instance type or instance family, the Availability Zone in which to allocate the host, and the number of hosts to allocate.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AllocateHostsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::allocate_hosts::builders::AllocateHostsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::allocate_hosts::AllocateHostsOutput,
        crate::operation::allocate_hosts::AllocateHostsError,
    > for AllocateHostsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::allocate_hosts::AllocateHostsOutput,
            crate::operation::allocate_hosts::AllocateHostsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AllocateHostsFluentBuilder {
    /// Creates a new `AllocateHostsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AllocateHosts as a reference.
    pub fn as_input(&self) -> &crate::operation::allocate_hosts::builders::AllocateHostsInputBuilder {
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
        crate::operation::allocate_hosts::AllocateHostsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::allocate_hosts::AllocateHostsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::allocate_hosts::AllocateHosts::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::allocate_hosts::AllocateHosts::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::allocate_hosts::AllocateHostsOutput,
        crate::operation::allocate_hosts::AllocateHostsError,
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
    /// <p>Specifies the instance family to be supported by the Dedicated Hosts. If you specify an instance family, the Dedicated Hosts support multiple instance types within that instance family.</p>
    /// <p>If you want the Dedicated Hosts to support a specific instance type only, omit this parameter and specify <b>InstanceType</b> instead. You cannot specify <b>InstanceFamily</b> and <b>InstanceType</b> in the same request.</p>
    pub fn instance_family(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_family(input.into());
        self
    }
    /// <p>Specifies the instance family to be supported by the Dedicated Hosts. If you specify an instance family, the Dedicated Hosts support multiple instance types within that instance family.</p>
    /// <p>If you want the Dedicated Hosts to support a specific instance type only, omit this parameter and specify <b>InstanceType</b> instead. You cannot specify <b>InstanceFamily</b> and <b>InstanceType</b> in the same request.</p>
    pub fn set_instance_family(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_family(input);
        self
    }
    /// <p>Specifies the instance family to be supported by the Dedicated Hosts. If you specify an instance family, the Dedicated Hosts support multiple instance types within that instance family.</p>
    /// <p>If you want the Dedicated Hosts to support a specific instance type only, omit this parameter and specify <b>InstanceType</b> instead. You cannot specify <b>InstanceFamily</b> and <b>InstanceType</b> in the same request.</p>
    pub fn get_instance_family(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_instance_family()
    }
    ///
    /// Appends an item to `TagSpecifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to apply to the Dedicated Host during creation.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        self.inner = self.inner.tag_specifications(input);
        self
    }
    /// <p>The tags to apply to the Dedicated Host during creation.</p>
    pub fn set_tag_specifications(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>) -> Self {
        self.inner = self.inner.set_tag_specifications(input);
        self
    }
    /// <p>The tags to apply to the Dedicated Host during creation.</p>
    pub fn get_tag_specifications(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>> {
        self.inner.get_tag_specifications()
    }
    /// <p>Indicates whether to enable or disable host recovery for the Dedicated Host. Host recovery is disabled by default. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/dedicated-hosts-recovery.html"> Host recovery</a> in the <i>Amazon EC2 User Guide</i>.</p>
    /// <p>Default: <code>off</code></p>
    pub fn host_recovery(mut self, input: crate::types::HostRecovery) -> Self {
        self.inner = self.inner.host_recovery(input);
        self
    }
    /// <p>Indicates whether to enable or disable host recovery for the Dedicated Host. Host recovery is disabled by default. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/dedicated-hosts-recovery.html"> Host recovery</a> in the <i>Amazon EC2 User Guide</i>.</p>
    /// <p>Default: <code>off</code></p>
    pub fn set_host_recovery(mut self, input: ::std::option::Option<crate::types::HostRecovery>) -> Self {
        self.inner = self.inner.set_host_recovery(input);
        self
    }
    /// <p>Indicates whether to enable or disable host recovery for the Dedicated Host. Host recovery is disabled by default. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/dedicated-hosts-recovery.html"> Host recovery</a> in the <i>Amazon EC2 User Guide</i>.</p>
    /// <p>Default: <code>off</code></p>
    pub fn get_host_recovery(&self) -> &::std::option::Option<crate::types::HostRecovery> {
        self.inner.get_host_recovery()
    }
    /// <p>The Amazon Resource Name (ARN) of the Amazon Web Services Outpost on which to allocate the Dedicated Host. If you specify <b>OutpostArn</b>, you can optionally specify <b>AssetIds</b>.</p>
    /// <p>If you are allocating the Dedicated Host in a Region, omit this parameter.</p>
    pub fn outpost_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.outpost_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Amazon Web Services Outpost on which to allocate the Dedicated Host. If you specify <b>OutpostArn</b>, you can optionally specify <b>AssetIds</b>.</p>
    /// <p>If you are allocating the Dedicated Host in a Region, omit this parameter.</p>
    pub fn set_outpost_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_outpost_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Amazon Web Services Outpost on which to allocate the Dedicated Host. If you specify <b>OutpostArn</b>, you can optionally specify <b>AssetIds</b>.</p>
    /// <p>If you are allocating the Dedicated Host in a Region, omit this parameter.</p>
    pub fn get_outpost_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_outpost_arn()
    }
    /// <p>Indicates whether to enable or disable host maintenance for the Dedicated Host. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/dedicated-hosts-maintenance.html">Host maintenance</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn host_maintenance(mut self, input: crate::types::HostMaintenance) -> Self {
        self.inner = self.inner.host_maintenance(input);
        self
    }
    /// <p>Indicates whether to enable or disable host maintenance for the Dedicated Host. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/dedicated-hosts-maintenance.html">Host maintenance</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn set_host_maintenance(mut self, input: ::std::option::Option<crate::types::HostMaintenance>) -> Self {
        self.inner = self.inner.set_host_maintenance(input);
        self
    }
    /// <p>Indicates whether to enable or disable host maintenance for the Dedicated Host. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/dedicated-hosts-maintenance.html">Host maintenance</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn get_host_maintenance(&self) -> &::std::option::Option<crate::types::HostMaintenance> {
        self.inner.get_host_maintenance()
    }
    ///
    /// Appends an item to `AssetIds`.
    ///
    /// To override the contents of this collection use [`set_asset_ids`](Self::set_asset_ids).
    ///
    /// <p>The IDs of the Outpost hardware assets on which to allocate the Dedicated Hosts. Targeting specific hardware assets on an Outpost can help to minimize latency between your workloads. This parameter is supported only if you specify <b>OutpostArn</b>. If you are allocating the Dedicated Hosts in a Region, omit this parameter.</p>
    /// <ul>
    /// <li>
    /// <p>If you specify this parameter, you can omit <b>Quantity</b>. In this case, Amazon EC2 allocates a Dedicated Host on each specified hardware asset.</p></li>
    /// <li>
    /// <p>If you specify both <b>AssetIds</b> and <b>Quantity</b>, then the value for <b>Quantity</b> must be equal to the number of asset IDs specified.</p></li>
    /// </ul>
    pub fn asset_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.asset_ids(input.into());
        self
    }
    /// <p>The IDs of the Outpost hardware assets on which to allocate the Dedicated Hosts. Targeting specific hardware assets on an Outpost can help to minimize latency between your workloads. This parameter is supported only if you specify <b>OutpostArn</b>. If you are allocating the Dedicated Hosts in a Region, omit this parameter.</p>
    /// <ul>
    /// <li>
    /// <p>If you specify this parameter, you can omit <b>Quantity</b>. In this case, Amazon EC2 allocates a Dedicated Host on each specified hardware asset.</p></li>
    /// <li>
    /// <p>If you specify both <b>AssetIds</b> and <b>Quantity</b>, then the value for <b>Quantity</b> must be equal to the number of asset IDs specified.</p></li>
    /// </ul>
    pub fn set_asset_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_asset_ids(input);
        self
    }
    /// <p>The IDs of the Outpost hardware assets on which to allocate the Dedicated Hosts. Targeting specific hardware assets on an Outpost can help to minimize latency between your workloads. This parameter is supported only if you specify <b>OutpostArn</b>. If you are allocating the Dedicated Hosts in a Region, omit this parameter.</p>
    /// <ul>
    /// <li>
    /// <p>If you specify this parameter, you can omit <b>Quantity</b>. In this case, Amazon EC2 allocates a Dedicated Host on each specified hardware asset.</p></li>
    /// <li>
    /// <p>If you specify both <b>AssetIds</b> and <b>Quantity</b>, then the value for <b>Quantity</b> must be equal to the number of asset IDs specified.</p></li>
    /// </ul>
    pub fn get_asset_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_asset_ids()
    }
    /// <p>The ID of the Availability Zone.</p>
    pub fn availability_zone_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.availability_zone_id(input.into());
        self
    }
    /// <p>The ID of the Availability Zone.</p>
    pub fn set_availability_zone_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_availability_zone_id(input);
        self
    }
    /// <p>The ID of the Availability Zone.</p>
    pub fn get_availability_zone_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_availability_zone_id()
    }
    /// <p>Indicates whether the host accepts any untargeted instance launches that match its instance type configuration, or if it only accepts Host tenancy instance launches that specify its unique host ID. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/how-dedicated-hosts-work.html#dedicated-hosts-understanding"> Understanding auto-placement and affinity</a> in the <i>Amazon EC2 User Guide</i>.</p>
    /// <p>Default: <code>off</code></p>
    pub fn auto_placement(mut self, input: crate::types::AutoPlacement) -> Self {
        self.inner = self.inner.auto_placement(input);
        self
    }
    /// <p>Indicates whether the host accepts any untargeted instance launches that match its instance type configuration, or if it only accepts Host tenancy instance launches that specify its unique host ID. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/how-dedicated-hosts-work.html#dedicated-hosts-understanding"> Understanding auto-placement and affinity</a> in the <i>Amazon EC2 User Guide</i>.</p>
    /// <p>Default: <code>off</code></p>
    pub fn set_auto_placement(mut self, input: ::std::option::Option<crate::types::AutoPlacement>) -> Self {
        self.inner = self.inner.set_auto_placement(input);
        self
    }
    /// <p>Indicates whether the host accepts any untargeted instance launches that match its instance type configuration, or if it only accepts Host tenancy instance launches that specify its unique host ID. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/how-dedicated-hosts-work.html#dedicated-hosts-understanding"> Understanding auto-placement and affinity</a> in the <i>Amazon EC2 User Guide</i>.</p>
    /// <p>Default: <code>off</code></p>
    pub fn get_auto_placement(&self) -> &::std::option::Option<crate::types::AutoPlacement> {
        self.inner.get_auto_placement()
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// <p>Specifies the instance type to be supported by the Dedicated Hosts. If you specify an instance type, the Dedicated Hosts support instances of the specified instance type only.</p>
    /// <p>If you want the Dedicated Hosts to support multiple instance types in a specific instance family, omit this parameter and specify <b>InstanceFamily</b> instead. You cannot specify <b>InstanceType</b> and <b>InstanceFamily</b> in the same request.</p>
    pub fn instance_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_type(input.into());
        self
    }
    /// <p>Specifies the instance type to be supported by the Dedicated Hosts. If you specify an instance type, the Dedicated Hosts support instances of the specified instance type only.</p>
    /// <p>If you want the Dedicated Hosts to support multiple instance types in a specific instance family, omit this parameter and specify <b>InstanceFamily</b> instead. You cannot specify <b>InstanceType</b> and <b>InstanceFamily</b> in the same request.</p>
    pub fn set_instance_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_type(input);
        self
    }
    /// <p>Specifies the instance type to be supported by the Dedicated Hosts. If you specify an instance type, the Dedicated Hosts support instances of the specified instance type only.</p>
    /// <p>If you want the Dedicated Hosts to support multiple instance types in a specific instance family, omit this parameter and specify <b>InstanceFamily</b> instead. You cannot specify <b>InstanceType</b> and <b>InstanceFamily</b> in the same request.</p>
    pub fn get_instance_type(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_instance_type()
    }
    /// <p>The number of Dedicated Hosts to allocate to your account with these parameters. If you are allocating the Dedicated Hosts on an Outpost, and you specify <b>AssetIds</b>, you can omit this parameter. In this case, Amazon EC2 allocates a Dedicated Host on each specified hardware asset. If you specify both <b>AssetIds</b> and <b>Quantity</b>, then the value that you specify for <b>Quantity</b> must be equal to the number of asset IDs specified.</p>
    pub fn quantity(mut self, input: i32) -> Self {
        self.inner = self.inner.quantity(input);
        self
    }
    /// <p>The number of Dedicated Hosts to allocate to your account with these parameters. If you are allocating the Dedicated Hosts on an Outpost, and you specify <b>AssetIds</b>, you can omit this parameter. In this case, Amazon EC2 allocates a Dedicated Host on each specified hardware asset. If you specify both <b>AssetIds</b> and <b>Quantity</b>, then the value that you specify for <b>Quantity</b> must be equal to the number of asset IDs specified.</p>
    pub fn set_quantity(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_quantity(input);
        self
    }
    /// <p>The number of Dedicated Hosts to allocate to your account with these parameters. If you are allocating the Dedicated Hosts on an Outpost, and you specify <b>AssetIds</b>, you can omit this parameter. In this case, Amazon EC2 allocates a Dedicated Host on each specified hardware asset. If you specify both <b>AssetIds</b> and <b>Quantity</b>, then the value that you specify for <b>Quantity</b> must be equal to the number of asset IDs specified.</p>
    pub fn get_quantity(&self) -> &::std::option::Option<i32> {
        self.inner.get_quantity()
    }
    /// <p>The Availability Zone in which to allocate the Dedicated Host.</p>
    pub fn availability_zone(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.availability_zone(input.into());
        self
    }
    /// <p>The Availability Zone in which to allocate the Dedicated Host.</p>
    pub fn set_availability_zone(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_availability_zone(input);
        self
    }
    /// <p>The Availability Zone in which to allocate the Dedicated Host.</p>
    pub fn get_availability_zone(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_availability_zone()
    }
}
