// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The configuration for an Amazon Web Services ECS capacity increase.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EcsCapacityIncreaseConfiguration {
    /// <p>The timeout value specified for the configuration.</p>
    pub timeout_minutes: i32,
    /// <p>The services specified for the configuration.</p>
    pub services: ::std::vec::Vec<crate::types::Service>,
    /// <p>The settings for ungraceful execution.</p>
    pub ungraceful: ::std::option::Option<crate::types::EcsUngraceful>,
    /// <p>The target percentage specified for the configuration.</p>
    pub target_percent: i32,
    /// <p>The monitoring approach specified for the configuration, for example, <code>Most_Recent</code>.</p>
    pub capacity_monitoring_approach: crate::types::EcsCapacityMonitoringApproach,
}
impl EcsCapacityIncreaseConfiguration {
    /// <p>The timeout value specified for the configuration.</p>
    pub fn timeout_minutes(&self) -> i32 {
        self.timeout_minutes
    }
    /// <p>The services specified for the configuration.</p>
    pub fn services(&self) -> &[crate::types::Service] {
        use std::ops::Deref;
        self.services.deref()
    }
    /// <p>The settings for ungraceful execution.</p>
    pub fn ungraceful(&self) -> ::std::option::Option<&crate::types::EcsUngraceful> {
        self.ungraceful.as_ref()
    }
    /// <p>The target percentage specified for the configuration.</p>
    pub fn target_percent(&self) -> i32 {
        self.target_percent
    }
    /// <p>The monitoring approach specified for the configuration, for example, <code>Most_Recent</code>.</p>
    pub fn capacity_monitoring_approach(&self) -> &crate::types::EcsCapacityMonitoringApproach {
        &self.capacity_monitoring_approach
    }
}
impl EcsCapacityIncreaseConfiguration {
    /// Creates a new builder-style object to manufacture [`EcsCapacityIncreaseConfiguration`](crate::types::EcsCapacityIncreaseConfiguration).
    pub fn builder() -> crate::types::builders::EcsCapacityIncreaseConfigurationBuilder {
        crate::types::builders::EcsCapacityIncreaseConfigurationBuilder::default()
    }
}

/// A builder for [`EcsCapacityIncreaseConfiguration`](crate::types::EcsCapacityIncreaseConfiguration).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct EcsCapacityIncreaseConfigurationBuilder {
    pub(crate) timeout_minutes: ::std::option::Option<i32>,
    pub(crate) services: ::std::option::Option<::std::vec::Vec<crate::types::Service>>,
    pub(crate) ungraceful: ::std::option::Option<crate::types::EcsUngraceful>,
    pub(crate) target_percent: ::std::option::Option<i32>,
    pub(crate) capacity_monitoring_approach: ::std::option::Option<crate::types::EcsCapacityMonitoringApproach>,
}
impl EcsCapacityIncreaseConfigurationBuilder {
    /// <p>The timeout value specified for the configuration.</p>
    pub fn timeout_minutes(mut self, input: i32) -> Self {
        self.timeout_minutes = ::std::option::Option::Some(input);
        self
    }
    /// <p>The timeout value specified for the configuration.</p>
    pub fn set_timeout_minutes(mut self, input: ::std::option::Option<i32>) -> Self {
        self.timeout_minutes = input;
        self
    }
    /// <p>The timeout value specified for the configuration.</p>
    pub fn get_timeout_minutes(&self) -> &::std::option::Option<i32> {
        &self.timeout_minutes
    }
    /// Appends an item to `services`.
    ///
    /// To override the contents of this collection use [`set_services`](Self::set_services).
    ///
    /// <p>The services specified for the configuration.</p>
    pub fn services(mut self, input: crate::types::Service) -> Self {
        let mut v = self.services.unwrap_or_default();
        v.push(input);
        self.services = ::std::option::Option::Some(v);
        self
    }
    /// <p>The services specified for the configuration.</p>
    pub fn set_services(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Service>>) -> Self {
        self.services = input;
        self
    }
    /// <p>The services specified for the configuration.</p>
    pub fn get_services(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Service>> {
        &self.services
    }
    /// <p>The settings for ungraceful execution.</p>
    pub fn ungraceful(mut self, input: crate::types::EcsUngraceful) -> Self {
        self.ungraceful = ::std::option::Option::Some(input);
        self
    }
    /// <p>The settings for ungraceful execution.</p>
    pub fn set_ungraceful(mut self, input: ::std::option::Option<crate::types::EcsUngraceful>) -> Self {
        self.ungraceful = input;
        self
    }
    /// <p>The settings for ungraceful execution.</p>
    pub fn get_ungraceful(&self) -> &::std::option::Option<crate::types::EcsUngraceful> {
        &self.ungraceful
    }
    /// <p>The target percentage specified for the configuration.</p>
    pub fn target_percent(mut self, input: i32) -> Self {
        self.target_percent = ::std::option::Option::Some(input);
        self
    }
    /// <p>The target percentage specified for the configuration.</p>
    pub fn set_target_percent(mut self, input: ::std::option::Option<i32>) -> Self {
        self.target_percent = input;
        self
    }
    /// <p>The target percentage specified for the configuration.</p>
    pub fn get_target_percent(&self) -> &::std::option::Option<i32> {
        &self.target_percent
    }
    /// <p>The monitoring approach specified for the configuration, for example, <code>Most_Recent</code>.</p>
    pub fn capacity_monitoring_approach(mut self, input: crate::types::EcsCapacityMonitoringApproach) -> Self {
        self.capacity_monitoring_approach = ::std::option::Option::Some(input);
        self
    }
    /// <p>The monitoring approach specified for the configuration, for example, <code>Most_Recent</code>.</p>
    pub fn set_capacity_monitoring_approach(mut self, input: ::std::option::Option<crate::types::EcsCapacityMonitoringApproach>) -> Self {
        self.capacity_monitoring_approach = input;
        self
    }
    /// <p>The monitoring approach specified for the configuration, for example, <code>Most_Recent</code>.</p>
    pub fn get_capacity_monitoring_approach(&self) -> &::std::option::Option<crate::types::EcsCapacityMonitoringApproach> {
        &self.capacity_monitoring_approach
    }
    /// Consumes the builder and constructs a [`EcsCapacityIncreaseConfiguration`](crate::types::EcsCapacityIncreaseConfiguration).
    /// This method will fail if any of the following fields are not set:
    /// - [`services`](crate::types::builders::EcsCapacityIncreaseConfigurationBuilder::services)
    pub fn build(self) -> ::std::result::Result<crate::types::EcsCapacityIncreaseConfiguration, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::EcsCapacityIncreaseConfiguration {
            timeout_minutes: self.timeout_minutes.unwrap_or(60),
            services: self.services.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "services",
                    "services was not specified but it is required when building EcsCapacityIncreaseConfiguration",
                )
            })?,
            ungraceful: self.ungraceful,
            target_percent: self.target_percent.unwrap_or(100),
            capacity_monitoring_approach: self.capacity_monitoring_approach.unwrap_or(
                "sampledMaxInLast24Hours"
                    .parse::<crate::types::EcsCapacityMonitoringApproach>()
                    .expect("static value validated to member"),
            ),
        })
    }
}
