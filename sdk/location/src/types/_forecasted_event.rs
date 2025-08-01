// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A forecasted event represents a geofence event in relation to the requested device state, that may occur given the provided device state and time horizon.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct ForecastedEvent {
    /// <p>The forecasted event identifier.</p>
    pub event_id: ::std::string::String,
    /// <p>The geofence identifier pertaining to the forecasted event.</p>
    pub geofence_id: ::std::string::String,
    /// <p>Indicates if the device is located within the geofence.</p>
    pub is_device_in_geofence: bool,
    /// <p>The closest distance from the device's position to the geofence.</p>
    pub nearest_distance: f64,
    /// <p>The event type, forecasting three states for which a device can be in relative to a geofence:</p>
    /// <p><code>ENTER</code>: If a device is outside of a geofence, but would breach the fence if the device is moving at its current speed within time horizon window.</p>
    /// <p><code>EXIT</code>: If a device is inside of a geofence, but would breach the fence if the device is moving at its current speed within time horizon window.</p>
    /// <p><code>IDLE</code>: If a device is inside of a geofence, and the device is not moving.</p>
    pub event_type: crate::types::ForecastedGeofenceEventType,
    /// <p>The forecasted time the device will breach the geofence in <a href="https://www.iso.org/iso-8601-date-and-time-format.html">ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code></p>
    pub forecasted_breach_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The geofence properties.</p>
    pub geofence_properties: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
}
impl ForecastedEvent {
    /// <p>The forecasted event identifier.</p>
    pub fn event_id(&self) -> &str {
        use std::ops::Deref;
        self.event_id.deref()
    }
    /// <p>The geofence identifier pertaining to the forecasted event.</p>
    pub fn geofence_id(&self) -> &str {
        use std::ops::Deref;
        self.geofence_id.deref()
    }
    /// <p>Indicates if the device is located within the geofence.</p>
    pub fn is_device_in_geofence(&self) -> bool {
        self.is_device_in_geofence
    }
    /// <p>The closest distance from the device's position to the geofence.</p>
    pub fn nearest_distance(&self) -> f64 {
        self.nearest_distance
    }
    /// <p>The event type, forecasting three states for which a device can be in relative to a geofence:</p>
    /// <p><code>ENTER</code>: If a device is outside of a geofence, but would breach the fence if the device is moving at its current speed within time horizon window.</p>
    /// <p><code>EXIT</code>: If a device is inside of a geofence, but would breach the fence if the device is moving at its current speed within time horizon window.</p>
    /// <p><code>IDLE</code>: If a device is inside of a geofence, and the device is not moving.</p>
    pub fn event_type(&self) -> &crate::types::ForecastedGeofenceEventType {
        &self.event_type
    }
    /// <p>The forecasted time the device will breach the geofence in <a href="https://www.iso.org/iso-8601-date-and-time-format.html">ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code></p>
    pub fn forecasted_breach_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.forecasted_breach_time.as_ref()
    }
    /// <p>The geofence properties.</p>
    pub fn geofence_properties(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.geofence_properties.as_ref()
    }
}
impl ::std::fmt::Debug for ForecastedEvent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("ForecastedEvent");
        formatter.field("event_id", &self.event_id);
        formatter.field("geofence_id", &self.geofence_id);
        formatter.field("is_device_in_geofence", &self.is_device_in_geofence);
        formatter.field("nearest_distance", &self.nearest_distance);
        formatter.field("event_type", &self.event_type);
        formatter.field("forecasted_breach_time", &"*** Sensitive Data Redacted ***");
        formatter.field("geofence_properties", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl ForecastedEvent {
    /// Creates a new builder-style object to manufacture [`ForecastedEvent`](crate::types::ForecastedEvent).
    pub fn builder() -> crate::types::builders::ForecastedEventBuilder {
        crate::types::builders::ForecastedEventBuilder::default()
    }
}

/// A builder for [`ForecastedEvent`](crate::types::ForecastedEvent).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct ForecastedEventBuilder {
    pub(crate) event_id: ::std::option::Option<::std::string::String>,
    pub(crate) geofence_id: ::std::option::Option<::std::string::String>,
    pub(crate) is_device_in_geofence: ::std::option::Option<bool>,
    pub(crate) nearest_distance: ::std::option::Option<f64>,
    pub(crate) event_type: ::std::option::Option<crate::types::ForecastedGeofenceEventType>,
    pub(crate) forecasted_breach_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) geofence_properties: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
}
impl ForecastedEventBuilder {
    /// <p>The forecasted event identifier.</p>
    /// This field is required.
    pub fn event_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.event_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The forecasted event identifier.</p>
    pub fn set_event_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.event_id = input;
        self
    }
    /// <p>The forecasted event identifier.</p>
    pub fn get_event_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.event_id
    }
    /// <p>The geofence identifier pertaining to the forecasted event.</p>
    /// This field is required.
    pub fn geofence_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.geofence_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The geofence identifier pertaining to the forecasted event.</p>
    pub fn set_geofence_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.geofence_id = input;
        self
    }
    /// <p>The geofence identifier pertaining to the forecasted event.</p>
    pub fn get_geofence_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.geofence_id
    }
    /// <p>Indicates if the device is located within the geofence.</p>
    /// This field is required.
    pub fn is_device_in_geofence(mut self, input: bool) -> Self {
        self.is_device_in_geofence = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates if the device is located within the geofence.</p>
    pub fn set_is_device_in_geofence(mut self, input: ::std::option::Option<bool>) -> Self {
        self.is_device_in_geofence = input;
        self
    }
    /// <p>Indicates if the device is located within the geofence.</p>
    pub fn get_is_device_in_geofence(&self) -> &::std::option::Option<bool> {
        &self.is_device_in_geofence
    }
    /// <p>The closest distance from the device's position to the geofence.</p>
    /// This field is required.
    pub fn nearest_distance(mut self, input: f64) -> Self {
        self.nearest_distance = ::std::option::Option::Some(input);
        self
    }
    /// <p>The closest distance from the device's position to the geofence.</p>
    pub fn set_nearest_distance(mut self, input: ::std::option::Option<f64>) -> Self {
        self.nearest_distance = input;
        self
    }
    /// <p>The closest distance from the device's position to the geofence.</p>
    pub fn get_nearest_distance(&self) -> &::std::option::Option<f64> {
        &self.nearest_distance
    }
    /// <p>The event type, forecasting three states for which a device can be in relative to a geofence:</p>
    /// <p><code>ENTER</code>: If a device is outside of a geofence, but would breach the fence if the device is moving at its current speed within time horizon window.</p>
    /// <p><code>EXIT</code>: If a device is inside of a geofence, but would breach the fence if the device is moving at its current speed within time horizon window.</p>
    /// <p><code>IDLE</code>: If a device is inside of a geofence, and the device is not moving.</p>
    /// This field is required.
    pub fn event_type(mut self, input: crate::types::ForecastedGeofenceEventType) -> Self {
        self.event_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The event type, forecasting three states for which a device can be in relative to a geofence:</p>
    /// <p><code>ENTER</code>: If a device is outside of a geofence, but would breach the fence if the device is moving at its current speed within time horizon window.</p>
    /// <p><code>EXIT</code>: If a device is inside of a geofence, but would breach the fence if the device is moving at its current speed within time horizon window.</p>
    /// <p><code>IDLE</code>: If a device is inside of a geofence, and the device is not moving.</p>
    pub fn set_event_type(mut self, input: ::std::option::Option<crate::types::ForecastedGeofenceEventType>) -> Self {
        self.event_type = input;
        self
    }
    /// <p>The event type, forecasting three states for which a device can be in relative to a geofence:</p>
    /// <p><code>ENTER</code>: If a device is outside of a geofence, but would breach the fence if the device is moving at its current speed within time horizon window.</p>
    /// <p><code>EXIT</code>: If a device is inside of a geofence, but would breach the fence if the device is moving at its current speed within time horizon window.</p>
    /// <p><code>IDLE</code>: If a device is inside of a geofence, and the device is not moving.</p>
    pub fn get_event_type(&self) -> &::std::option::Option<crate::types::ForecastedGeofenceEventType> {
        &self.event_type
    }
    /// <p>The forecasted time the device will breach the geofence in <a href="https://www.iso.org/iso-8601-date-and-time-format.html">ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code></p>
    pub fn forecasted_breach_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.forecasted_breach_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The forecasted time the device will breach the geofence in <a href="https://www.iso.org/iso-8601-date-and-time-format.html">ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code></p>
    pub fn set_forecasted_breach_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.forecasted_breach_time = input;
        self
    }
    /// <p>The forecasted time the device will breach the geofence in <a href="https://www.iso.org/iso-8601-date-and-time-format.html">ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code></p>
    pub fn get_forecasted_breach_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.forecasted_breach_time
    }
    /// Adds a key-value pair to `geofence_properties`.
    ///
    /// To override the contents of this collection use [`set_geofence_properties`](Self::set_geofence_properties).
    ///
    /// <p>The geofence properties.</p>
    pub fn geofence_properties(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.geofence_properties.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.geofence_properties = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>The geofence properties.</p>
    pub fn set_geofence_properties(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    ) -> Self {
        self.geofence_properties = input;
        self
    }
    /// <p>The geofence properties.</p>
    pub fn get_geofence_properties(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        &self.geofence_properties
    }
    /// Consumes the builder and constructs a [`ForecastedEvent`](crate::types::ForecastedEvent).
    /// This method will fail if any of the following fields are not set:
    /// - [`event_id`](crate::types::builders::ForecastedEventBuilder::event_id)
    /// - [`geofence_id`](crate::types::builders::ForecastedEventBuilder::geofence_id)
    /// - [`is_device_in_geofence`](crate::types::builders::ForecastedEventBuilder::is_device_in_geofence)
    /// - [`event_type`](crate::types::builders::ForecastedEventBuilder::event_type)
    pub fn build(self) -> ::std::result::Result<crate::types::ForecastedEvent, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::ForecastedEvent {
            event_id: self.event_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "event_id",
                    "event_id was not specified but it is required when building ForecastedEvent",
                )
            })?,
            geofence_id: self.geofence_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "geofence_id",
                    "geofence_id was not specified but it is required when building ForecastedEvent",
                )
            })?,
            is_device_in_geofence: self.is_device_in_geofence.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "is_device_in_geofence",
                    "is_device_in_geofence was not specified but it is required when building ForecastedEvent",
                )
            })?,
            nearest_distance: self.nearest_distance.unwrap_or_default(),
            event_type: self.event_type.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "event_type",
                    "event_type was not specified but it is required when building ForecastedEvent",
                )
            })?,
            forecasted_breach_time: self.forecasted_breach_time,
            geofence_properties: self.geofence_properties,
        })
    }
}
impl ::std::fmt::Debug for ForecastedEventBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("ForecastedEventBuilder");
        formatter.field("event_id", &self.event_id);
        formatter.field("geofence_id", &self.geofence_id);
        formatter.field("is_device_in_geofence", &self.is_device_in_geofence);
        formatter.field("nearest_distance", &self.nearest_distance);
        formatter.field("event_type", &self.event_type);
        formatter.field("forecasted_breach_time", &"*** Sensitive Data Redacted ***");
        formatter.field("geofence_properties", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
