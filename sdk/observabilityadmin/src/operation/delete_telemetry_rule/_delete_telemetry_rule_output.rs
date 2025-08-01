// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteTelemetryRuleOutput {
    _request_id: Option<String>,
}
impl ::aws_types::request_id::RequestId for DeleteTelemetryRuleOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteTelemetryRuleOutput {
    /// Creates a new builder-style object to manufacture [`DeleteTelemetryRuleOutput`](crate::operation::delete_telemetry_rule::DeleteTelemetryRuleOutput).
    pub fn builder() -> crate::operation::delete_telemetry_rule::builders::DeleteTelemetryRuleOutputBuilder {
        crate::operation::delete_telemetry_rule::builders::DeleteTelemetryRuleOutputBuilder::default()
    }
}

/// A builder for [`DeleteTelemetryRuleOutput`](crate::operation::delete_telemetry_rule::DeleteTelemetryRuleOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DeleteTelemetryRuleOutputBuilder {
    _request_id: Option<String>,
}
impl DeleteTelemetryRuleOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DeleteTelemetryRuleOutput`](crate::operation::delete_telemetry_rule::DeleteTelemetryRuleOutput).
    pub fn build(self) -> crate::operation::delete_telemetry_rule::DeleteTelemetryRuleOutput {
        crate::operation::delete_telemetry_rule::DeleteTelemetryRuleOutput {
            _request_id: self._request_id,
        }
    }
}
