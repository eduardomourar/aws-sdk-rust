// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListDataLakeNamespaces`](crate::operation::list_data_lake_namespaces::builders::ListDataLakeNamespacesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_data_lake_namespaces::builders::ListDataLakeNamespacesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_id(impl Into<String>)`](crate::operation::list_data_lake_namespaces::builders::ListDataLakeNamespacesFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::list_data_lake_namespaces::builders::ListDataLakeNamespacesFluentBuilder::set_instance_id):<br>required: **true**<br><p>The Amazon Web Services Supply Chain instance identifier.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_data_lake_namespaces::builders::ListDataLakeNamespacesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_data_lake_namespaces::builders::ListDataLakeNamespacesFluentBuilder::set_next_token):<br>required: **false**<br><p>The pagination token to fetch next page of namespaces.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_data_lake_namespaces::builders::ListDataLakeNamespacesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_data_lake_namespaces::builders::ListDataLakeNamespacesFluentBuilder::set_max_results):<br>required: **false**<br><p>The max number of namespaces to fetch in this paginated request.</p><br>
    /// - On success, responds with [`ListDataLakeNamespacesOutput`](crate::operation::list_data_lake_namespaces::ListDataLakeNamespacesOutput) with field(s):
    ///   - [`namespaces(Vec::<DataLakeNamespace>)`](crate::operation::list_data_lake_namespaces::ListDataLakeNamespacesOutput::namespaces): <p>The list of fetched namespace details. Noted it only contains custom namespaces, pre-defined namespaces are not included.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_data_lake_namespaces::ListDataLakeNamespacesOutput::next_token): <p>The pagination token to fetch next page of namespaces.</p>
    /// - On failure, responds with [`SdkError<ListDataLakeNamespacesError>`](crate::operation::list_data_lake_namespaces::ListDataLakeNamespacesError)
    pub fn list_data_lake_namespaces(&self) -> crate::operation::list_data_lake_namespaces::builders::ListDataLakeNamespacesFluentBuilder {
        crate::operation::list_data_lake_namespaces::builders::ListDataLakeNamespacesFluentBuilder::new(self.handle.clone())
    }
}
