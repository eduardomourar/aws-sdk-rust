// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct DescribeKeyOutput {
    /// <p>The key value/string of an API key.</p>
    pub key: ::std::string::String,
    /// <p>The Amazon Resource Name (ARN) for the API key resource. Used when you need to specify a resource across all Amazon Web Services.</p>
    /// <ul>
    /// <li>
    /// <p>Format example: <code>arn:aws:geo:region:account-id:key/ExampleKey</code></p></li>
    /// </ul>
    pub key_arn: ::std::string::String,
    /// <p>The name of the API key resource.</p>
    pub key_name: ::std::string::String,
    /// <p>API Restrictions on the allowed actions, resources, and referers for an API key resource.</p>
    pub restrictions: ::std::option::Option<crate::types::ApiKeyRestrictions>,
    /// <p>The timestamp for when the API key resource was created in <a href="https://www.iso.org/iso-8601-date-and-time-format.html"> ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code>.</p>
    pub create_time: ::aws_smithy_types::DateTime,
    /// <p>The timestamp for when the API key resource will expire in <a href="https://www.iso.org/iso-8601-date-and-time-format.html"> ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code>.</p>
    pub expire_time: ::aws_smithy_types::DateTime,
    /// <p>The timestamp for when the API key resource was last updated in <a href="https://www.iso.org/iso-8601-date-and-time-format.html"> ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code>.</p>
    pub update_time: ::aws_smithy_types::DateTime,
    /// <p>The optional description for the API key resource.</p>
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>Tags associated with the API key resource.</p>
    pub tags: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    _request_id: Option<String>,
}
impl DescribeKeyOutput {
    /// <p>The key value/string of an API key.</p>
    pub fn key(&self) -> &str {
        use std::ops::Deref;
        self.key.deref()
    }
    /// <p>The Amazon Resource Name (ARN) for the API key resource. Used when you need to specify a resource across all Amazon Web Services.</p>
    /// <ul>
    /// <li>
    /// <p>Format example: <code>arn:aws:geo:region:account-id:key/ExampleKey</code></p></li>
    /// </ul>
    pub fn key_arn(&self) -> &str {
        use std::ops::Deref;
        self.key_arn.deref()
    }
    /// <p>The name of the API key resource.</p>
    pub fn key_name(&self) -> &str {
        use std::ops::Deref;
        self.key_name.deref()
    }
    /// <p>API Restrictions on the allowed actions, resources, and referers for an API key resource.</p>
    pub fn restrictions(&self) -> ::std::option::Option<&crate::types::ApiKeyRestrictions> {
        self.restrictions.as_ref()
    }
    /// <p>The timestamp for when the API key resource was created in <a href="https://www.iso.org/iso-8601-date-and-time-format.html"> ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code>.</p>
    pub fn create_time(&self) -> &::aws_smithy_types::DateTime {
        &self.create_time
    }
    /// <p>The timestamp for when the API key resource will expire in <a href="https://www.iso.org/iso-8601-date-and-time-format.html"> ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code>.</p>
    pub fn expire_time(&self) -> &::aws_smithy_types::DateTime {
        &self.expire_time
    }
    /// <p>The timestamp for when the API key resource was last updated in <a href="https://www.iso.org/iso-8601-date-and-time-format.html"> ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code>.</p>
    pub fn update_time(&self) -> &::aws_smithy_types::DateTime {
        &self.update_time
    }
    /// <p>The optional description for the API key resource.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>Tags associated with the API key resource.</p>
    pub fn tags(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.tags.as_ref()
    }
}
impl ::std::fmt::Debug for DescribeKeyOutput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeKeyOutput");
        formatter.field("key", &"*** Sensitive Data Redacted ***");
        formatter.field("key_arn", &self.key_arn);
        formatter.field("key_name", &self.key_name);
        formatter.field("restrictions", &self.restrictions);
        formatter.field("create_time", &"*** Sensitive Data Redacted ***");
        formatter.field("expire_time", &"*** Sensitive Data Redacted ***");
        formatter.field("update_time", &"*** Sensitive Data Redacted ***");
        formatter.field("description", &self.description);
        formatter.field("tags", &self.tags);
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
impl ::aws_types::request_id::RequestId for DescribeKeyOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeKeyOutput {
    /// Creates a new builder-style object to manufacture [`DescribeKeyOutput`](crate::operation::describe_key::DescribeKeyOutput).
    pub fn builder() -> crate::operation::describe_key::builders::DescribeKeyOutputBuilder {
        crate::operation::describe_key::builders::DescribeKeyOutputBuilder::default()
    }
}

