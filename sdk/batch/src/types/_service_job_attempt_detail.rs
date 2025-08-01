// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Detailed information about an attempt to run a service job.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ServiceJobAttemptDetail {
    /// <p>The service resource identifier associated with the service job attempt.</p>
    pub service_resource_id: ::std::option::Option<crate::types::ServiceResourceId>,
    /// <p>The Unix timestamp (in milliseconds) for when the service job attempt was started.</p>
    pub started_at: ::std::option::Option<i64>,
    /// <p>The Unix timestamp (in milliseconds) for when the service job attempt stopped running.</p>
    pub stopped_at: ::std::option::Option<i64>,
    /// <p>A string that provides additional details for the current status of the service job attempt.</p>
    pub status_reason: ::std::option::Option<::std::string::String>,
}
impl ServiceJobAttemptDetail {
    /// <p>The service resource identifier associated with the service job attempt.</p>
    pub fn service_resource_id(&self) -> ::std::option::Option<&crate::types::ServiceResourceId> {
        self.service_resource_id.as_ref()
    }
    /// <p>The Unix timestamp (in milliseconds) for when the service job attempt was started.</p>
    pub fn started_at(&self) -> ::std::option::Option<i64> {
        self.started_at
    }
    /// <p>The Unix timestamp (in milliseconds) for when the service job attempt stopped running.</p>
    pub fn stopped_at(&self) -> ::std::option::Option<i64> {
        self.stopped_at
    }
    /// <p>A string that provides additional details for the current status of the service job attempt.</p>
    pub fn status_reason(&self) -> ::std::option::Option<&str> {
        self.status_reason.as_deref()
    }
}
impl ServiceJobAttemptDetail {
    /// Creates a new builder-style object to manufacture [`ServiceJobAttemptDetail`](crate::types::ServiceJobAttemptDetail).
    pub fn builder() -> crate::types::builders::ServiceJobAttemptDetailBuilder {
        crate::types::builders::ServiceJobAttemptDetailBuilder::default()
    }
}

/// A builder for [`ServiceJobAttemptDetail`](crate::types::ServiceJobAttemptDetail).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ServiceJobAttemptDetailBuilder {
    pub(crate) service_resource_id: ::std::option::Option<crate::types::ServiceResourceId>,
    pub(crate) started_at: ::std::option::Option<i64>,
    pub(crate) stopped_at: ::std::option::Option<i64>,
    pub(crate) status_reason: ::std::option::Option<::std::string::String>,
}
impl ServiceJobAttemptDetailBuilder {
    /// <p>The service resource identifier associated with the service job attempt.</p>
    pub fn service_resource_id(mut self, input: crate::types::ServiceResourceId) -> Self {
        self.service_resource_id = ::std::option::Option::Some(input);
        self
    }
    /// <p>The service resource identifier associated with the service job attempt.</p>
    pub fn set_service_resource_id(mut self, input: ::std::option::Option<crate::types::ServiceResourceId>) -> Self {
        self.service_resource_id = input;
        self
    }
    /// <p>The service resource identifier associated with the service job attempt.</p>
    pub fn get_service_resource_id(&self) -> &::std::option::Option<crate::types::ServiceResourceId> {
        &self.service_resource_id
    }
    /// <p>The Unix timestamp (in milliseconds) for when the service job attempt was started.</p>
    pub fn started_at(mut self, input: i64) -> Self {
        self.started_at = ::std::option::Option::Some(input);
        self
    }
    /// <p>The Unix timestamp (in milliseconds) for when the service job attempt was started.</p>
    pub fn set_started_at(mut self, input: ::std::option::Option<i64>) -> Self {
        self.started_at = input;
        self
    }
    /// <p>The Unix timestamp (in milliseconds) for when the service job attempt was started.</p>
    pub fn get_started_at(&self) -> &::std::option::Option<i64> {
        &self.started_at
    }
    /// <p>The Unix timestamp (in milliseconds) for when the service job attempt stopped running.</p>
    pub fn stopped_at(mut self, input: i64) -> Self {
        self.stopped_at = ::std::option::Option::Some(input);
        self
    }
    /// <p>The Unix timestamp (in milliseconds) for when the service job attempt stopped running.</p>
    pub fn set_stopped_at(mut self, input: ::std::option::Option<i64>) -> Self {
        self.stopped_at = input;
        self
    }
    /// <p>The Unix timestamp (in milliseconds) for when the service job attempt stopped running.</p>
    pub fn get_stopped_at(&self) -> &::std::option::Option<i64> {
        &self.stopped_at
    }
    /// <p>A string that provides additional details for the current status of the service job attempt.</p>
    pub fn status_reason(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.status_reason = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A string that provides additional details for the current status of the service job attempt.</p>
    pub fn set_status_reason(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.status_reason = input;
        self
    }
    /// <p>A string that provides additional details for the current status of the service job attempt.</p>
    pub fn get_status_reason(&self) -> &::std::option::Option<::std::string::String> {
        &self.status_reason
    }
    /// Consumes the builder and constructs a [`ServiceJobAttemptDetail`](crate::types::ServiceJobAttemptDetail).
    pub fn build(self) -> crate::types::ServiceJobAttemptDetail {
        crate::types::ServiceJobAttemptDetail {
            service_resource_id: self.service_resource_id,
            started_at: self.started_at,
            stopped_at: self.stopped_at,
            status_reason: self.status_reason,
        }
    }
}
