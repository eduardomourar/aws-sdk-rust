// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::enable_route_server_propagation::_enable_route_server_propagation_output::EnableRouteServerPropagationOutputBuilder;

pub use crate::operation::enable_route_server_propagation::_enable_route_server_propagation_input::EnableRouteServerPropagationInputBuilder;

impl crate::operation::enable_route_server_propagation::builders::EnableRouteServerPropagationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::enable_route_server_propagation::EnableRouteServerPropagationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::enable_route_server_propagation::EnableRouteServerPropagationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.enable_route_server_propagation();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `EnableRouteServerPropagation`.
///
/// <p>Defines which route tables the route server can update with routes.</p>
/// <p>When enabled, route server propagation installs the routes in the FIB on the route table you've specified. Route server supports IPv4 and IPv6 route propagation.</p>
/// <p>For more information see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/dynamic-routing-route-server.html">Dynamic routing in your VPC with VPC Route Server</a> in the <i>Amazon VPC User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct EnableRouteServerPropagationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::enable_route_server_propagation::builders::EnableRouteServerPropagationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::enable_route_server_propagation::EnableRouteServerPropagationOutput,
        crate::operation::enable_route_server_propagation::EnableRouteServerPropagationError,
    > for EnableRouteServerPropagationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::enable_route_server_propagation::EnableRouteServerPropagationOutput,
            crate::operation::enable_route_server_propagation::EnableRouteServerPropagationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl EnableRouteServerPropagationFluentBuilder {
    /// Creates a new `EnableRouteServerPropagationFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the EnableRouteServerPropagation as a reference.
    pub fn as_input(&self) -> &crate::operation::enable_route_server_propagation::builders::EnableRouteServerPropagationInputBuilder {
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
        crate::operation::enable_route_server_propagation::EnableRouteServerPropagationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::enable_route_server_propagation::EnableRouteServerPropagationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::enable_route_server_propagation::EnableRouteServerPropagation::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::enable_route_server_propagation::EnableRouteServerPropagation::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::enable_route_server_propagation::EnableRouteServerPropagationOutput,
        crate::operation::enable_route_server_propagation::EnableRouteServerPropagationError,
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
    /// <p>The ID of the route server for which to enable propagation.</p>
    pub fn route_server_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.route_server_id(input.into());
        self
    }
    /// <p>The ID of the route server for which to enable propagation.</p>
    pub fn set_route_server_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_route_server_id(input);
        self
    }
    /// <p>The ID of the route server for which to enable propagation.</p>
    pub fn get_route_server_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_route_server_id()
    }
    /// <p>The ID of the route table to which route server will propagate routes.</p>
    pub fn route_table_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.route_table_id(input.into());
        self
    }
    /// <p>The ID of the route table to which route server will propagate routes.</p>
    pub fn set_route_table_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_route_table_id(input);
        self
    }
    /// <p>The ID of the route table to which route server will propagate routes.</p>
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
