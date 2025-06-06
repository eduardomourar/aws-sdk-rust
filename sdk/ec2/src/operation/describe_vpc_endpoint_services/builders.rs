// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_vpc_endpoint_services::_describe_vpc_endpoint_services_output::DescribeVpcEndpointServicesOutputBuilder;

pub use crate::operation::describe_vpc_endpoint_services::_describe_vpc_endpoint_services_input::DescribeVpcEndpointServicesInputBuilder;

impl crate::operation::describe_vpc_endpoint_services::builders::DescribeVpcEndpointServicesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_vpc_endpoint_services::DescribeVpcEndpointServicesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_vpc_endpoint_services::DescribeVpcEndpointServicesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_vpc_endpoint_services();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeVpcEndpointServices`.
///
/// <p>Describes available services to which you can create a VPC endpoint.</p>
/// <p>When the service provider and the consumer have different accounts in multiple Availability Zones, and the consumer views the VPC endpoint service information, the response only includes the common Availability Zones. For example, when the service provider account uses <code>us-east-1a</code> and <code>us-east-1c</code> and the consumer uses <code>us-east-1a</code> and <code>us-east-1b</code>, the response includes the VPC endpoint services in the common Availability Zone, <code>us-east-1a</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeVpcEndpointServicesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_vpc_endpoint_services::builders::DescribeVpcEndpointServicesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_vpc_endpoint_services::DescribeVpcEndpointServicesOutput,
        crate::operation::describe_vpc_endpoint_services::DescribeVpcEndpointServicesError,
    > for DescribeVpcEndpointServicesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_vpc_endpoint_services::DescribeVpcEndpointServicesOutput,
            crate::operation::describe_vpc_endpoint_services::DescribeVpcEndpointServicesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeVpcEndpointServicesFluentBuilder {
    /// Creates a new `DescribeVpcEndpointServicesFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeVpcEndpointServices as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_vpc_endpoint_services::builders::DescribeVpcEndpointServicesInputBuilder {
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
        crate::operation::describe_vpc_endpoint_services::DescribeVpcEndpointServicesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_vpc_endpoint_services::DescribeVpcEndpointServicesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_vpc_endpoint_services::DescribeVpcEndpointServices::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_vpc_endpoint_services::DescribeVpcEndpointServices::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_vpc_endpoint_services::DescribeVpcEndpointServicesOutput,
        crate::operation::describe_vpc_endpoint_services::DescribeVpcEndpointServicesError,
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
    /// Appends an item to `ServiceNames`.
    ///
    /// To override the contents of this collection use [`set_service_names`](Self::set_service_names).
    ///
    /// <p>The service names.</p>
    pub fn service_names(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.service_names(input.into());
        self
    }
    /// <p>The service names.</p>
    pub fn set_service_names(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_service_names(input);
        self
    }
    /// <p>The service names.</p>
    pub fn get_service_names(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_service_names()
    }
    ///
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>The filters.</p>
    /// <ul>
    /// <li>
    /// <p><code>owner</code> - The ID or alias of the Amazon Web Services account that owns the service.</p></li>
    /// <li>
    /// <p><code>service-name</code> - The name of the service.</p></li>
    /// <li>
    /// <p><code>service-region</code> - The Region of the service.</p></li>
    /// <li>
    /// <p><code>service-type</code> - The type of service (<code>Interface</code> | <code>Gateway</code> | <code>GatewayLoadBalancer</code>).</p></li>
    /// <li>
    /// <p><code>supported-ip-address-types</code> - The IP address type (<code>ipv4</code> | <code>ipv6</code>).</p></li>
    /// <li>
    /// <p><code>tag</code>:<key>
    /// - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key
    /// <code>Owner</code> and the value
    /// <code>TeamA</code>, specify
    /// <code>tag:Owner</code> for the filter name and
    /// <code>TeamA</code> for the filter value.
    /// </key></p></li>
    /// <li>
    /// <p><code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p></li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>The filters.</p>
    /// <ul>
    /// <li>
    /// <p><code>owner</code> - The ID or alias of the Amazon Web Services account that owns the service.</p></li>
    /// <li>
    /// <p><code>service-name</code> - The name of the service.</p></li>
    /// <li>
    /// <p><code>service-region</code> - The Region of the service.</p></li>
    /// <li>
    /// <p><code>service-type</code> - The type of service (<code>Interface</code> | <code>Gateway</code> | <code>GatewayLoadBalancer</code>).</p></li>
    /// <li>
    /// <p><code>supported-ip-address-types</code> - The IP address type (<code>ipv4</code> | <code>ipv6</code>).</p></li>
    /// <li>
    /// <p><code>tag</code>:<key>
    /// - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key
    /// <code>Owner</code> and the value
    /// <code>TeamA</code>, specify
    /// <code>tag:Owner</code> for the filter name and
    /// <code>TeamA</code> for the filter value.
    /// </key></p></li>
    /// <li>
    /// <p><code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p></li>
    /// </ul>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>The filters.</p>
    /// <ul>
    /// <li>
    /// <p><code>owner</code> - The ID or alias of the Amazon Web Services account that owns the service.</p></li>
    /// <li>
    /// <p><code>service-name</code> - The name of the service.</p></li>
    /// <li>
    /// <p><code>service-region</code> - The Region of the service.</p></li>
    /// <li>
    /// <p><code>service-type</code> - The type of service (<code>Interface</code> | <code>Gateway</code> | <code>GatewayLoadBalancer</code>).</p></li>
    /// <li>
    /// <p><code>supported-ip-address-types</code> - The IP address type (<code>ipv4</code> | <code>ipv6</code>).</p></li>
    /// <li>
    /// <p><code>tag</code>:<key>
    /// - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key
    /// <code>Owner</code> and the value
    /// <code>TeamA</code>, specify
    /// <code>tag:Owner</code> for the filter name and
    /// <code>TeamA</code> for the filter value.
    /// </key></p></li>
    /// <li>
    /// <p><code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p></li>
    /// </ul>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Filter>> {
        self.inner.get_filters()
    }
    /// <p>The maximum number of items to return for this request. The request returns a token that you can specify in a subsequent call to get the next set of results.</p>
    /// <p>Constraint: If the value is greater than 1,000, we return only 1,000 items.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of items to return for this request. The request returns a token that you can specify in a subsequent call to get the next set of results.</p>
    /// <p>Constraint: If the value is greater than 1,000, we return only 1,000 items.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of items to return for this request. The request returns a token that you can specify in a subsequent call to get the next set of results.</p>
    /// <p>Constraint: If the value is greater than 1,000, we return only 1,000 items.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>The token for the next set of items to return. (You received this token from a prior call.)</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token for the next set of items to return. (You received this token from a prior call.)</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token for the next set of items to return. (You received this token from a prior call.)</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    ///
    /// Appends an item to `ServiceRegions`.
    ///
    /// To override the contents of this collection use [`set_service_regions`](Self::set_service_regions).
    ///
    /// <p>The service Regions.</p>
    pub fn service_regions(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.service_regions(input.into());
        self
    }
    /// <p>The service Regions.</p>
    pub fn set_service_regions(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_service_regions(input);
        self
    }
    /// <p>The service Regions.</p>
    pub fn get_service_regions(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_service_regions()
    }
}
