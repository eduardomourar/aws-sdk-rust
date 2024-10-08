// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListGroups`](crate::operation::list_groups::builders::ListGroupsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_groups::builders::ListGroupsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`directory_id(impl Into<String>)`](crate::operation::list_groups::builders::ListGroupsFluentBuilder::directory_id) / [`set_directory_id(Option<String>)`](crate::operation::list_groups::builders::ListGroupsFluentBuilder::set_directory_id):<br>required: **true**<br><p>The identifier (ID) of the directory that's associated with the group.</p><br>
    ///   - [`realm(impl Into<String>)`](crate::operation::list_groups::builders::ListGroupsFluentBuilder::realm) / [`set_realm(Option<String>)`](crate::operation::list_groups::builders::ListGroupsFluentBuilder::set_realm):<br>required: **false**<br><p>The domain name associated with the directory.</p><note>  <p>This parameter is optional, so you can return groups outside of your Managed Microsoft AD domain. When no value is defined, only your Managed Microsoft AD groups are returned.</p>  <p>This value is case insensitive.</p> </note><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_groups::builders::ListGroupsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_groups::builders::ListGroupsFluentBuilder::set_next_token):<br>required: **false**<br><p>An encoded paging token for paginated calls that can be passed back to retrieve the next page.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_groups::builders::ListGroupsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_groups::builders::ListGroupsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to be returned per request.</p><br>
    /// - On success, responds with [`ListGroupsOutput`](crate::operation::list_groups::ListGroupsOutput) with field(s):
    ///   - [`directory_id(Option<String>)`](crate::operation::list_groups::ListGroupsOutput::directory_id): <p>The identifier (ID) of the directory that's associated with the group.</p>
    ///   - [`realm(Option<String>)`](crate::operation::list_groups::ListGroupsOutput::realm): <p>The domain name associated with the group.</p>
    ///   - [`groups(Option<Vec::<GroupSummary>>)`](crate::operation::list_groups::ListGroupsOutput::groups): <p>The group information that the request returns.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_groups::ListGroupsOutput::next_token): <p>An encoded paging token for paginated calls that can be passed back to retrieve the next page.</p>
    /// - On failure, responds with [`SdkError<ListGroupsError>`](crate::operation::list_groups::ListGroupsError)
    pub fn list_groups(&self) -> crate::operation::list_groups::builders::ListGroupsFluentBuilder {
        crate::operation::list_groups::builders::ListGroupsFluentBuilder::new(self.handle.clone())
    }
}
