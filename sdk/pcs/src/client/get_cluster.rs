// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetCluster`](crate::operation::get_cluster::builders::GetClusterFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`cluster_identifier(impl Into<String>)`](crate::operation::get_cluster::builders::GetClusterFluentBuilder::cluster_identifier) / [`set_cluster_identifier(Option<String>)`](crate::operation::get_cluster::builders::GetClusterFluentBuilder::set_cluster_identifier):<br>required: **true**<br><p>The name or ID of the cluster of the queue.</p><br>
    /// - On success, responds with [`GetClusterOutput`](crate::operation::get_cluster::GetClusterOutput) with field(s):
    ///   - [`cluster(Option<Cluster>)`](crate::operation::get_cluster::GetClusterOutput::cluster): <p>The cluster resource.</p>
    /// - On failure, responds with [`SdkError<GetClusterError>`](crate::operation::get_cluster::GetClusterError)
    pub fn get_cluster(&self) -> crate::operation::get_cluster::builders::GetClusterFluentBuilder {
        crate::operation::get_cluster::builders::GetClusterFluentBuilder::new(self.handle.clone())
    }
}
