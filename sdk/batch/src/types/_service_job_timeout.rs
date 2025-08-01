// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The timeout configuration for service jobs.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ServiceJobTimeout {
    /// <p>The maximum duration in seconds that a service job attempt can run. After this time is reached, Batch terminates the service job attempt.</p>
    pub attempt_duration_seconds: ::std::option::Option<i32>,
}
impl ServiceJobTimeout {
    /// <p>The maximum duration in seconds that a service job attempt can run. After this time is reached, Batch terminates the service job attempt.</p>
    pub fn attempt_duration_seconds(&self) -> ::std::option::Option<i32> {
        self.attempt_duration_seconds
    }
}
impl ServiceJobTimeout {
    /// Creates a new builder-style object to manufacture [`ServiceJobTimeout`](crate::types::ServiceJobTimeout).
    pub fn builder() -> crate::types::builders::ServiceJobTimeoutBuilder {
        crate::types::builders::ServiceJobTimeoutBuilder::default()
    }
}

/// A builder for [`ServiceJobTimeout`](crate::types::ServiceJobTimeout).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ServiceJobTimeoutBuilder {
    pub(crate) attempt_duration_seconds: ::std::option::Option<i32>,
}
impl ServiceJobTimeoutBuilder {
    /// <p>The maximum duration in seconds that a service job attempt can run. After this time is reached, Batch terminates the service job attempt.</p>
    pub fn attempt_duration_seconds(mut self, input: i32) -> Self {
        self.attempt_duration_seconds = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum duration in seconds that a service job attempt can run. After this time is reached, Batch terminates the service job attempt.</p>
    pub fn set_attempt_duration_seconds(mut self, input: ::std::option::Option<i32>) -> Self {
        self.attempt_duration_seconds = input;
        self
    }
    /// <p>The maximum duration in seconds that a service job attempt can run. After this time is reached, Batch terminates the service job attempt.</p>
    pub fn get_attempt_duration_seconds(&self) -> &::std::option::Option<i32> {
        &self.attempt_duration_seconds
    }
    /// Consumes the builder and constructs a [`ServiceJobTimeout`](crate::types::ServiceJobTimeout).
    pub fn build(self) -> crate::types::ServiceJobTimeout {
        crate::types::ServiceJobTimeout {
            attempt_duration_seconds: self.attempt_duration_seconds,
        }
    }
}
