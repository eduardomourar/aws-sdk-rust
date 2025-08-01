// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct DescribeGeofenceCollectionOutput {
    /// <p>The name of the geofence collection.</p>
    pub collection_name: ::std::string::String,
    /// <p>The Amazon Resource Name (ARN) for the geofence collection resource. Used when you need to specify a resource across all Amazon Web Services.</p>
    /// <ul>
    /// <li>
    /// <p>Format example: <code>arn:aws:geo:region:account-id:geofence-collection/ExampleGeofenceCollection</code></p></li>
    /// </ul>
    pub collection_arn: ::std::string::String,
    /// <p>The optional description for the geofence collection.</p>
    pub description: ::std::string::String,
    /// <p>No longer used. Always returns <code>RequestBasedUsage</code>.</p>
    #[deprecated(note = "Deprecated. Always returns RequestBasedUsage.", since = "2022-02-01")]
    pub pricing_plan: ::std::option::Option<crate::types::PricingPlan>,
    /// <p>No longer used. Always returns an empty string.</p>
    #[deprecated(note = "Deprecated. Unused.", since = "2022-02-01")]
    pub pricing_plan_data_source: ::std::option::Option<::std::string::String>,
    /// <p>A key identifier for an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keys.html">Amazon Web Services KMS customer managed key</a> assigned to the Amazon Location resource</p>
    pub kms_key_id: ::std::option::Option<::std::string::String>,
    /// <p>Displays the key, value pairs of tags associated with this resource.</p>
    pub tags: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    /// <p>The timestamp for when the geofence resource was created in <a href="https://www.iso.org/iso-8601-date-and-time-format.html">ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code></p>
    pub create_time: ::aws_smithy_types::DateTime,
    /// <p>The timestamp for when the geofence collection was last updated in <a href="https://www.iso.org/iso-8601-date-and-time-format.html">ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code></p>
    pub update_time: ::aws_smithy_types::DateTime,
    /// <p>The number of geofences in the geofence collection.</p>
    pub geofence_count: ::std::option::Option<i32>,
    _request_id: Option<String>,
}
impl DescribeGeofenceCollectionOutput {
    /// <p>The name of the geofence collection.</p>
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
    /// <p>The optional description for the geofence collection.</p>
    pub fn description(&self) -> &str {
        use std::ops::Deref;
        self.description.deref()
    }
    /// <p>No longer used. Always returns <code>RequestBasedUsage</code>.</p>
    #[deprecated(note = "Deprecated. Always returns RequestBasedUsage.", since = "2022-02-01")]
    pub fn pricing_plan(&self) -> ::std::option::Option<&crate::types::PricingPlan> {
        self.pricing_plan.as_ref()
    }
    /// <p>No longer used. Always returns an empty string.</p>
    #[deprecated(note = "Deprecated. Unused.", since = "2022-02-01")]
    pub fn pricing_plan_data_source(&self) -> ::std::option::Option<&str> {
        self.pricing_plan_data_source.as_deref()
    }
    /// <p>A key identifier for an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keys.html">Amazon Web Services KMS customer managed key</a> assigned to the Amazon Location resource</p>
    pub fn kms_key_id(&self) -> ::std::option::Option<&str> {
        self.kms_key_id.as_deref()
    }
    /// <p>Displays the key, value pairs of tags associated with this resource.</p>
    pub fn tags(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.tags.as_ref()
    }
    /// <p>The timestamp for when the geofence resource was created in <a href="https://www.iso.org/iso-8601-date-and-time-format.html">ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code></p>
    pub fn create_time(&self) -> &::aws_smithy_types::DateTime {
        &self.create_time
    }
    /// <p>The timestamp for when the geofence collection was last updated in <a href="https://www.iso.org/iso-8601-date-and-time-format.html">ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code></p>
    pub fn update_time(&self) -> &::aws_smithy_types::DateTime {
        &self.update_time
    }
    /// <p>The number of geofences in the geofence collection.</p>
    pub fn geofence_count(&self) -> ::std::option::Option<i32> {
        self.geofence_count
    }
}
impl ::std::fmt::Debug for DescribeGeofenceCollectionOutput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeGeofenceCollectionOutput");
        formatter.field("collection_name", &self.collection_name);
        formatter.field("collection_arn", &self.collection_arn);
        formatter.field("description", &self.description);
        formatter.field("pricing_plan", &self.pricing_plan);
        formatter.field("pricing_plan_data_source", &self.pricing_plan_data_source);
        formatter.field("kms_key_id", &self.kms_key_id);
        formatter.field("tags", &self.tags);
        formatter.field("create_time", &"*** Sensitive Data Redacted ***");
        formatter.field("update_time", &"*** Sensitive Data Redacted ***");
        formatter.field("geofence_count", &self.geofence_count);
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
impl ::aws_types::request_id::RequestId for DescribeGeofenceCollectionOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeGeofenceCollectionOutput {
    /// Creates a new builder-style object to manufacture [`DescribeGeofenceCollectionOutput`](crate::operation::describe_geofence_collection::DescribeGeofenceCollectionOutput).
    pub fn builder() -> crate::operation::describe_geofence_collection::builders::DescribeGeofenceCollectionOutputBuilder {
        crate::operation::describe_geofence_collection::builders::DescribeGeofenceCollectionOutputBuilder::default()
    }
}

