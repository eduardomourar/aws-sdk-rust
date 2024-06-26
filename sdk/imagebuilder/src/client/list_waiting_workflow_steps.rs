// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListWaitingWorkflowSteps`](crate::operation::list_waiting_workflow_steps::builders::ListWaitingWorkflowStepsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_waiting_workflow_steps::builders::ListWaitingWorkflowStepsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::operation::list_waiting_workflow_steps::builders::ListWaitingWorkflowStepsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_waiting_workflow_steps::builders::ListWaitingWorkflowStepsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum items to return in a request.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_waiting_workflow_steps::builders::ListWaitingWorkflowStepsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_waiting_workflow_steps::builders::ListWaitingWorkflowStepsFluentBuilder::set_next_token):<br>required: **false**<br><p>A token to specify where to start paginating. This is the nextToken from a previously truncated response.</p><br>
    /// - On success, responds with [`ListWaitingWorkflowStepsOutput`](crate::operation::list_waiting_workflow_steps::ListWaitingWorkflowStepsOutput) with field(s):
    ///   - [`steps(Option<Vec::<WorkflowStepExecution>>)`](crate::operation::list_waiting_workflow_steps::ListWaitingWorkflowStepsOutput::steps): <p>An array of the workflow steps that are waiting for action in your Amazon Web Services account.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_waiting_workflow_steps::ListWaitingWorkflowStepsOutput::next_token): <p>The next token used for paginated responses. When this field isn't empty, there are additional elements that the service hasn't included in this request. Use this token with the next request to retrieve additional objects.</p>
    /// - On failure, responds with [`SdkError<ListWaitingWorkflowStepsError>`](crate::operation::list_waiting_workflow_steps::ListWaitingWorkflowStepsError)
    pub fn list_waiting_workflow_steps(&self) -> crate::operation::list_waiting_workflow_steps::builders::ListWaitingWorkflowStepsFluentBuilder {
        crate::operation::list_waiting_workflow_steps::builders::ListWaitingWorkflowStepsFluentBuilder::new(self.handle.clone())
    }
}