/// A builder for [`DescribeKeyOutput`](crate::operation::describe_key::DescribeKeyOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct DescribeKeyOutputBuilder {
    pub(crate) key: ::std::option::Option<::std::string::String>,
    pub(crate) key_arn: ::std::option::Option<::std::string::String>,
    pub(crate) key_name: ::std::option::Option<::std::string::String>,
    pub(crate) restrictions: ::std::option::Option<crate::types::ApiKeyRestrictions>,
    pub(crate) create_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) expire_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) update_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    _request_id: Option<String>,
}
impl DescribeKeyOutputBuilder {
    /// <p>The key value/string of an API key.</p>
    /// This field is required.
    pub fn key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The key value/string of an API key.</p>
    pub fn set_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key = input;
        self
    }
    /// <p>The key value/string of an API key.</p>
    pub fn get_key(&self) -> &::std::option::Option<::std::string::String> {
        &self.key
    }
    /// <p>The Amazon Resource Name (ARN) for the API key resource. Used when you need to specify a resource across all Amazon Web Services.</p>
    /// <ul>
    /// <li>
    /// <p>Format example: <code>arn:aws:geo:region:account-id:key/ExampleKey</code></p></li>
    /// </ul>
    /// This field is required.
    pub fn key_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) for the API key resource. Used when you need to specify a resource across all Amazon Web Services.</p>
    /// <ul>
    /// <li>
    /// <p>Format example: <code>arn:aws:geo:region:account-id:key/ExampleKey</code></p></li>
    /// </ul>
    pub fn set_key_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) for the API key resource. Used when you need to specify a resource across all Amazon Web Services.</p>
    /// <ul>
    /// <li>
    /// <p>Format example: <code>arn:aws:geo:region:account-id:key/ExampleKey</code></p></li>
    /// </ul>
    pub fn get_key_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.key_arn
    }
    /// <p>The name of the API key resource.</p>
    /// This field is required.
    pub fn key_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the API key resource.</p>
    pub fn set_key_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key_name = input;
        self
    }
    /// <p>The name of the API key resource.</p>
    pub fn get_key_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.key_name
    }
    /// <p>API Restrictions on the allowed actions, resources, and referers for an API key resource.</p>
    /// This field is required.
    pub fn restrictions(mut self, input: crate::types::ApiKeyRestrictions) -> Self {
        self.restrictions = ::std::option::Option::Some(input);
        self
    }
    /// <p>API Restrictions on the allowed actions, resources, and referers for an API key resource.</p>
    pub fn set_restrictions(mut self, input: ::std::option::Option<crate::types::ApiKeyRestrictions>) -> Self {
        self.restrictions = input;
        self
    }
    /// <p>API Restrictions on the allowed actions, resources, and referers for an API key resource.</p>
    pub fn get_restrictions(&self) -> &::std::option::Option<crate::types::ApiKeyRestrictions> {
        &self.restrictions
    }
    /// <p>The timestamp for when the API key resource was created in <a href="https://www.iso.org/iso-8601-date-and-time-format.html"> ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code>.</p>
    /// This field is required.
    pub fn create_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.create_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The timestamp for when the API key resource was created in <a href="https://www.iso.org/iso-8601-date-and-time-format.html"> ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code>.</p>
    pub fn set_create_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.create_time = input;
        self
    }
    /// <p>The timestamp for when the API key resource was created in <a href="https://www.iso.org/iso-8601-date-and-time-format.html"> ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code>.</p>
    pub fn get_create_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.create_time
    }
    /// <p>The timestamp for when the API key resource will expire in <a href="https://www.iso.org/iso-8601-date-and-time-format.html"> ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code>.</p>
    /// This field is required.
    pub fn expire_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.expire_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The timestamp for when the API key resource will expire in <a href="https://www.iso.org/iso-8601-date-and-time-format.html"> ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code>.</p>
    pub fn set_expire_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.expire_time = input;
        self
    }
    /// <p>The timestamp for when the API key resource will expire in <a href="https://www.iso.org/iso-8601-date-and-time-format.html"> ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code>.</p>
    pub fn get_expire_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.expire_time
    }
    /// <p>The timestamp for when the API key resource was last updated in <a href="https://www.iso.org/iso-8601-date-and-time-format.html"> ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code>.</p>
    /// This field is required.
    pub fn update_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.update_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The timestamp for when the API key resource was last updated in <a href="https://www.iso.org/iso-8601-date-and-time-format.html"> ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code>.</p>
    pub fn set_update_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.update_time = input;
        self
    }
    /// <p>The timestamp for when the API key resource was last updated in <a href="https://www.iso.org/iso-8601-date-and-time-format.html"> ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code>.</p>
    pub fn get_update_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.update_time
    }
    /// <p>The optional description for the API key resource.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The optional description for the API key resource.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The optional description for the API key resource.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Tags associated with the API key resource.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>Tags associated with the API key resource.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.tags = input;
        self
    }
    /// <p>Tags associated with the API key resource.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        &self.tags
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DescribeKeyOutput`](crate::operation::describe_key::DescribeKeyOutput).
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](crate::operation::describe_key::builders::DescribeKeyOutputBuilder::key)
    /// - [`key_arn`](crate::operation::describe_key::builders::DescribeKeyOutputBuilder::key_arn)
    /// - [`key_name`](crate::operation::describe_key::builders::DescribeKeyOutputBuilder::key_name)
    /// - [`create_time`](crate::operation::describe_key::builders::DescribeKeyOutputBuilder::create_time)
    /// - [`expire_time`](crate::operation::describe_key::builders::DescribeKeyOutputBuilder::expire_time)
    /// - [`update_time`](crate::operation::describe_key::builders::DescribeKeyOutputBuilder::update_time)
    pub fn build(self) -> ::std::result::Result<crate::operation::describe_key::DescribeKeyOutput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::describe_key::DescribeKeyOutput {
            key: self.key.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "key",
                    "key was not specified but it is required when building DescribeKeyOutput",
                )
            })?,
            key_arn: self.key_arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "key_arn",
                    "key_arn was not specified but it is required when building DescribeKeyOutput",
                )
            })?,
            key_name: self.key_name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "key_name",
                    "key_name was not specified but it is required when building DescribeKeyOutput",
                )
            })?,
            restrictions: self.restrictions,
            create_time: self.create_time.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "create_time",
                    "create_time was not specified but it is required when building DescribeKeyOutput",
                )
            })?,
            expire_time: self.expire_time.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "expire_time",
                    "expire_time was not specified but it is required when building DescribeKeyOutput",
                )
            })?,
            update_time: self.update_time.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "update_time",
                    "update_time was not specified but it is required when building DescribeKeyOutput",
                )
            })?,
            description: self.description,
            tags: self.tags,
            _request_id: self._request_id,
        })
    }
}
impl ::std::fmt::Debug for DescribeKeyOutputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("DescribeKeyOutputBuilder");
        formatter.field("key", &"*** Sensitive Data Redacted ***");
        formatter.field("key_arn", &self.key_arn);
        formatter.field("key_name", &self.key_name);
        formatter.field("restrictions", &self.restrictions);
        formatter.field("create_time", &"*** Sensitive Data Redacted ***");
        formatter.field("expire_time", &"*** Sensitive Data Redacted ***");
        formatter.field("update_time", &"*** Sensitive Data Redacted ***");
        formatter.field("description", &self.description);
        formatter.field("tags", &self.tags);
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
