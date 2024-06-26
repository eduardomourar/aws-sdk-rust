// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListConnectors`](crate::operation::list_connectors::builders::ListConnectorsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_connectors::builders::ListConnectorsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`filters(ListConnectorsRequestFilters)`](crate::operation::list_connectors::builders::ListConnectorsFluentBuilder::filters) / [`set_filters(Option<ListConnectorsRequestFilters>)`](crate::operation::list_connectors::builders::ListConnectorsFluentBuilder::set_filters):<br>required: **false**<br><p>List Connectors Request filters.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_connectors::builders::ListConnectorsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_connectors::builders::ListConnectorsFluentBuilder::set_max_results):<br>required: **false**<br><p>List Connectors Request max results.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_connectors::builders::ListConnectorsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_connectors::builders::ListConnectorsFluentBuilder::set_next_token):<br>required: **false**<br><p>List Connectors Request next token.</p><br>
    /// - On success, responds with [`ListConnectorsOutput`](crate::operation::list_connectors::ListConnectorsOutput) with field(s):
    ///   - [`items(Option<Vec::<Connector>>)`](crate::operation::list_connectors::ListConnectorsOutput::items): <p>List connectors response items.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_connectors::ListConnectorsOutput::next_token): <p>List connectors response next token.</p>
    /// - On failure, responds with [`SdkError<ListConnectorsError>`](crate::operation::list_connectors::ListConnectorsError)
    pub fn list_connectors(&self) -> crate::operation::list_connectors::builders::ListConnectorsFluentBuilder {
        crate::operation::list_connectors::builders::ListConnectorsFluentBuilder::new(self.handle.clone())
    }
}
