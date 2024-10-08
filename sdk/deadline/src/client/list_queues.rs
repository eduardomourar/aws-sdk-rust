// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListQueues`](crate::operation::list_queues::builders::ListQueuesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_queues::builders::ListQueuesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`farm_id(impl Into<String>)`](crate::operation::list_queues::builders::ListQueuesFluentBuilder::farm_id) / [`set_farm_id(Option<String>)`](crate::operation::list_queues::builders::ListQueuesFluentBuilder::set_farm_id):<br>required: **true**<br><p>The farm ID of the queue.</p><br>
    ///   - [`principal_id(impl Into<String>)`](crate::operation::list_queues::builders::ListQueuesFluentBuilder::principal_id) / [`set_principal_id(Option<String>)`](crate::operation::list_queues::builders::ListQueuesFluentBuilder::set_principal_id):<br>required: **false**<br><p>The principal IDs to include in the list of queues.</p><br>
    ///   - [`status(QueueStatus)`](crate::operation::list_queues::builders::ListQueuesFluentBuilder::status) / [`set_status(Option<QueueStatus>)`](crate::operation::list_queues::builders::ListQueuesFluentBuilder::set_status):<br>required: **false**<br><p>The status of the queues listed.</p> <ul>  <li>   <p><code>ACTIVE</code>–The queues are active.</p></li>  <li>   <p><code>SCHEDULING</code>–The queues are scheduling.</p></li>  <li>   <p><code>SCHEDULING_BLOCKED</code>–The queue scheduling is blocked for these queues.</p></li> </ul><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_queues::builders::ListQueuesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_queues::builders::ListQueuesFluentBuilder::set_next_token):<br>required: **false**<br><p>The token for the next set of results, or <code>null</code> to start from the beginning.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_queues::builders::ListQueuesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_queues::builders::ListQueuesFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages.</p><br>
    /// - On success, responds with [`ListQueuesOutput`](crate::operation::list_queues::ListQueuesOutput) with field(s):
    ///   - [`queues(Vec::<QueueSummary>)`](crate::operation::list_queues::ListQueuesOutput::queues): <p>The queues on the list.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_queues::ListQueuesOutput::next_token): <p>If Deadline Cloud returns <code>nextToken</code>, then there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. To retrieve the next page, call the operation again using the returned token. Keep all other arguments unchanged. If no results remain, then <code>nextToken</code> is set to <code>null</code>. Each pagination token expires after 24 hours. If you provide a token that isn't valid, then you receive an HTTP 400 <code>ValidationException</code> error.</p>
    /// - On failure, responds with [`SdkError<ListQueuesError>`](crate::operation::list_queues::ListQueuesError)
    pub fn list_queues(&self) -> crate::operation::list_queues::builders::ListQueuesFluentBuilder {
        crate::operation::list_queues::builders::ListQueuesFluentBuilder::new(self.handle.clone())
    }
}
