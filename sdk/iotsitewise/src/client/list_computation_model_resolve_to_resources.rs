// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListComputationModelResolveToResources`](crate::operation::list_computation_model_resolve_to_resources::builders::ListComputationModelResolveToResourcesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_computation_model_resolve_to_resources::builders::ListComputationModelResolveToResourcesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`computation_model_id(impl Into<String>)`](crate::operation::list_computation_model_resolve_to_resources::builders::ListComputationModelResolveToResourcesFluentBuilder::computation_model_id) / [`set_computation_model_id(Option<String>)`](crate::operation::list_computation_model_resolve_to_resources::builders::ListComputationModelResolveToResourcesFluentBuilder::set_computation_model_id):<br>required: **true**<br><p>The ID of the computation model for which to list resolved resources.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_computation_model_resolve_to_resources::builders::ListComputationModelResolveToResourcesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_computation_model_resolve_to_resources::builders::ListComputationModelResolveToResourcesFluentBuilder::set_next_token):<br>required: **false**<br><p>The token used for the next set of paginated results.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_computation_model_resolve_to_resources::builders::ListComputationModelResolveToResourcesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_computation_model_resolve_to_resources::builders::ListComputationModelResolveToResourcesFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results returned for each paginated request.</p><br>
    /// - On success, responds with [`ListComputationModelResolveToResourcesOutput`](crate::operation::list_computation_model_resolve_to_resources::ListComputationModelResolveToResourcesOutput) with field(s):
    ///   - [`computation_model_resolve_to_resource_summaries(Vec::<ComputationModelResolveToResourceSummary>)`](crate::operation::list_computation_model_resolve_to_resources::ListComputationModelResolveToResourcesOutput::computation_model_resolve_to_resource_summaries): <p>A list of summaries describing the distinct resources that this computation model resolves to when actions were executed.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_computation_model_resolve_to_resources::ListComputationModelResolveToResourcesOutput::next_token): <p>The token for the next set of paginated results, or null if there are no additional results.</p>
    /// - On failure, responds with [`SdkError<ListComputationModelResolveToResourcesError>`](crate::operation::list_computation_model_resolve_to_resources::ListComputationModelResolveToResourcesError)
    pub fn list_computation_model_resolve_to_resources(
        &self,
    ) -> crate::operation::list_computation_model_resolve_to_resources::builders::ListComputationModelResolveToResourcesFluentBuilder {
        crate::operation::list_computation_model_resolve_to_resources::builders::ListComputationModelResolveToResourcesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
