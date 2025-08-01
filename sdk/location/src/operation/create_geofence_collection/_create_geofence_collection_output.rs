// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct CreateGeofenceCollectionOutput {
    /// <p>The name for the geofence collection.</p>
    pub collection_name: ::std::string::String,
    /// <p>The Amazon Resource Name (ARN) for the geofence collection resource. Used when you need to specify a resource across all Amazon Web Services.</p>
    /// <ul>
    /// <li>
    /// <p>Format example: <code>arn:aws:geo:region:account-id:geofence-collection/ExampleGeofenceCollection</code></p></li>
    /// </ul>
    pub collection_arn: ::std::string::String,
    /// <p>The timestamp for when the geofence collection was created in <a href="https://www.iso.org/iso-8601-date-and-time-format.html">ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code></p>
    pub create_time: ::aws_smithy_types::DateTime,
    _request_id: Option<String>,
}
impl CreateGeofenceCollectionOutput {
    /// <p>The name for the geofence collection.</p>
    pub fn collection_name(&self) -> &str {
        use std::ops::Deref;
        self.collection_name.deref()
    }
    /// <p>The Amazon Resource Name (ARN) for the geofence collection resource. Used when you need to specify a resource across all Amazon Web Services.</p>
    /// <ul>
    /// <li>
    /// <p>Format example: <code>arn:aws:geo:region:account-id:geofence-collection/ExampleGeofenceCollection</code></p></li>
    /// </ul>
    pub fn collection_arn(&self) -> &str {
        use std::ops::Deref;
        self.collection_arn.deref()
    }
    /// <p>The timestamp for when the geofence collection was created in <a href="https://www.iso.org/iso-8601-date-and-time-format.html">ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code></p>
    pub fn create_time(&self) -> &::aws_smithy_types::DateTime {
        &self.create_time
    }
}
impl ::std::fmt::Debug for CreateGeofenceCollectionOutput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CreateGeofenceCollectionOutput");
        formatter.field("collection_name", &self.collection_name);
        formatter.field("collection_arn", &self.collection_arn);
        formatter.field("create_time", &"*** Sensitive Data Redacted ***");
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
impl ::aws_types::request_id::RequestId for CreateGeofenceCollectionOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateGeofenceCollectionOutput {
    /// Creates a new builder-style object to manufacture [`CreateGeofenceCollectionOutput`](crate::operation::create_geofence_collection::CreateGeofenceCollectionOutput).
    pub fn builder() -> crate::operation::create_geofence_collection::builders::CreateGeofenceCollectionOutputBuilder {
        crate::operation::create_geofence_collection::builders::CreateGeofenceCollectionOutputBuilder::default()
    }
}

/// A builder for [`CreateGeofenceCollectionOutput`](crate::operation::create_geofence_collection::CreateGeofenceCollectionOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct CreateGeofenceCollectionOutputBuilder {
    pub(crate) collection_name: ::std::option::Option<::std::string::String>,
    pub(crate) collection_arn: ::std::option::Option<::std::string::String>,
    pub(crate) create_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    _request_id: Option<String>,
}
impl CreateGeofenceCollectionOutputBuilder {
    /// <p>The name for the geofence collection.</p>
    /// This field is required.
    pub fn collection_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.collection_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name for the geofence collection.</p>
    pub fn set_collection_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.collection_name = input;
        self
    }
    /// <p>The name for the geofence collection.</p>
    pub fn get_collection_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.collection_name
    }
    /// <p>The Amazon Resource Name (ARN) for the geofence collection resource. Used when you need to specify a resource across all Amazon Web Services.</p>
    /// <ul>
    /// <li>
    /// <p>Format example: <code>arn:aws:geo:region:account-id:geofence-collection/ExampleGeofenceCollection</code></p></li>
    /// </ul>
    /// This field is required.
    pub fn collection_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.collection_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) for the geofence collection resource. Used when you need to specify a resource across all Amazon Web Services.</p>
    /// <ul>
    /// <li>
    /// <p>Format example: <code>arn:aws:geo:region:account-id:geofence-collection/ExampleGeofenceCollection</code></p></li>
    /// </ul>
    pub fn set_collection_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.collection_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) for the geofence collection resource. Used when you need to specify a resource across all Amazon Web Services.</p>
    /// <ul>
    /// <li>
    /// <p>Format example: <code>arn:aws:geo:region:account-id:geofence-collection/ExampleGeofenceCollection</code></p></li>
    /// </ul>
    pub fn get_collection_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.collection_arn
    }
    /// <p>The timestamp for when the geofence collection was created in <a href="https://www.iso.org/iso-8601-date-and-time-format.html">ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code></p>
    /// This field is required.
    pub fn create_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.create_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The timestamp for when the geofence collection was created in <a href="https://www.iso.org/iso-8601-date-and-time-format.html">ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code></p>
    pub fn set_create_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.create_time = input;
        self
    }
    /// <p>The timestamp for when the geofence collection was created in <a href="https://www.iso.org/iso-8601-date-and-time-format.html">ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code></p>
    pub fn get_create_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.create_time
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`CreateGeofenceCollectionOutput`](crate::operation::create_geofence_collection::CreateGeofenceCollectionOutput).
    /// This method will fail if any of the following fields are not set:
    /// - [`collection_name`](crate::operation::create_geofence_collection::builders::CreateGeofenceCollectionOutputBuilder::collection_name)
    /// - [`collection_arn`](crate::operation::create_geofence_collection::builders::CreateGeofenceCollectionOutputBuilder::collection_arn)
    /// - [`create_time`](crate::operation::create_geofence_collection::builders::CreateGeofenceCollectionOutputBuilder::create_time)
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_geofence_collection::CreateGeofenceCollectionOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_geofence_collection::CreateGeofenceCollectionOutput {
            collection_name: self.collection_name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "collection_name",
                    "collection_name was not specified but it is required when building CreateGeofenceCollectionOutput",
                )
            })?,
            collection_arn: self.collection_arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "collection_arn",
                    "collection_arn was not specified but it is required when building CreateGeofenceCollectionOutput",
                )
            })?,
            create_time: self.create_time.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "create_time",
                    "create_time was not specified but it is required when building CreateGeofenceCollectionOutput",
                )
            })?,
            _request_id: self._request_id,
        })
    }
}
impl ::std::fmt::Debug for CreateGeofenceCollectionOutputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CreateGeofenceCollectionOutputBuilder");
        formatter.field("collection_name", &self.collection_name);
        formatter.field("collection_arn", &self.collection_arn);
        formatter.field("create_time", &"*** Sensitive Data Redacted ***");
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
