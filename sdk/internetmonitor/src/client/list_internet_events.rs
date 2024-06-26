// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListInternetEvents`](crate::operation::list_internet_events::builders::ListInternetEventsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_internet_events::builders::ListInternetEventsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_internet_events::builders::ListInternetEventsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_internet_events::builders::ListInternetEventsFluentBuilder::set_next_token):<br>required: **false**<br><p>The token for the next set of results. You receive this token from a previous call.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_internet_events::builders::ListInternetEventsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_internet_events::builders::ListInternetEventsFluentBuilder::set_max_results):<br>required: **false**<br><p>The number of query results that you want to return with this call.</p><br>
    ///   - [`start_time(DateTime)`](crate::operation::list_internet_events::builders::ListInternetEventsFluentBuilder::start_time) / [`set_start_time(Option<DateTime>)`](crate::operation::list_internet_events::builders::ListInternetEventsFluentBuilder::set_start_time):<br>required: **false**<br><p>The start time of the time window that you want to get a list of internet events for.</p><br>
    ///   - [`end_time(DateTime)`](crate::operation::list_internet_events::builders::ListInternetEventsFluentBuilder::end_time) / [`set_end_time(Option<DateTime>)`](crate::operation::list_internet_events::builders::ListInternetEventsFluentBuilder::set_end_time):<br>required: **false**<br><p>The end time of the time window that you want to get a list of internet events for.</p><br>
    ///   - [`event_status(impl Into<String>)`](crate::operation::list_internet_events::builders::ListInternetEventsFluentBuilder::event_status) / [`set_event_status(Option<String>)`](crate::operation::list_internet_events::builders::ListInternetEventsFluentBuilder::set_event_status):<br>required: **false**<br><p>The status of an internet event.</p><br>
    ///   - [`event_type(impl Into<String>)`](crate::operation::list_internet_events::builders::ListInternetEventsFluentBuilder::event_type) / [`set_event_type(Option<String>)`](crate::operation::list_internet_events::builders::ListInternetEventsFluentBuilder::set_event_type):<br>required: **false**<br><p>The type of network impairment.</p><br>
    /// - On success, responds with [`ListInternetEventsOutput`](crate::operation::list_internet_events::ListInternetEventsOutput) with field(s):
    ///   - [`internet_events(Vec::<InternetEventSummary>)`](crate::operation::list_internet_events::ListInternetEventsOutput::internet_events): <p>A set of internet events returned for the list operation.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_internet_events::ListInternetEventsOutput::next_token): <p>The token for the next set of results. You receive this token from a previous call.</p>
    /// - On failure, responds with [`SdkError<ListInternetEventsError>`](crate::operation::list_internet_events::ListInternetEventsError)
    pub fn list_internet_events(&self) -> crate::operation::list_internet_events::builders::ListInternetEventsFluentBuilder {
        crate::operation::list_internet_events::builders::ListInternetEventsFluentBuilder::new(self.handle.clone())
    }
}
