// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the position update details for a device.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct DevicePositionUpdate {
    /// <p>The device associated to the position update.</p>
    pub device_id: ::std::string::String,
    /// <p>The timestamp at which the device's position was determined. Uses <a href="https://www.iso.org/iso-8601-date-and-time-format.html">ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code></p>
    pub sample_time: ::aws_smithy_types::DateTime,
    /// <p>The latest device position defined in <a href="https://earth-info.nga.mil/index.php?dir=wgs84&amp;action=wgs84">WGS 84</a> format: <code>\[X or longitude, Y or latitude\]</code>.</p>
    pub position: ::std::vec::Vec<f64>,
    /// <p>The accuracy of the device position.</p>
    pub accuracy: ::std::option::Option<crate::types::PositionalAccuracy>,
    /// <p>Associates one of more properties with the position update. A property is a key-value pair stored with the position update and added to any geofence event the update may trigger.</p>
    /// <p>Format: <code>"key" : "value"</code></p>
    pub position_properties: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
}
impl DevicePositionUpdate {
    /// <p>The device associated to the position update.</p>
    pub fn device_id(&self) -> &str {
        use std::ops::Deref;
        self.device_id.deref()
    }
    /// <p>The timestamp at which the device's position was determined. Uses <a href="https://www.iso.org/iso-8601-date-and-time-format.html">ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code></p>
    pub fn sample_time(&self) -> &::aws_smithy_types::DateTime {
        &self.sample_time
    }
    /// <p>The latest device position defined in <a href="https://earth-info.nga.mil/index.php?dir=wgs84&amp;action=wgs84">WGS 84</a> format: <code>\[X or longitude, Y or latitude\]</code>.</p>
    pub fn position(&self) -> &[f64] {
        use std::ops::Deref;
        self.position.deref()
    }
    /// <p>The accuracy of the device position.</p>
    pub fn accuracy(&self) -> ::std::option::Option<&crate::types::PositionalAccuracy> {
        self.accuracy.as_ref()
    }
    /// <p>Associates one of more properties with the position update. A property is a key-value pair stored with the position update and added to any geofence event the update may trigger.</p>
    /// <p>Format: <code>"key" : "value"</code></p>
    pub fn position_properties(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.position_properties.as_ref()
    }
}
impl ::std::fmt::Debug for DevicePositionUpdate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("DevicePositionUpdate");
        formatter.field("device_id", &self.device_id);
        formatter.field("sample_time", &"*** Sensitive Data Redacted ***");
        formatter.field("position", &"*** Sensitive Data Redacted ***");
        formatter.field("accuracy", &self.accuracy);
        formatter.field("position_properties", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl DevicePositionUpdate {
    /// Creates a new builder-style object to manufacture [`DevicePositionUpdate`](crate::types::DevicePositionUpdate).
    pub fn builder() -> crate::types::builders::DevicePositionUpdateBuilder {
        crate::types::builders::DevicePositionUpdateBuilder::default()
    }
}

/// A builder for [`DevicePositionUpdate`](crate::types::DevicePositionUpdate).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct DevicePositionUpdateBuilder {
    pub(crate) device_id: ::std::option::Option<::std::string::String>,
    pub(crate) sample_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) position: ::std::option::Option<::std::vec::Vec<f64>>,
    pub(crate) accuracy: ::std::option::Option<crate::types::PositionalAccuracy>,
    pub(crate) position_properties: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
}
impl DevicePositionUpdateBuilder {
    /// <p>The device associated to the position update.</p>
    /// This field is required.
    pub fn device_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.device_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The device associated to the position update.</p>
    pub fn set_device_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.device_id = input;
        self
    }
    /// <p>The device associated to the position update.</p>
    pub fn get_device_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.device_id
    }
    /// <p>The timestamp at which the device's position was determined. Uses <a href="https://www.iso.org/iso-8601-date-and-time-format.html">ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code></p>
    /// This field is required.
    pub fn sample_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.sample_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The timestamp at which the device's position was determined. Uses <a href="https://www.iso.org/iso-8601-date-and-time-format.html">ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code></p>
    pub fn set_sample_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.sample_time = input;
        self
    }
    /// <p>The timestamp at which the device's position was determined. Uses <a href="https://www.iso.org/iso-8601-date-and-time-format.html">ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code></p>
    pub fn get_sample_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.sample_time
    }
    /// Appends an item to `position`.
    ///
    /// To override the contents of this collection use [`set_position`](Self::set_position).
    ///
    /// <p>The latest device position defined in <a href="https://earth-info.nga.mil/index.php?dir=wgs84&amp;action=wgs84">WGS 84</a> format: <code>\[X or longitude, Y or latitude\]</code>.</p>
    pub fn position(mut self, input: f64) -> Self {
        let mut v = self.position.unwrap_or_default();
        v.push(input);
        self.position = ::std::option::Option::Some(v);
        self
    }
    /// <p>The latest device position defined in <a href="https://earth-info.nga.mil/index.php?dir=wgs84&amp;action=wgs84">WGS 84</a> format: <code>\[X or longitude, Y or latitude\]</code>.</p>
    pub fn set_position(mut self, input: ::std::option::Option<::std::vec::Vec<f64>>) -> Self {
        self.position = input;
        self
    }
    /// <p>The latest device position defined in <a href="https://earth-info.nga.mil/index.php?dir=wgs84&amp;action=wgs84">WGS 84</a> format: <code>\[X or longitude, Y or latitude\]</code>.</p>
    pub fn get_position(&self) -> &::std::option::Option<::std::vec::Vec<f64>> {
        &self.position
    }
    /// <p>The accuracy of the device position.</p>
    pub fn accuracy(mut self, input: crate::types::PositionalAccuracy) -> Self {
        self.accuracy = ::std::option::Option::Some(input);
        self
    }
    /// <p>The accuracy of the device position.</p>
    pub fn set_accuracy(mut self, input: ::std::option::Option<crate::types::PositionalAccuracy>) -> Self {
        self.accuracy = input;
        self
    }
    /// <p>The accuracy of the device position.</p>
    pub fn get_accuracy(&self) -> &::std::option::Option<crate::types::PositionalAccuracy> {
        &self.accuracy
    }
    /// Adds a key-value pair to `position_properties`.
    ///
    /// To override the contents of this collection use [`set_position_properties`](Self::set_position_properties).
    ///
    /// <p>Associates one of more properties with the position update. A property is a key-value pair stored with the position update and added to any geofence event the update may trigger.</p>
    /// <p>Format: <code>"key" : "value"</code></p>
    pub fn position_properties(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.position_properties.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.position_properties = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>Associates one of more properties with the position update. A property is a key-value pair stored with the position update and added to any geofence event the update may trigger.</p>
    /// <p>Format: <code>"key" : "value"</code></p>
    pub fn set_position_properties(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    ) -> Self {
        self.position_properties = input;
        self
    }
    /// <p>Associates one of more properties with the position update. A property is a key-value pair stored with the position update and added to any geofence event the update may trigger.</p>
    /// <p>Format: <code>"key" : "value"</code></p>
    pub fn get_position_properties(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        &self.position_properties
    }
    /// Consumes the builder and constructs a [`DevicePositionUpdate`](crate::types::DevicePositionUpdate).
    /// This method will fail if any of the following fields are not set:
    /// - [`device_id`](crate::types::builders::DevicePositionUpdateBuilder::device_id)
    /// - [`sample_time`](crate::types::builders::DevicePositionUpdateBuilder::sample_time)
    /// - [`position`](crate::types::builders::DevicePositionUpdateBuilder::position)
    pub fn build(self) -> ::std::result::Result<crate::types::DevicePositionUpdate, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::DevicePositionUpdate {
            device_id: self.device_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "device_id",
                    "device_id was not specified but it is required when building DevicePositionUpdate",
                )
            })?,
            sample_time: self.sample_time.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "sample_time",
                    "sample_time was not specified but it is required when building DevicePositionUpdate",
                )
            })?,
            position: self.position.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "position",
                    "position was not specified but it is required when building DevicePositionUpdate",
                )
            })?,
            accuracy: self.accuracy,
            position_properties: self.position_properties,
        })
    }
}
impl ::std::fmt::Debug for DevicePositionUpdateBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("DevicePositionUpdateBuilder");
        formatter.field("device_id", &self.device_id);
        formatter.field("sample_time", &"*** Sensitive Data Redacted ***");
        formatter.field("position", &"*** Sensitive Data Redacted ***");
        formatter.field("accuracy", &self.accuracy);
        formatter.field("position_properties", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
