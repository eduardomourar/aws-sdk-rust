// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListExecutions`](crate::operation::list_executions::builders::ListExecutionsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_executions::builders::ListExecutionsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`target_resource_type(TargetResourceType)`](crate::operation::list_executions::builders::ListExecutionsFluentBuilder::target_resource_type) / [`set_target_resource_type(Option<TargetResourceType>)`](crate::operation::list_executions::builders::ListExecutionsFluentBuilder::set_target_resource_type):<br>required: **true**<br><p>The type of the target resource.</p><br>
    ///   - [`target_resource_id(impl Into<String>)`](crate::operation::list_executions::builders::ListExecutionsFluentBuilder::target_resource_id) / [`set_target_resource_id(Option<String>)`](crate::operation::list_executions::builders::ListExecutionsFluentBuilder::set_target_resource_id):<br>required: **true**<br><p>The ID of the target resource.</p><br>
    ///   - [`resolve_to_resource_type(ResolveToResourceType)`](crate::operation::list_executions::builders::ListExecutionsFluentBuilder::resolve_to_resource_type) / [`set_resolve_to_resource_type(Option<ResolveToResourceType>)`](crate::operation::list_executions::builders::ListExecutionsFluentBuilder::set_resolve_to_resource_type):<br>required: **false**<br><p>The type of the resolved resource.</p><br>
    ///   - [`resolve_to_resource_id(impl Into<String>)`](crate::operation::list_executions::builders::ListExecutionsFluentBuilder::resolve_to_resource_id) / [`set_resolve_to_resource_id(Option<String>)`](crate::operation::list_executions::builders::ListExecutionsFluentBuilder::set_resolve_to_resource_id):<br>required: **false**<br><p>The ID of the resolved resource.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_executions::builders::ListExecutionsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_executions::builders::ListExecutionsFluentBuilder::set_next_token):<br>required: **false**<br><p>The token used for the next set of paginated results.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_executions::builders::ListExecutionsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_executions::builders::ListExecutionsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results returned for each paginated request.</p><br>
    ///   - [`action_type(impl Into<String>)`](crate::operation::list_executions::builders::ListExecutionsFluentBuilder::action_type) / [`set_action_type(Option<String>)`](crate::operation::list_executions::builders::ListExecutionsFluentBuilder::set_action_type):<br>required: **false**<br><p>The type of action exectued.</p><br>
    /// - On success, responds with [`ListExecutionsOutput`](crate::operation::list_executions::ListExecutionsOutput) with field(s):
    ///   - [`execution_summaries(Vec::<ExecutionSummary>)`](crate::operation::list_executions::ListExecutionsOutput::execution_summaries): <p>Contains the list of execution summaries of the computation models.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_executions::ListExecutionsOutput::next_token): <p>The token for the next set of results, or null if there are no additional results.</p>
    /// - On failure, responds with [`SdkError<ListExecutionsError>`](crate::operation::list_executions::ListExecutionsError)
    pub fn list_executions(&self) -> crate::operation::list_executions::builders::ListExecutionsFluentBuilder {
        crate::operation::list_executions::builders::ListExecutionsFluentBuilder::new(self.handle.clone())
    }
}
