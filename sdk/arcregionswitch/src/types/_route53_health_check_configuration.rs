// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The Amazon Route 53 health check configuration.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Route53HealthCheckConfiguration {
    /// <p>The Amazon Route 53 health check configuration time out (in minutes).</p>
    pub timeout_minutes: i32,
    /// <p>The cross account role for the configuration.</p>
    pub cross_account_role: ::std::option::Option<::std::string::String>,
    /// <p>The external ID (secret key) for the configuration.</p>
    pub external_id: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Route 53 health check configuration hosted zone ID.</p>
    pub hosted_zone_id: ::std::string::String,
    /// <p>The Amazon Route 53 health check configuration record name.</p>
    pub record_name: ::std::string::String,
    /// <p>The Amazon Route 53 health check configuration record sets.</p>
    pub record_sets: ::std::option::Option<::std::vec::Vec<crate::types::Route53ResourceRecordSet>>,
}
impl Route53HealthCheckConfiguration {
    /// <p>The Amazon Route 53 health check configuration time out (in minutes).</p>
    pub fn timeout_minutes(&self) -> i32 {
        self.timeout_minutes
    }
    /// <p>The cross account role for the configuration.</p>
    pub fn cross_account_role(&self) -> ::std::option::Option<&str> {
        self.cross_account_role.as_deref()
    }
    /// <p>The external ID (secret key) for the configuration.</p>
    pub fn external_id(&self) -> ::std::option::Option<&str> {
        self.external_id.as_deref()
    }
    /// <p>The Amazon Route 53 health check configuration hosted zone ID.</p>
    pub fn hosted_zone_id(&self) -> &str {
        use std::ops::Deref;
        self.hosted_zone_id.deref()
    }
    /// <p>The Amazon Route 53 health check configuration record name.</p>
    pub fn record_name(&self) -> &str {
        use std::ops::Deref;
        self.record_name.deref()
    }
    /// <p>The Amazon Route 53 health check configuration record sets.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.record_sets.is_none()`.
    pub fn record_sets(&self) -> &[crate::types::Route53ResourceRecordSet] {
        self.record_sets.as_deref().unwrap_or_default()
    }
}
impl Route53HealthCheckConfiguration {
    /// Creates a new builder-style object to manufacture [`Route53HealthCheckConfiguration`](crate::types::Route53HealthCheckConfiguration).
    pub fn builder() -> crate::types::builders::Route53HealthCheckConfigurationBuilder {
        crate::types::builders::Route53HealthCheckConfigurationBuilder::default()
    }
}

/// A builder for [`Route53HealthCheckConfiguration`](crate::types::Route53HealthCheckConfiguration).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct Route53HealthCheckConfigurationBuilder {
    pub(crate) timeout_minutes: ::std::option::Option<i32>,
    pub(crate) cross_account_role: ::std::option::Option<::std::string::String>,
    pub(crate) external_id: ::std::option::Option<::std::string::String>,
    pub(crate) hosted_zone_id: ::std::option::Option<::std::string::String>,
    pub(crate) record_name: ::std::option::Option<::std::string::String>,
    pub(crate) record_sets: ::std::option::Option<::std::vec::Vec<crate::types::Route53ResourceRecordSet>>,
}
impl Route53HealthCheckConfigurationBuilder {
    /// <p>The Amazon Route 53 health check configuration time out (in minutes).</p>
    pub fn timeout_minutes(mut self, input: i32) -> Self {
        self.timeout_minutes = ::std::option::Option::Some(input);
        self
    }
    /// <p>The Amazon Route 53 health check configuration time out (in minutes).</p>
    pub fn set_timeout_minutes(mut self, input: ::std::option::Option<i32>) -> Self {
        self.timeout_minutes = input;
        self
    }
    /// <p>The Amazon Route 53 health check configuration time out (in minutes).</p>
    pub fn get_timeout_minutes(&self) -> &::std::option::Option<i32> {
        &self.timeout_minutes
    }
    /// <p>The cross account role for the configuration.</p>
    pub fn cross_account_role(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.cross_account_role = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The cross account role for the configuration.</p>
    pub fn set_cross_account_role(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.cross_account_role = input;
        self
    }
    /// <p>The cross account role for the configuration.</p>
    pub fn get_cross_account_role(&self) -> &::std::option::Option<::std::string::String> {
        &self.cross_account_role
    }
    /// <p>The external ID (secret key) for the configuration.</p>
    pub fn external_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.external_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The external ID (secret key) for the configuration.</p>
    pub fn set_external_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.external_id = input;
        self
    }
    /// <p>The external ID (secret key) for the configuration.</p>
    pub fn get_external_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.external_id
    }
    /// <p>The Amazon Route 53 health check configuration hosted zone ID.</p>
    /// This field is required.
    pub fn hosted_zone_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.hosted_zone_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Route 53 health check configuration hosted zone ID.</p>
    pub fn set_hosted_zone_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.hosted_zone_id = input;
        self
    }
    /// <p>The Amazon Route 53 health check configuration hosted zone ID.</p>
    pub fn get_hosted_zone_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.hosted_zone_id
    }
    /// <p>The Amazon Route 53 health check configuration record name.</p>
    /// This field is required.
    pub fn record_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.record_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Route 53 health check configuration record name.</p>
    pub fn set_record_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.record_name = input;
        self
    }
    /// <p>The Amazon Route 53 health check configuration record name.</p>
    pub fn get_record_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.record_name
    }
    /// Appends an item to `record_sets`.
    ///
    /// To override the contents of this collection use [`set_record_sets`](Self::set_record_sets).
    ///
    /// <p>The Amazon Route 53 health check configuration record sets.</p>
    pub fn record_sets(mut self, input: crate::types::Route53ResourceRecordSet) -> Self {
        let mut v = self.record_sets.unwrap_or_default();
        v.push(input);
        self.record_sets = ::std::option::Option::Some(v);
        self
    }
    /// <p>The Amazon Route 53 health check configuration record sets.</p>
    pub fn set_record_sets(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Route53ResourceRecordSet>>) -> Self {
        self.record_sets = input;
        self
    }
    /// <p>The Amazon Route 53 health check configuration record sets.</p>
    pub fn get_record_sets(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Route53ResourceRecordSet>> {
        &self.record_sets
    }
    /// Consumes the builder and constructs a [`Route53HealthCheckConfiguration`](crate::types::Route53HealthCheckConfiguration).
    /// This method will fail if any of the following fields are not set:
    /// - [`hosted_zone_id`](crate::types::builders::Route53HealthCheckConfigurationBuilder::hosted_zone_id)
    /// - [`record_name`](crate::types::builders::Route53HealthCheckConfigurationBuilder::record_name)
    pub fn build(self) -> ::std::result::Result<crate::types::Route53HealthCheckConfiguration, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::Route53HealthCheckConfiguration {
            timeout_minutes: self.timeout_minutes.unwrap_or(60),
            cross_account_role: self.cross_account_role,
            external_id: self.external_id,
            hosted_zone_id: self.hosted_zone_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "hosted_zone_id",
                    "hosted_zone_id was not specified but it is required when building Route53HealthCheckConfiguration",
                )
            })?,
            record_name: self.record_name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "record_name",
                    "record_name was not specified but it is required when building Route53HealthCheckConfiguration",
                )
            })?,
            record_sets: self.record_sets,
        })
    }
}
