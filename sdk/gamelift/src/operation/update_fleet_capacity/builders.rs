// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_fleet_capacity::_update_fleet_capacity_output::UpdateFleetCapacityOutputBuilder;

pub use crate::operation::update_fleet_capacity::_update_fleet_capacity_input::UpdateFleetCapacityInputBuilder;

impl crate::operation::update_fleet_capacity::builders::UpdateFleetCapacityInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_fleet_capacity::UpdateFleetCapacityOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_fleet_capacity::UpdateFleetCapacityError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_fleet_capacity();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateFleetCapacity`.
///
/// <p>Updates capacity settings for a managed EC2 fleet or managed container fleet. For these fleets, you adjust capacity by changing the number of instances in the fleet. Fleet capacity determines the number of game sessions and players that the fleet can host based on its configuration. For fleets with multiple locations, use this operation to manage capacity settings in each location individually.</p>
/// <p>Use this operation to set these fleet capacity properties:</p>
/// <ul>
/// <li>
/// <p>Minimum/maximum size: Set hard limits on the number of Amazon EC2 instances allowed. If Amazon GameLift Servers receives a request--either through manual update or automatic scaling--it won't change the capacity to a value outside of this range.</p></li>
/// <li>
/// <p>Desired capacity: As an alternative to automatic scaling, manually set the number of Amazon EC2 instances to be maintained. Before changing a fleet's desired capacity, check the maximum capacity of the fleet's Amazon EC2 instance type by calling <a href="https://docs.aws.amazon.com/gamelift/latest/apireference/API_DescribeEC2InstanceLimits.html">DescribeEC2InstanceLimits</a>.</p></li>
/// </ul>
/// <p>To update capacity for a fleet's home Region, or if the fleet has no remote locations, omit the <code>Location</code> parameter. The fleet must be in <code>ACTIVE</code> status.</p>
/// <p>To update capacity for a fleet's remote location, set the <code>Location</code> parameter to the location to update. The location must be in <code>ACTIVE</code> status.</p>
/// <p>If successful, Amazon GameLift Servers updates the capacity settings and returns the identifiers for the updated fleet and/or location. If a requested change to desired capacity exceeds the instance type's limit, the <code>LimitExceeded</code> exception occurs.</p>
/// <p>Updates often prompt an immediate change in fleet capacity, such as when current capacity is different than the new desired capacity or outside the new limits. In this scenario, Amazon GameLift Servers automatically initiates steps to add or remove instances in the fleet location. You can track a fleet's current capacity by calling <a href="https://docs.aws.amazon.com/gamelift/latest/apireference/API_DescribeFleetCapacity.html">DescribeFleetCapacity</a> or <a href="https://docs.aws.amazon.com/gamelift/latest/apireference/API_DescribeFleetLocationCapacity.html">DescribeFleetLocationCapacity</a>.</p>
/// <p><b>Learn more</b></p>
/// <p><a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-manage-capacity.html">Scaling fleet capacity</a></p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateFleetCapacityFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_fleet_capacity::builders::UpdateFleetCapacityInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_fleet_capacity::UpdateFleetCapacityOutput,
        crate::operation::update_fleet_capacity::UpdateFleetCapacityError,
    > for UpdateFleetCapacityFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_fleet_capacity::UpdateFleetCapacityOutput,
            crate::operation::update_fleet_capacity::UpdateFleetCapacityError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateFleetCapacityFluentBuilder {
    /// Creates a new `UpdateFleetCapacityFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateFleetCapacity as a reference.
    pub fn as_input(&self) -> &crate::operation::update_fleet_capacity::builders::UpdateFleetCapacityInputBuilder {
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
        crate::operation::update_fleet_capacity::UpdateFleetCapacityOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_fleet_capacity::UpdateFleetCapacityError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_fleet_capacity::UpdateFleetCapacity::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_fleet_capacity::UpdateFleetCapacity::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_fleet_capacity::UpdateFleetCapacityOutput,
        crate::operation::update_fleet_capacity::UpdateFleetCapacityError,
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
    /// <p>A unique identifier for the fleet to update capacity settings for. You can use either the fleet ID or ARN value.</p>
    pub fn fleet_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.fleet_id(input.into());
        self
    }
    /// <p>A unique identifier for the fleet to update capacity settings for. You can use either the fleet ID or ARN value.</p>
    pub fn set_fleet_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_fleet_id(input);
        self
    }
    /// <p>A unique identifier for the fleet to update capacity settings for. You can use either the fleet ID or ARN value.</p>
    pub fn get_fleet_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_fleet_id()
    }
    /// <p>The number of Amazon EC2 instances you want to maintain in the specified fleet location. This value must fall between the minimum and maximum size limits. Changes in desired instance value can take up to 1 minute to be reflected when viewing the fleet's capacity settings.</p>
    pub fn desired_instances(mut self, input: i32) -> Self {
        self.inner = self.inner.desired_instances(input);
        self
    }
    /// <p>The number of Amazon EC2 instances you want to maintain in the specified fleet location. This value must fall between the minimum and maximum size limits. Changes in desired instance value can take up to 1 minute to be reflected when viewing the fleet's capacity settings.</p>
    pub fn set_desired_instances(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_desired_instances(input);
        self
    }
    /// <p>The number of Amazon EC2 instances you want to maintain in the specified fleet location. This value must fall between the minimum and maximum size limits. Changes in desired instance value can take up to 1 minute to be reflected when viewing the fleet's capacity settings.</p>
    pub fn get_desired_instances(&self) -> &::std::option::Option<i32> {
        self.inner.get_desired_instances()
    }
    /// <p>The minimum number of instances that are allowed in the specified fleet location. If this parameter is not set, the default is 0.</p>
    pub fn min_size(mut self, input: i32) -> Self {
        self.inner = self.inner.min_size(input);
        self
    }
    /// <p>The minimum number of instances that are allowed in the specified fleet location. If this parameter is not set, the default is 0.</p>
    pub fn set_min_size(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_min_size(input);
        self
    }
    /// <p>The minimum number of instances that are allowed in the specified fleet location. If this parameter is not set, the default is 0.</p>
    pub fn get_min_size(&self) -> &::std::option::Option<i32> {
        self.inner.get_min_size()
    }
    /// <p>The maximum number of instances that are allowed in the specified fleet location. If this parameter is not set, the default is 1.</p>
    pub fn max_size(mut self, input: i32) -> Self {
        self.inner = self.inner.max_size(input);
        self
    }
    /// <p>The maximum number of instances that are allowed in the specified fleet location. If this parameter is not set, the default is 1.</p>
    pub fn set_max_size(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_size(input);
        self
    }
    /// <p>The maximum number of instances that are allowed in the specified fleet location. If this parameter is not set, the default is 1.</p>
    pub fn get_max_size(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_size()
    }
    /// <p>The name of a remote location to update fleet capacity settings for, in the form of an Amazon Web Services Region code such as <code>us-west-2</code>.</p>
    pub fn location(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.location(input.into());
        self
    }
    /// <p>The name of a remote location to update fleet capacity settings for, in the form of an Amazon Web Services Region code such as <code>us-west-2</code>.</p>
    pub fn set_location(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_location(input);
        self
    }
    /// <p>The name of a remote location to update fleet capacity settings for, in the form of an Amazon Web Services Region code such as <code>us-west-2</code>.</p>
    pub fn get_location(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_location()
    }
}
