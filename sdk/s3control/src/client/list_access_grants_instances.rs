// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListAccessGrantsInstances`](crate::operation::list_access_grants_instances::builders::ListAccessGrantsInstancesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_access_grants_instances::builders::ListAccessGrantsInstancesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_id(impl Into<String>)`](crate::operation::list_access_grants_instances::builders::ListAccessGrantsInstancesFluentBuilder::account_id) / [`set_account_id(Option<String>)`](crate::operation::list_access_grants_instances::builders::ListAccessGrantsInstancesFluentBuilder::set_account_id):<br>required: **true**<br><p>The Amazon Web Services account ID of the S3 Access Grants instance.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_access_grants_instances::builders::ListAccessGrantsInstancesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_access_grants_instances::builders::ListAccessGrantsInstancesFluentBuilder::set_next_token):<br>required: **false**<br><p>A pagination token to request the next page of results. Pass this value into a subsequent <code>List Access Grants Instances</code> request in order to retrieve the next page of results.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_access_grants_instances::builders::ListAccessGrantsInstancesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_access_grants_instances::builders::ListAccessGrantsInstancesFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of access grants that you would like returned in the <code>List Access Grants</code> response. If the results include the pagination token <code>NextToken</code>, make another call using the <code>NextToken</code> to determine if there are more results.</p><br>
    /// - On success, responds with [`ListAccessGrantsInstancesOutput`](crate::operation::list_access_grants_instances::ListAccessGrantsInstancesOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::list_access_grants_instances::ListAccessGrantsInstancesOutput::next_token): <p>A pagination token to request the next page of results. Pass this value into a subsequent <code>List Access Grants Instances</code> request in order to retrieve the next page of results.</p>
    ///   - [`access_grants_instances_list(Option<Vec::<ListAccessGrantsInstanceEntry>>)`](crate::operation::list_access_grants_instances::ListAccessGrantsInstancesOutput::access_grants_instances_list): <p>A container for a list of S3 Access Grants instances.</p>
    /// - On failure, responds with [`SdkError<ListAccessGrantsInstancesError>`](crate::operation::list_access_grants_instances::ListAccessGrantsInstancesError)
    pub fn list_access_grants_instances(&self) -> crate::operation::list_access_grants_instances::builders::ListAccessGrantsInstancesFluentBuilder {
        crate::operation::list_access_grants_instances::builders::ListAccessGrantsInstancesFluentBuilder::new(self.handle.clone())
    }
}
