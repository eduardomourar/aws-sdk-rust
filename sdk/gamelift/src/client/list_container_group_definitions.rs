// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListContainerGroupDefinitions`](crate::operation::list_container_group_definitions::builders::ListContainerGroupDefinitionsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_container_group_definitions::builders::ListContainerGroupDefinitionsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`container_group_type(ContainerGroupType)`](crate::operation::list_container_group_definitions::builders::ListContainerGroupDefinitionsFluentBuilder::container_group_type) / [`set_container_group_type(Option<ContainerGroupType>)`](crate::operation::list_container_group_definitions::builders::ListContainerGroupDefinitionsFluentBuilder::set_container_group_type):<br>required: **false**<br><p>The type of container group to retrieve. Container group type determines how Amazon GameLift Servers deploys the container group on each fleet instance.</p><br>
    ///   - [`limit(i32)`](crate::operation::list_container_group_definitions::builders::ListContainerGroupDefinitionsFluentBuilder::limit) / [`set_limit(Option<i32>)`](crate::operation::list_container_group_definitions::builders::ListContainerGroupDefinitionsFluentBuilder::set_limit):<br>required: **false**<br><p>The maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_container_group_definitions::builders::ListContainerGroupDefinitionsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_container_group_definitions::builders::ListContainerGroupDefinitionsFluentBuilder::set_next_token):<br>required: **false**<br><p>A token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this operation. To start at the beginning of the result set, do not specify a value.</p><br>
    /// - On success, responds with [`ListContainerGroupDefinitionsOutput`](crate::operation::list_container_group_definitions::ListContainerGroupDefinitionsOutput) with field(s):
    ///   - [`container_group_definitions(Option<Vec::<ContainerGroupDefinition>>)`](crate::operation::list_container_group_definitions::ListContainerGroupDefinitionsOutput::container_group_definitions): <p>A result set of container group definitions that match the request.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_container_group_definitions::ListContainerGroupDefinitionsOutput::next_token): <p>A token that indicates where to resume retrieving results on the next call to this operation. If no token is returned, these results represent the end of the list.</p>
    /// - On failure, responds with [`SdkError<ListContainerGroupDefinitionsError>`](crate::operation::list_container_group_definitions::ListContainerGroupDefinitionsError)
    pub fn list_container_group_definitions(
        &self,
    ) -> crate::operation::list_container_group_definitions::builders::ListContainerGroupDefinitionsFluentBuilder {
        crate::operation::list_container_group_definitions::builders::ListContainerGroupDefinitionsFluentBuilder::new(self.handle.clone())
    }
}
