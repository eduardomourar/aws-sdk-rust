// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteAccessEntry`](crate::operation::delete_access_entry::builders::DeleteAccessEntryFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`cluster_name(impl Into<String>)`](crate::operation::delete_access_entry::builders::DeleteAccessEntryFluentBuilder::cluster_name) / [`set_cluster_name(Option<String>)`](crate::operation::delete_access_entry::builders::DeleteAccessEntryFluentBuilder::set_cluster_name):<br>required: **true**<br><p>The name of your cluster.</p><br>
    ///   - [`principal_arn(impl Into<String>)`](crate::operation::delete_access_entry::builders::DeleteAccessEntryFluentBuilder::principal_arn) / [`set_principal_arn(Option<String>)`](crate::operation::delete_access_entry::builders::DeleteAccessEntryFluentBuilder::set_principal_arn):<br>required: **true**<br><p>The ARN of the IAM principal for the <code>AccessEntry</code>.</p><br>
    /// - On success, responds with [`DeleteAccessEntryOutput`](crate::operation::delete_access_entry::DeleteAccessEntryOutput)
    /// - On failure, responds with [`SdkError<DeleteAccessEntryError>`](crate::operation::delete_access_entry::DeleteAccessEntryError)
    pub fn delete_access_entry(&self) -> crate::operation::delete_access_entry::builders::DeleteAccessEntryFluentBuilder {
        crate::operation::delete_access_entry::builders::DeleteAccessEntryFluentBuilder::new(self.handle.clone())
    }
}
