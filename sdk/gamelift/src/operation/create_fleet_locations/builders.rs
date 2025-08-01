// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_fleet_locations::_create_fleet_locations_output::CreateFleetLocationsOutputBuilder;

pub use crate::operation::create_fleet_locations::_create_fleet_locations_input::CreateFleetLocationsInputBuilder;

impl crate::operation::create_fleet_locations::builders::CreateFleetLocationsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_fleet_locations::CreateFleetLocationsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_fleet_locations::CreateFleetLocationsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_fleet_locations();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateFleetLocations`.
///
/// <p>Adds remote locations to an EC2 and begins populating the new locations with instances. The new instances conform to the fleet's instance type, auto-scaling, and other configuration settings.</p><note>
/// <p>You can't add remote locations to a fleet that resides in an Amazon Web Services Region that doesn't support multiple locations. Fleets created prior to March 2021 can't support multiple locations.</p>
/// </note>
/// <p>To add fleet locations, specify the fleet to be updated and provide a list of one or more locations.</p>
/// <p>If successful, this operation returns the list of added locations with their status set to <code>NEW</code>. Amazon GameLift Servers initiates the process of starting an instance in each added location. You can track the status of each new location by monitoring location creation events using <a href="https://docs.aws.amazon.com/gamelift/latest/apireference/API_DescribeFleetEvents.html">DescribeFleetEvents</a>.</p>
/// <p><b>Learn more</b></p>
/// <p><a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-intro.html">Setting up fleets</a></p>
/// <p><a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-editing.html#fleets-update-locations">Update fleet locations</a></p>
/// <p><a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/gamelift-regions.html"> Amazon GameLift Servers service locations</a> for managed hosting.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateFleetLocationsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_fleet_locations::builders::CreateFleetLocationsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_fleet_locations::CreateFleetLocationsOutput,
        crate::operation::create_fleet_locations::CreateFleetLocationsError,
    > for CreateFleetLocationsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_fleet_locations::CreateFleetLocationsOutput,
            crate::operation::create_fleet_locations::CreateFleetLocationsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateFleetLocationsFluentBuilder {
    /// Creates a new `CreateFleetLocationsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateFleetLocations as a reference.
    pub fn as_input(&self) -> &crate::operation::create_fleet_locations::builders::CreateFleetLocationsInputBuilder {
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
        crate::operation::create_fleet_locations::CreateFleetLocationsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_fleet_locations::CreateFleetLocationsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_fleet_locations::CreateFleetLocations::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_fleet_locations::CreateFleetLocations::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_fleet_locations::CreateFleetLocationsOutput,
        crate::operation::create_fleet_locations::CreateFleetLocationsError,
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
    /// <p>A unique identifier for the fleet to add locations to. You can use either the fleet ID or ARN value.</p>
    pub fn fleet_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.fleet_id(input.into());
        self
    }
    /// <p>A unique identifier for the fleet to add locations to. You can use either the fleet ID or ARN value.</p>
    pub fn set_fleet_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_fleet_id(input);
        self
    }
    /// <p>A unique identifier for the fleet to add locations to. You can use either the fleet ID or ARN value.</p>
    pub fn get_fleet_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_fleet_id()
    }
    ///
    /// Appends an item to `Locations`.
    ///
    /// To override the contents of this collection use [`set_locations`](Self::set_locations).
    ///
    /// <p>A list of locations to deploy additional instances to and manage as part of the fleet. You can add any Amazon GameLift Servers-supported Amazon Web Services Region as a remote location, in the form of an Amazon Web Services Region code such as <code>us-west-2</code>.</p>
    pub fn locations(mut self, input: crate::types::LocationConfiguration) -> Self {
        self.inner = self.inner.locations(input);
        self
    }
    /// <p>A list of locations to deploy additional instances to and manage as part of the fleet. You can add any Amazon GameLift Servers-supported Amazon Web Services Region as a remote location, in the form of an Amazon Web Services Region code such as <code>us-west-2</code>.</p>
    pub fn set_locations(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::LocationConfiguration>>) -> Self {
        self.inner = self.inner.set_locations(input);
        self
    }
    /// <p>A list of locations to deploy additional instances to and manage as part of the fleet. You can add any Amazon GameLift Servers-supported Amazon Web Services Region as a remote location, in the form of an Amazon Web Services Region code such as <code>us-west-2</code>.</p>
    pub fn get_locations(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::LocationConfiguration>> {
        self.inner.get_locations()
    }
}
