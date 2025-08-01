// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListTelemetryRules`](crate::operation::list_telemetry_rules::builders::ListTelemetryRulesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_telemetry_rules::builders::ListTelemetryRulesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`rule_name_prefix(impl Into<String>)`](crate::operation::list_telemetry_rules::builders::ListTelemetryRulesFluentBuilder::rule_name_prefix) / [`set_rule_name_prefix(Option<String>)`](crate::operation::list_telemetry_rules::builders::ListTelemetryRulesFluentBuilder::set_rule_name_prefix):<br>required: **false**<br><p>A string to filter telemetry rules whose names begin with the specified prefix.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_telemetry_rules::builders::ListTelemetryRulesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_telemetry_rules::builders::ListTelemetryRulesFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of telemetry rules to return in a single call.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_telemetry_rules::builders::ListTelemetryRulesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_telemetry_rules::builders::ListTelemetryRulesFluentBuilder::set_next_token):<br>required: **false**<br><p>The token for the next set of results. A previous call generates this token.</p><br>
    /// - On success, responds with [`ListTelemetryRulesOutput`](crate::operation::list_telemetry_rules::ListTelemetryRulesOutput) with field(s):
    ///   - [`telemetry_rule_summaries(Option<Vec::<TelemetryRuleSummary>>)`](crate::operation::list_telemetry_rules::ListTelemetryRulesOutput::telemetry_rule_summaries): <p>A list of telemetry rule summaries.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_telemetry_rules::ListTelemetryRulesOutput::next_token): <p>A token to resume pagination of results.</p>
    /// - On failure, responds with [`SdkError<ListTelemetryRulesError>`](crate::operation::list_telemetry_rules::ListTelemetryRulesError)
    pub fn list_telemetry_rules(&self) -> crate::operation::list_telemetry_rules::builders::ListTelemetryRulesFluentBuilder {
        crate::operation::list_telemetry_rules::builders::ListTelemetryRulesFluentBuilder::new(self.handle.clone())
    }
}
