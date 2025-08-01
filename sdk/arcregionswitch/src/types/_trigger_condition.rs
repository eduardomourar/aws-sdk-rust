// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Defines a condition that must be met for a trigger to fire.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TriggerCondition {
    /// <p>The name of the CloudWatch alarm associated with the condition.</p>
    pub associated_alarm_name: ::std::string::String,
    /// <p>The condition that must be met. Valid values include ALARM and OK.</p>
    pub condition: crate::types::AlarmCondition,
}
impl TriggerCondition {
    /// <p>The name of the CloudWatch alarm associated with the condition.</p>
    pub fn associated_alarm_name(&self) -> &str {
        use std::ops::Deref;
        self.associated_alarm_name.deref()
    }
    /// <p>The condition that must be met. Valid values include ALARM and OK.</p>
    pub fn condition(&self) -> &crate::types::AlarmCondition {
        &self.condition
    }
}
impl TriggerCondition {
    /// Creates a new builder-style object to manufacture [`TriggerCondition`](crate::types::TriggerCondition).
    pub fn builder() -> crate::types::builders::TriggerConditionBuilder {
        crate::types::builders::TriggerConditionBuilder::default()
    }
}

/// A builder for [`TriggerCondition`](crate::types::TriggerCondition).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct TriggerConditionBuilder {
    pub(crate) associated_alarm_name: ::std::option::Option<::std::string::String>,
    pub(crate) condition: ::std::option::Option<crate::types::AlarmCondition>,
}
impl TriggerConditionBuilder {
    /// <p>The name of the CloudWatch alarm associated with the condition.</p>
    /// This field is required.
    pub fn associated_alarm_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.associated_alarm_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the CloudWatch alarm associated with the condition.</p>
    pub fn set_associated_alarm_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.associated_alarm_name = input;
        self
    }
    /// <p>The name of the CloudWatch alarm associated with the condition.</p>
    pub fn get_associated_alarm_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.associated_alarm_name
    }
    /// <p>The condition that must be met. Valid values include ALARM and OK.</p>
    /// This field is required.
    pub fn condition(mut self, input: crate::types::AlarmCondition) -> Self {
        self.condition = ::std::option::Option::Some(input);
        self
    }
    /// <p>The condition that must be met. Valid values include ALARM and OK.</p>
    pub fn set_condition(mut self, input: ::std::option::Option<crate::types::AlarmCondition>) -> Self {
        self.condition = input;
        self
    }
    /// <p>The condition that must be met. Valid values include ALARM and OK.</p>
    pub fn get_condition(&self) -> &::std::option::Option<crate::types::AlarmCondition> {
        &self.condition
    }
    /// Consumes the builder and constructs a [`TriggerCondition`](crate::types::TriggerCondition).
    /// This method will fail if any of the following fields are not set:
    /// - [`associated_alarm_name`](crate::types::builders::TriggerConditionBuilder::associated_alarm_name)
    /// - [`condition`](crate::types::builders::TriggerConditionBuilder::condition)
    pub fn build(self) -> ::std::result::Result<crate::types::TriggerCondition, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::TriggerCondition {
            associated_alarm_name: self.associated_alarm_name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "associated_alarm_name",
                    "associated_alarm_name was not specified but it is required when building TriggerCondition",
                )
            })?,
            condition: self.condition.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "condition",
                    "condition was not specified but it is required when building TriggerCondition",
                )
            })?,
        })
    }
}