/// A builder for [`DescribeGeofenceCollectionOutput`](crate::operation::describe_geofence_collection::DescribeGeofenceCollectionOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct DescribeGeofenceCollectionOutputBuilder {
    pub(crate) collection_name: ::std::option::Option<::std::string::String>,
    pub(crate) collection_arn: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) pricing_plan: ::std::option::Option<crate::types::PricingPlan>,
    pub(crate) pricing_plan_data_source: ::std::option::Option<::std::string::String>,
    pub(crate) kms_key_id: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    pub(crate) create_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) update_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) geofence_count: ::std::option::Option<i32>,
    _request_id: Option<String>,
}
impl DescribeGeofenceCollectionOutputBuilder {
    /// <p>The name of the geofence collection.</p>
    /// This field is required.
    pub fn collection_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.collection_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the geofence collection.</p>
    pub fn set_collection_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.collection_name = input;
        self
    }
    /// <p>The name of the geofence collection.</p>
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
    /// <p>The optional description for the geofence collection.</p>
    /// This field is required.
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The optional description for the geofence collection.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The optional description for the geofence collection.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }
    /// <p>No longer used. Always returns <code>RequestBasedUsage</code>.</p>
    #[deprecated(note = "Deprecated. Always returns RequestBasedUsage.", since = "2022-02-01")]
    pub fn pricing_plan(mut self, input: crate::types::PricingPlan) -> Self {
        self.pricing_plan = ::std::option::Option::Some(input);
        self
    }
    /// <p>No longer used. Always returns <code>RequestBasedUsage</code>.</p>
    #[deprecated(note = "Deprecated. Always returns RequestBasedUsage.", since = "2022-02-01")]
    pub fn set_pricing_plan(mut self, input: ::std::option::Option<crate::types::PricingPlan>) -> Self {
        self.pricing_plan = input;
        self
    }
    /// <p>No longer used. Always returns <code>RequestBasedUsage</code>.</p>
    #[deprecated(note = "Deprecated. Always returns RequestBasedUsage.", since = "2022-02-01")]
    pub fn get_pricing_plan(&self) -> &::std::option::Option<crate::types::PricingPlan> {
        &self.pricing_plan
    }
    /// <p>No longer used. Always returns an empty string.</p>
    #[deprecated(note = "Deprecated. Unused.", since = "2022-02-01")]
    pub fn pricing_plan_data_source(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.pricing_plan_data_source = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>No longer used. Always returns an empty string.</p>
    #[deprecated(note = "Deprecated. Unused.", since = "2022-02-01")]
    pub fn set_pricing_plan_data_source(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.pricing_plan_data_source = input;
        self
    }
    /// <p>No longer used. Always returns an empty string.</p>
    #[deprecated(note = "Deprecated. Unused.", since = "2022-02-01")]
    pub fn get_pricing_plan_data_source(&self) -> &::std::option::Option<::std::string::String> {
        &self.pricing_plan_data_source
    }
    /// <p>A key identifier for an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keys.html">Amazon Web Services KMS customer managed key</a> assigned to the Amazon Location resource</p>
    pub fn kms_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.kms_key_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A key identifier for an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keys.html">Amazon Web Services KMS customer managed key</a> assigned to the Amazon Location resource</p>
    pub fn set_kms_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.kms_key_id = input;
        self
    }
    /// <p>A key identifier for an <a href="https://docs.aws.amazon.com/kms/latest/developerguide/create-keys.html">Amazon Web Services KMS customer managed key</a> assigned to the Amazon Location resource</p>
    pub fn get_kms_key_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.kms_key_id
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Displays the key, value pairs of tags associated with this resource.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>Displays the key, value pairs of tags associated with this resource.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.tags = input;
        self
    }
    /// <p>Displays the key, value pairs of tags associated with this resource.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        &self.tags
    }
    /// <p>The timestamp for when the geofence resource was created in <a href="https://www.iso.org/iso-8601-date-and-time-format.html">ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code></p>
    /// This field is required.
    pub fn create_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.create_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The timestamp for when the geofence resource was created in <a href="https://www.iso.org/iso-8601-date-and-time-format.html">ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code></p>
    pub fn set_create_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.create_time = input;
        self
    }
    /// <p>The timestamp for when the geofence resource was created in <a href="https://www.iso.org/iso-8601-date-and-time-format.html">ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code></p>
    pub fn get_create_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.create_time
    }
    /// <p>The timestamp for when the geofence collection was last updated in <a href="https://www.iso.org/iso-8601-date-and-time-format.html">ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code></p>
    /// This field is required.
    pub fn update_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.update_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The timestamp for when the geofence collection was last updated in <a href="https://www.iso.org/iso-8601-date-and-time-format.html">ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code></p>
    pub fn set_update_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.update_time = input;
        self
    }
    /// <p>The timestamp for when the geofence collection was last updated in <a href="https://www.iso.org/iso-8601-date-and-time-format.html">ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code></p>
    pub fn get_update_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.update_time
    }
    /// <p>The number of geofences in the geofence collection.</p>
    pub fn geofence_count(mut self, input: i32) -> Self {
        self.geofence_count = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of geofences in the geofence collection.</p>
    pub fn set_geofence_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.geofence_count = input;
        self
    }
    /// <p>The number of geofences in the geofence collection.</p>
    pub fn get_geofence_count(&self) -> &::std::option::Option<i32> {
        &self.geofence_count
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DescribeGeofenceCollectionOutput`](crate::operation::describe_geofence_collection::DescribeGeofenceCollectionOutput).
    /// This method will fail if any of the following fields are not set:
    /// - [`collection_name`](crate::operation::describe_geofence_collection::builders::DescribeGeofenceCollectionOutputBuilder::collection_name)
    /// - [`collection_arn`](crate::operation::describe_geofence_collection::builders::DescribeGeofenceCollectionOutputBuilder::collection_arn)
    /// - [`description`](crate::operation::describe_geofence_collection::builders::DescribeGeofenceCollectionOutputBuilder::description)
    /// - [`create_time`](crate::operation::describe_geofence_collection::builders::DescribeGeofenceCollectionOutputBuilder::create_time)
    /// - [`update_time`](crate::operation::describe_geofence_collection::builders::DescribeGeofenceCollectionOutputBuilder::update_time)
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_geofence_collection::DescribeGeofenceCollectionOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_geofence_collection::DescribeGeofenceCollectionOutput {
            collection_name: self.collection_name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "collection_name",
                    "collection_name was not specified but it is required when building DescribeGeofenceCollectionOutput",
                )
            })?,
            collection_arn: self.collection_arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "collection_arn",
                    "collection_arn was not specified but it is required when building DescribeGeofenceCollectionOutput",
                )
            })?,
            description: self.description.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "description",
                    "description was not specified but it is required when building DescribeGeofenceCollectionOutput",
                )
            })?,
            pricing_plan: self.pricing_plan,
            pricing_plan_data_source: self.pricing_plan_data_source,
            kms_key_id: self.kms_key_id,
            tags: self.tags,
            create_time: self.create_time.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "create_time",
                    "create_time was not specified but it is required when building DescribeGeofenceCollectionOutput",
                )
            })?,
            update_time: self.update_time.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "update_time",
                    "update_time was not specified but it is required when building DescribeGeofenceCollectionOutput",
                )
            })?,
            geofence_count: self.geofence_count,
            _request_id: self._request_id,
        })
    }
}
impl ::std::fmt::Debug for DescribeGeofenceCollectionOutputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeGeofenceCollectionOutputBuilder");
        formatter.field("collection_name", &self.collection_name);
        formatter.field("collection_arn", &self.collection_arn);
        formatter.field("description", &self.description);
        formatter.field("pricing_plan", &self.pricing_plan);
        formatter.field("pricing_plan_data_source", &self.pricing_plan_data_source);
        formatter.field("kms_key_id", &self.kms_key_id);
        formatter.field("tags", &self.tags);
        formatter.field("create_time", &"*** Sensitive Data Redacted ***");
        formatter.field("update_time", &"*** Sensitive Data Redacted ***");
        formatter.field("geofence_count", &self.geofence_count);
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
