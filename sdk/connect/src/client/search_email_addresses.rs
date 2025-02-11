// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`SearchEmailAddresses`](crate::operation::search_email_addresses::builders::SearchEmailAddressesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_id(impl Into<String>)`](crate::operation::search_email_addresses::builders::SearchEmailAddressesFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::search_email_addresses::builders::SearchEmailAddressesFluentBuilder::set_instance_id):<br>required: **true**<br><p>The identifier of the Amazon Connect instance. You can <a href="https://docs.aws.amazon.com/connect/latest/adminguide/find-instance-arn.html">find the instance ID</a> in the Amazon Resource Name (ARN) of the instance.</p><br>
    ///   - [`max_results(i32)`](crate::operation::search_email_addresses::builders::SearchEmailAddressesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::search_email_addresses::builders::SearchEmailAddressesFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return per page.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::search_email_addresses::builders::SearchEmailAddressesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::search_email_addresses::builders::SearchEmailAddressesFluentBuilder::set_next_token):<br>required: **false**<br><p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p><br>
    ///   - [`search_criteria(EmailAddressSearchCriteria)`](crate::operation::search_email_addresses::builders::SearchEmailAddressesFluentBuilder::search_criteria) / [`set_search_criteria(Option<EmailAddressSearchCriteria>)`](crate::operation::search_email_addresses::builders::SearchEmailAddressesFluentBuilder::set_search_criteria):<br>required: **false**<br><p>The search criteria to be used to return email addresses.</p><br>
    ///   - [`search_filter(EmailAddressSearchFilter)`](crate::operation::search_email_addresses::builders::SearchEmailAddressesFluentBuilder::search_filter) / [`set_search_filter(Option<EmailAddressSearchFilter>)`](crate::operation::search_email_addresses::builders::SearchEmailAddressesFluentBuilder::set_search_filter):<br>required: **false**<br><p>Filters to be applied to search results.</p><br>
    /// - On success, responds with [`SearchEmailAddressesOutput`](crate::operation::search_email_addresses::SearchEmailAddressesOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::search_email_addresses::SearchEmailAddressesOutput::next_token): <p>If there are additional results, this is the token for the next set of results.</p>
    ///   - [`email_addresses(Option<Vec::<EmailAddressMetadata>>)`](crate::operation::search_email_addresses::SearchEmailAddressesOutput::email_addresses): <p>List of email addresses matching SearchFilter and SearchCriteria</p>
    ///   - [`approximate_total_count(Option<i64>)`](crate::operation::search_email_addresses::SearchEmailAddressesOutput::approximate_total_count): <p>The total number of email addresses which matched your search query.</p>
    /// - On failure, responds with [`SdkError<SearchEmailAddressesError>`](crate::operation::search_email_addresses::SearchEmailAddressesError)
    pub fn search_email_addresses(&self) -> crate::operation::search_email_addresses::builders::SearchEmailAddressesFluentBuilder {
        crate::operation::search_email_addresses::builders::SearchEmailAddressesFluentBuilder::new(self.handle.clone())
    }
}
