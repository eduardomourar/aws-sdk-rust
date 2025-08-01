// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains geofence geometry details.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct BatchPutGeofenceRequestEntry {
    /// <p>The identifier for the geofence to be stored in a given geofence collection.</p>
    pub geofence_id: ::std::string::String,
    /// <p>Contains the details to specify the position of the geofence. Can be a circle, a polygon, or a multipolygon. <code>Polygon</code> and <code>MultiPolygon</code> geometries can be defined using their respective parameters, or encoded in Geobuf format using the <code>Geobuf</code> parameter. Including multiple geometry types in the same request will return a validation error.</p><note>
    /// <p>The geofence <code>Polygon</code> and <code>MultiPolygon</code> formats support a maximum of 1,000 total vertices. The <code>Geobuf</code> format supports a maximum of 100,000 vertices.</p>
    /// </note>
    pub geometry: ::std::option::Option<crate::types::GeofenceGeometry>,
    /// <p>Associates one of more properties with the geofence. A property is a key-value pair stored with the geofence and added to any geofence event triggered with that geofence.</p>
    /// <p>Format: <code>"key" : "value"</code></p>
    pub geofence_properties: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
}
impl BatchPutGeofenceRequestEntry {
    /// <p>The identifier for the geofence to be stored in a given geofence collection.</p>
    pub fn geofence_id(&self) -> &str {
        use std::ops::Deref;
        self.geofence_id.deref()
    }
    /// <p>Contains the details to specify the position of the geofence. Can be a circle, a polygon, or a multipolygon. <code>Polygon</code> and <code>MultiPolygon</code> geometries can be defined using their respective parameters, or encoded in Geobuf format using the <code>Geobuf</code> parameter. Including multiple geometry types in the same request will return a validation error.</p><note>
    /// <p>The geofence <code>Polygon</code> and <code>MultiPolygon</code> formats support a maximum of 1,000 total vertices. The <code>Geobuf</code> format supports a maximum of 100,000 vertices.</p>
    /// </note>
    pub fn geometry(&self) -> ::std::option::Option<&crate::types::GeofenceGeometry> {
        self.geometry.as_ref()
    }
    /// <p>Associates one of more properties with the geofence. A property is a key-value pair stored with the geofence and added to any geofence event triggered with that geofence.</p>
    /// <p>Format: <code>"key" : "value"</code></p>
    pub fn geofence_properties(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.geofence_properties.as_ref()
    }
}
impl ::std::fmt::Debug for BatchPutGeofenceRequestEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("BatchPutGeofenceRequestEntry");
        formatter.field("geofence_id", &self.geofence_id);
        formatter.field("geometry", &self.geometry);
        formatter.field("geofence_properties", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl BatchPutGeofenceRequestEntry {
    /// Creates a new builder-style object to manufacture [`BatchPutGeofenceRequestEntry`](crate::types::BatchPutGeofenceRequestEntry).
    pub fn builder() -> crate::types::builders::BatchPutGeofenceRequestEntryBuilder {
        crate::types::builders::BatchPutGeofenceRequestEntryBuilder::default()
    }
}

/// A builder for [`BatchPutGeofenceRequestEntry`](crate::types::BatchPutGeofenceRequestEntry).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct BatchPutGeofenceRequestEntryBuilder {
    pub(crate) geofence_id: ::std::option::Option<::std::string::String>,
    pub(crate) geometry: ::std::option::Option<crate::types::GeofenceGeometry>,
    pub(crate) geofence_properties: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
}
impl BatchPutGeofenceRequestEntryBuilder {
    /// <p>The identifier for the geofence to be stored in a given geofence collection.</p>
    /// This field is required.
    pub fn geofence_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.geofence_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier for the geofence to be stored in a given geofence collection.</p>
    pub fn set_geofence_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.geofence_id = input;
        self
    }
    /// <p>The identifier for the geofence to be stored in a given geofence collection.</p>
    pub fn get_geofence_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.geofence_id
    }
    /// <p>Contains the details to specify the position of the geofence. Can be a circle, a polygon, or a multipolygon. <code>Polygon</code> and <code>MultiPolygon</code> geometries can be defined using their respective parameters, or encoded in Geobuf format using the <code>Geobuf</code> parameter. Including multiple geometry types in the same request will return a validation error.</p><note>
    /// <p>The geofence <code>Polygon</code> and <code>MultiPolygon</code> formats support a maximum of 1,000 total vertices. The <code>Geobuf</code> format supports a maximum of 100,000 vertices.</p>
    /// </note>
    /// This field is required.
    pub fn geometry(mut self, input: crate::types::GeofenceGeometry) -> Self {
        self.geometry = ::std::option::Option::Some(input);
        self
    }
    /// <p>Contains the details to specify the position of the geofence. Can be a circle, a polygon, or a multipolygon. <code>Polygon</code> and <code>MultiPolygon</code> geometries can be defined using their respective parameters, or encoded in Geobuf format using the <code>Geobuf</code> parameter. Including multiple geometry types in the same request will return a validation error.</p><note>
    /// <p>The geofence <code>Polygon</code> and <code>MultiPolygon</code> formats support a maximum of 1,000 total vertices. The <code>Geobuf</code> format supports a maximum of 100,000 vertices.</p>
    /// </note>
    pub fn set_geometry(mut self, input: ::std::option::Option<crate::types::GeofenceGeometry>) -> Self {
        self.geometry = input;
        self
    }
    /// <p>Contains the details to specify the position of the geofence. Can be a circle, a polygon, or a multipolygon. <code>Polygon</code> and <code>MultiPolygon</code> geometries can be defined using their respective parameters, or encoded in Geobuf format using the <code>Geobuf</code> parameter. Including multiple geometry types in the same request will return a validation error.</p><note>
    /// <p>The geofence <code>Polygon</code> and <code>MultiPolygon</code> formats support a maximum of 1,000 total vertices. The <code>Geobuf</code> format supports a maximum of 100,000 vertices.</p>
    /// </note>
    pub fn get_geometry(&self) -> &::std::option::Option<crate::types::GeofenceGeometry> {
        &self.geometry
    }
    /// Adds a key-value pair to `geofence_properties`.
    ///
    /// To override the contents of this collection use [`set_geofence_properties`](Self::set_geofence_properties).
    ///
    /// <p>Associates one of more properties with the geofence. A property is a key-value pair stored with the geofence and added to any geofence event triggered with that geofence.</p>
    /// <p>Format: <code>"key" : "value"</code></p>
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
    /// <p>Associates one of more properties with the geofence. A property is a key-value pair stored with the geofence and added to any geofence event triggered with that geofence.</p>
    /// <p>Format: <code>"key" : "value"</code></p>
    pub fn set_geofence_properties(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    ) -> Self {
        self.geofence_properties = input;
        self
    }
    /// <p>Associates one of more properties with the geofence. A property is a key-value pair stored with the geofence and added to any geofence event triggered with that geofence.</p>
    /// <p>Format: <code>"key" : "value"</code></p>
    pub fn get_geofence_properties(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        &self.geofence_properties
    }
    /// Consumes the builder and constructs a [`BatchPutGeofenceRequestEntry`](crate::types::BatchPutGeofenceRequestEntry).
    /// This method will fail if any of the following fields are not set:
    /// - [`geofence_id`](crate::types::builders::BatchPutGeofenceRequestEntryBuilder::geofence_id)
    pub fn build(self) -> ::std::result::Result<crate::types::BatchPutGeofenceRequestEntry, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::BatchPutGeofenceRequestEntry {
            geofence_id: self.geofence_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "geofence_id",
                    "geofence_id was not specified but it is required when building BatchPutGeofenceRequestEntry",
                )
            })?,
            geometry: self.geometry,
            geofence_properties: self.geofence_properties,
        })
    }
}
impl ::std::fmt::Debug for BatchPutGeofenceRequestEntryBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("BatchPutGeofenceRequestEntryBuilder");
        formatter.field("geofence_id", &self.geofence_id);
        formatter.field("geometry", &self.geometry);
        formatter.field("geofence_properties", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
