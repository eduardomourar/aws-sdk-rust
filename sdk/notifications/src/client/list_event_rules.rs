// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListEventRules`](crate::operation::list_event_rules::builders::ListEventRulesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_event_rules::builders::ListEventRulesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`notification_configuration_arn(impl Into<String>)`](crate::operation::list_event_rules::builders::ListEventRulesFluentBuilder::notification_configuration_arn) / [`set_notification_configuration_arn(Option<String>)`](crate::operation::list_event_rules::builders::ListEventRulesFluentBuilder::set_notification_configuration_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the <code>NotificationConfiguration</code>.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_event_rules::builders::ListEventRulesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_event_rules::builders::ListEventRulesFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to be returned in this call. The default value is 20.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_event_rules::builders::ListEventRulesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_event_rules::builders::ListEventRulesFluentBuilder::set_next_token):<br>required: **false**<br><p>The start token for paginated calls. Retrieved from the response of a previous <code>ListEventRules</code> call. Next token uses Base64 encoding.</p><br>
    /// - On success, responds with [`ListEventRulesOutput`](crate::operation::list_event_rules::ListEventRulesOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::list_event_rules::ListEventRulesOutput::next_token): <p>A pagination token. If a non-null pagination token is returned in a result, pass its value in another request to retrieve more entries.</p>
    ///   - [`event_rules(Vec::<EventRuleStructure>)`](crate::operation::list_event_rules::ListEventRulesOutput::event_rules): <p>A list of <code>EventRules</code>.</p>
    /// - On failure, responds with [`SdkError<ListEventRulesError>`](crate::operation::list_event_rules::ListEventRulesError)
    pub fn list_event_rules(&self) -> crate::operation::list_event_rules::builders::ListEventRulesFluentBuilder {
        crate::operation::list_event_rules::builders::ListEventRulesFluentBuilder::new(self.handle.clone())
    }
}
