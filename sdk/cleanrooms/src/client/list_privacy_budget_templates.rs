// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListPrivacyBudgetTemplates`](crate::operation::list_privacy_budget_templates::builders::ListPrivacyBudgetTemplatesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_privacy_budget_templates::builders::ListPrivacyBudgetTemplatesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`membership_identifier(impl Into<String>)`](crate::operation::list_privacy_budget_templates::builders::ListPrivacyBudgetTemplatesFluentBuilder::membership_identifier) / [`set_membership_identifier(Option<String>)`](crate::operation::list_privacy_budget_templates::builders::ListPrivacyBudgetTemplatesFluentBuilder::set_membership_identifier):<br>required: **true**<br><p>A unique identifier for one of your memberships for a collaboration. The privacy budget templates are retrieved from the collaboration that this membership belongs to. Accepts a membership ID.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_privacy_budget_templates::builders::ListPrivacyBudgetTemplatesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_privacy_budget_templates::builders::ListPrivacyBudgetTemplatesFluentBuilder::set_next_token):<br>required: **false**<br><p>The pagination token that's used to fetch the next set of results.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_privacy_budget_templates::builders::ListPrivacyBudgetTemplatesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_privacy_budget_templates::builders::ListPrivacyBudgetTemplatesFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results that are returned for an API request call. The service chooses a default number if you don't set one. The service might return a `nextToken` even if the `maxResults` value has not been met.</p><br>
    /// - On success, responds with [`ListPrivacyBudgetTemplatesOutput`](crate::operation::list_privacy_budget_templates::ListPrivacyBudgetTemplatesOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::list_privacy_budget_templates::ListPrivacyBudgetTemplatesOutput::next_token): <p>The pagination token that's used to fetch the next set of results.</p>
    ///   - [`privacy_budget_template_summaries(Vec::<PrivacyBudgetTemplateSummary>)`](crate::operation::list_privacy_budget_templates::ListPrivacyBudgetTemplatesOutput::privacy_budget_template_summaries): <p>An array that summarizes the privacy budget templates. The summary includes collaboration information, creation information, and privacy budget type.</p>
    /// - On failure, responds with [`SdkError<ListPrivacyBudgetTemplatesError>`](crate::operation::list_privacy_budget_templates::ListPrivacyBudgetTemplatesError)
    pub fn list_privacy_budget_templates(
        &self,
    ) -> crate::operation::list_privacy_budget_templates::builders::ListPrivacyBudgetTemplatesFluentBuilder {
        crate::operation::list_privacy_budget_templates::builders::ListPrivacyBudgetTemplatesFluentBuilder::new(self.handle.clone())
    }
}
