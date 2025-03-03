// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetTelemetryEvaluationStatus`](crate::operation::get_telemetry_evaluation_status::builders::GetTelemetryEvaluationStatusFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::get_telemetry_evaluation_status::builders::GetTelemetryEvaluationStatusFluentBuilder::send) it.
    /// - On success, responds with [`GetTelemetryEvaluationStatusOutput`](crate::operation::get_telemetry_evaluation_status::GetTelemetryEvaluationStatusOutput) with field(s):
    ///   - [`status(Option<Status>)`](crate::operation::get_telemetry_evaluation_status::GetTelemetryEvaluationStatusOutput::status): <p>The onboarding status of the telemetry config feature.</p>
    ///   - [`failure_reason(Option<String>)`](crate::operation::get_telemetry_evaluation_status::GetTelemetryEvaluationStatusOutput::failure_reason): <p>Describes the reason for the failure status. The field will only be populated if <code>Status</code> is <code>FAILED_START</code> or <code>FAILED_STOP</code>.</p>
    /// - On failure, responds with [`SdkError<GetTelemetryEvaluationStatusError>`](crate::operation::get_telemetry_evaluation_status::GetTelemetryEvaluationStatusError)
    pub fn get_telemetry_evaluation_status(
        &self,
    ) -> crate::operation::get_telemetry_evaluation_status::builders::GetTelemetryEvaluationStatusFluentBuilder {
        crate::operation::get_telemetry_evaluation_status::builders::GetTelemetryEvaluationStatusFluentBuilder::new(self.handle.clone())
    }
}
