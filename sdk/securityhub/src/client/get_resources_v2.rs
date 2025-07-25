// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetResourcesV2`](crate::operation::get_resources_v2::builders::GetResourcesV2FluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::get_resources_v2::builders::GetResourcesV2FluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`filters(ResourcesFilters)`](crate::operation::get_resources_v2::builders::GetResourcesV2FluentBuilder::filters) / [`set_filters(Option<ResourcesFilters>)`](crate::operation::get_resources_v2::builders::GetResourcesV2FluentBuilder::set_filters):<br>required: **false**<br><p>Filters resources based on a set of criteria.</p><br>
    ///   - [`sort_criteria(SortCriterion)`](crate::operation::get_resources_v2::builders::GetResourcesV2FluentBuilder::sort_criteria) / [`set_sort_criteria(Option<Vec::<SortCriterion>>)`](crate::operation::get_resources_v2::builders::GetResourcesV2FluentBuilder::set_sort_criteria):<br>required: **false**<br><p>The finding attributes used to sort the list of returned findings.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::get_resources_v2::builders::GetResourcesV2FluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::get_resources_v2::builders::GetResourcesV2FluentBuilder::set_next_token):<br>required: **false**<br><p>The token required for pagination. On your first call, set the value of this parameter to <code>NULL</code>. For subsequent calls, to continue listing data, set the value of this parameter to the value returned in the previous response.</p><br>
    ///   - [`max_results(i32)`](crate::operation::get_resources_v2::builders::GetResourcesV2FluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::get_resources_v2::builders::GetResourcesV2FluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return.</p><br>
    /// - On success, responds with [`GetResourcesV2Output`](crate::operation::get_resources_v2::GetResourcesV2Output) with field(s):
    ///   - [`resources(Option<Vec::<ResourceResult>>)`](crate::operation::get_resources_v2::GetResourcesV2Output::resources): <p>Filters resources based on a set of criteria.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::get_resources_v2::GetResourcesV2Output::next_token): <p>The pagination token to use to request the next page of results. Otherwise, this parameter is null.</p>
    /// - On failure, responds with [`SdkError<GetResourcesV2Error>`](crate::operation::get_resources_v2::GetResourcesV2Error)
    pub fn get_resources_v2(&self) -> crate::operation::get_resources_v2::builders::GetResourcesV2FluentBuilder {
        crate::operation::get_resources_v2::builders::GetResourcesV2FluentBuilder::new(self.handle.clone())
    }
}
