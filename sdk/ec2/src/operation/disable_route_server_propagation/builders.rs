// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disable_route_server_propagation::_disable_route_server_propagation_output::DisableRouteServerPropagationOutputBuilder;

pub use crate::operation::disable_route_server_propagation::_disable_route_server_propagation_input::DisableRouteServerPropagationInputBuilder;

impl crate::operation::disable_route_server_propagation::builders::DisableRouteServerPropagationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::disable_route_server_propagation::DisableRouteServerPropagationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::disable_route_server_propagation::DisableRouteServerPropagationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.disable_route_server_propagation();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DisableRouteServerPropagation`.
///
/// <p>Disables route propagation from a route server to a specified route table.</p>
/// <p>When enabled, route server propagation installs the routes in the FIB on the route table you've specified. Route server supports IPv4 and IPv6 route propagation.</p>
/// <p>Amazon VPC Route Server simplifies routing for traffic between workloads that are deployed within a VPC and its internet gateways. With this feature, VPC Route Server dynamically updates VPC and internet gateway route tables with your preferred IPv4 or IPv6 routes to achieve routing fault tolerance for those workloads. This enables you to automatically reroute traffic within a VPC, which increases the manageability of VPC routing and interoperability with third-party workloads.</p>
/// <p>Route server supports the follow route table types:</p>
/// <ul>
/// <li>
/// <p>VPC route tables not associated with subnets</p></li>
/// <li>
/// <p>Subnet route tables</p></li>
/// <li>
/// <p>Internet gateway route tables</p></li>
/// </ul>
/// <p>Route server does not support route tables associated with virtual private gateways. To propagate routes into a transit gateway route table, use <a href="https://docs.aws.amazon.com/vpc/latest/tgw/tgw-connect.html">Transit Gateway Connect</a>.</p>
/// <p>For more information see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/dynamic-routing-route-server.html">Dynamic routing in your VPC with VPC Route Server</a> in the <i>Amazon VPC User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DisableRouteServerPropagationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::disable_route_server_propagation::builders::DisableRouteServerPropagationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::disable_route_server_propagation::DisableRouteServerPropagationOutput,
        crate::operation::disable_route_server_propagation::DisableRouteServerPropagationError,
    > for DisableRouteServerPropagationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::disable_route_server_propagation::DisableRouteServerPropagationOutput,
            crate::operation::disable_route_server_propagation::DisableRouteServerPropagationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DisableRouteServerPropagationFluentBuilder {
    /// Creates a new `DisableRouteServerPropagationFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DisableRouteServerPropagation as a reference.
    pub fn as_input(&self) -> &crate::operation::disable_route_server_propagation::builders::DisableRouteServerPropagationInputBuilder {
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
        crate::operation::disable_route_server_propagation::DisableRouteServerPropagationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::disable_route_server_propagation::DisableRouteServerPropagationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::disable_route_server_propagation::DisableRouteServerPropagation::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::disable_route_server_propagation::DisableRouteServerPropagation::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::disable_route_server_propagation::DisableRouteServerPropagationOutput,
        crate::operation::disable_route_server_propagation::DisableRouteServerPropagationError,
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
    /// <p>The ID of the route server for which to disable propagation.</p>
    pub fn route_server_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.route_server_id(input.into());
        self
    }
    /// <p>The ID of the route server for which to disable propagation.</p>
    pub fn set_route_server_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_route_server_id(input);
        self
    }
    /// <p>The ID of the route server for which to disable propagation.</p>
    pub fn get_route_server_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_route_server_id()
    }
    /// <p>The ID of the route table for which to disable route server propagation.</p>
    pub fn route_table_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.route_table_id(input.into());
        self
    }
    /// <p>The ID of the route table for which to disable route server propagation.</p>
    pub fn set_route_table_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_route_table_id(input);
        self
    }
    /// <p>The ID of the route table for which to disable route server propagation.</p>
    pub fn get_route_table_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_route_table_id()
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
}
