// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::verify_device_position::_verify_device_position_output::VerifyDevicePositionOutputBuilder;

pub use crate::operation::verify_device_position::_verify_device_position_input::VerifyDevicePositionInputBuilder;

impl crate::operation::verify_device_position::builders::VerifyDevicePositionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::verify_device_position::VerifyDevicePositionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::verify_device_position::VerifyDevicePositionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.verify_device_position();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `VerifyDevicePosition`.
///
/// <p>Verifies the integrity of the device's position by determining if it was reported behind a proxy, and by comparing it to an inferred position estimated based on the device's state.</p><note>
/// <p>The Location Integrity SDK provides enhanced features related to device verification, and it is available for use by request. To get access to the SDK, contact <a href="https://aws.amazon.com/contact-us/sales-support/?pg=locationprice&amp;cta=herobtn">Sales Support</a>.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct VerifyDevicePositionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::verify_device_position::builders::VerifyDevicePositionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::verify_device_position::VerifyDevicePositionOutput,
        crate::operation::verify_device_position::VerifyDevicePositionError,
    > for VerifyDevicePositionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::verify_device_position::VerifyDevicePositionOutput,
            crate::operation::verify_device_position::VerifyDevicePositionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl VerifyDevicePositionFluentBuilder {
    /// Creates a new `VerifyDevicePositionFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the VerifyDevicePosition as a reference.
    pub fn as_input(&self) -> &crate::operation::verify_device_position::builders::VerifyDevicePositionInputBuilder {
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
        crate::operation::verify_device_position::VerifyDevicePositionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::verify_device_position::VerifyDevicePositionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::verify_device_position::VerifyDevicePosition::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::verify_device_position::VerifyDevicePosition::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::verify_device_position::VerifyDevicePositionOutput,
        crate::operation::verify_device_position::VerifyDevicePositionError,
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
    /// <p>The name of the tracker resource to be associated with verification request.</p>
    pub fn tracker_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tracker_name(input.into());
        self
    }
    /// <p>The name of the tracker resource to be associated with verification request.</p>
    pub fn set_tracker_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_tracker_name(input);
        self
    }
    /// <p>The name of the tracker resource to be associated with verification request.</p>
    pub fn get_tracker_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_tracker_name()
    }
    /// <p>The device's state, including position, IP address, cell signals and Wi-Fi access points.</p>
    pub fn device_state(mut self, input: crate::types::DeviceState) -> Self {
        self.inner = self.inner.device_state(input);
        self
    }
    /// <p>The device's state, including position, IP address, cell signals and Wi-Fi access points.</p>
    pub fn set_device_state(mut self, input: ::std::option::Option<crate::types::DeviceState>) -> Self {
        self.inner = self.inner.set_device_state(input);
        self
    }
    /// <p>The device's state, including position, IP address, cell signals and Wi-Fi access points.</p>
    pub fn get_device_state(&self) -> &::std::option::Option<crate::types::DeviceState> {
        self.inner.get_device_state()
    }
    /// <p>The distance unit for the verification request.</p>
    /// <p>Default Value: <code>Kilometers</code></p>
    pub fn distance_unit(mut self, input: crate::types::DistanceUnit) -> Self {
        self.inner = self.inner.distance_unit(input);
        self
    }
    /// <p>The distance unit for the verification request.</p>
    /// <p>Default Value: <code>Kilometers</code></p>
    pub fn set_distance_unit(mut self, input: ::std::option::Option<crate::types::DistanceUnit>) -> Self {
        self.inner = self.inner.set_distance_unit(input);
        self
    }
    /// <p>The distance unit for the verification request.</p>
    /// <p>Default Value: <code>Kilometers</code></p>
    pub fn get_distance_unit(&self) -> &::std::option::Option<crate::types::DistanceUnit> {
        self.inner.get_distance_unit()
    }
}
