// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A configuration object that specifies the destination of an event after Lambda processes it. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/invocation-async-retain-records.html#invocation-async-destinations">Adding a destination</a>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DestinationConfig {
    /// <p>The destination configuration for successful invocations. Not supported in <code>CreateEventSourceMapping</code> or <code>UpdateEventSourceMapping</code>.</p>
    pub on_success: ::std::option::Option<crate::types::OnSuccess>,
    /// <p>The destination configuration for failed invocations.</p>
    pub on_failure: ::std::option::Option<crate::types::OnFailure>,
}
impl DestinationConfig {
    /// <p>The destination configuration for successful invocations. Not supported in <code>CreateEventSourceMapping</code> or <code>UpdateEventSourceMapping</code>.</p>
    pub fn on_success(&self) -> ::std::option::Option<&crate::types::OnSuccess> {
        self.on_success.as_ref()
    }
    /// <p>The destination configuration for failed invocations.</p>
    pub fn on_failure(&self) -> ::std::option::Option<&crate::types::OnFailure> {
        self.on_failure.as_ref()
    }
}
impl DestinationConfig {
    /// Creates a new builder-style object to manufacture [`DestinationConfig`](crate::types::DestinationConfig).
    pub fn builder() -> crate::types::builders::DestinationConfigBuilder {
        crate::types::builders::DestinationConfigBuilder::default()
    }
}

/// A builder for [`DestinationConfig`](crate::types::DestinationConfig).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DestinationConfigBuilder {
    pub(crate) on_success: ::std::option::Option<crate::types::OnSuccess>,
    pub(crate) on_failure: ::std::option::Option<crate::types::OnFailure>,
}
impl DestinationConfigBuilder {
    /// <p>The destination configuration for successful invocations. Not supported in <code>CreateEventSourceMapping</code> or <code>UpdateEventSourceMapping</code>.</p>
    pub fn on_success(mut self, input: crate::types::OnSuccess) -> Self {
        self.on_success = ::std::option::Option::Some(input);
        self
    }
    /// <p>The destination configuration for successful invocations. Not supported in <code>CreateEventSourceMapping</code> or <code>UpdateEventSourceMapping</code>.</p>
    pub fn set_on_success(mut self, input: ::std::option::Option<crate::types::OnSuccess>) -> Self {
        self.on_success = input;
        self
    }
    /// <p>The destination configuration for successful invocations. Not supported in <code>CreateEventSourceMapping</code> or <code>UpdateEventSourceMapping</code>.</p>
    pub fn get_on_success(&self) -> &::std::option::Option<crate::types::OnSuccess> {
        &self.on_success
    }
    /// <p>The destination configuration for failed invocations.</p>
    pub fn on_failure(mut self, input: crate::types::OnFailure) -> Self {
        self.on_failure = ::std::option::Option::Some(input);
        self
    }
    /// <p>The destination configuration for failed invocations.</p>
    pub fn set_on_failure(mut self, input: ::std::option::Option<crate::types::OnFailure>) -> Self {
        self.on_failure = input;
        self
    }
    /// <p>The destination configuration for failed invocations.</p>
    pub fn get_on_failure(&self) -> &::std::option::Option<crate::types::OnFailure> {
        &self.on_failure
    }
    /// Consumes the builder and constructs a [`DestinationConfig`](crate::types::DestinationConfig).
    pub fn build(self) -> crate::types::DestinationConfig {
        crate::types::DestinationConfig {
            on_success: self.on_success,
            on_failure: self.on_failure,
        }
    }
}
