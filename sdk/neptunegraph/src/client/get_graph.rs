// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetGraph`](crate::operation::get_graph::builders::GetGraphFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`graph_identifier(impl Into<String>)`](crate::operation::get_graph::builders::GetGraphFluentBuilder::graph_identifier) / [`set_graph_identifier(Option<String>)`](crate::operation::get_graph::builders::GetGraphFluentBuilder::set_graph_identifier):<br>required: **true**<br><p>The unique identifier of the Neptune Analytics graph.</p><br>
    /// - On success, responds with [`GetGraphOutput`](crate::operation::get_graph::GetGraphOutput) with field(s):
    ///   - [`id(String)`](crate::operation::get_graph::GetGraphOutput::id): <p>The unique identifier of the graph.</p>
    ///   - [`name(String)`](crate::operation::get_graph::GetGraphOutput::name): <p>The name of the graph.</p>
    ///   - [`arn(String)`](crate::operation::get_graph::GetGraphOutput::arn): <p>The ARN associated with the graph.</p>
    ///   - [`status(Option<GraphStatus>)`](crate::operation::get_graph::GetGraphOutput::status): <p>The status of the graph.</p>
    ///   - [`status_reason(Option<String>)`](crate::operation::get_graph::GetGraphOutput::status_reason): <p>The reason that the graph has this status.</p>
    ///   - [`create_time(Option<DateTime>)`](crate::operation::get_graph::GetGraphOutput::create_time): <p>The time at which the graph was created.</p>
    ///   - [`provisioned_memory(Option<i32>)`](crate::operation::get_graph::GetGraphOutput::provisioned_memory): <p>The number of memory-optimized Neptune Capacity Units (m-NCUs) allocated to the graph.</p>
    ///   - [`endpoint(Option<String>)`](crate::operation::get_graph::GetGraphOutput::endpoint): <p>The graph endpoint.</p>
    ///   - [`public_connectivity(Option<bool>)`](crate::operation::get_graph::GetGraphOutput::public_connectivity): <p>If <code>true</code>, the graph has a public endpoint, otherwise not.</p>
    ///   - [`vector_search_configuration(Option<VectorSearchConfiguration>)`](crate::operation::get_graph::GetGraphOutput::vector_search_configuration): <p>Specifies the number of dimensions for vector embeddings loaded into the graph. Max = 65535</p>
    ///   - [`replica_count(Option<i32>)`](crate::operation::get_graph::GetGraphOutput::replica_count): <p>The number of replicas for the graph.</p>
    ///   - [`kms_key_identifier(Option<String>)`](crate::operation::get_graph::GetGraphOutput::kms_key_identifier): <p>The ID of the KMS key used to encrypt and decrypt graph data.</p>
    ///   - [`source_snapshot_id(Option<String>)`](crate::operation::get_graph::GetGraphOutput::source_snapshot_id): <p>The ID of the snapshot from which the graph was created, if it was created from a snapshot.</p>
    ///   - [`deletion_protection(Option<bool>)`](crate::operation::get_graph::GetGraphOutput::deletion_protection): <p>If <code>true</code>, deletion protection is enabled for the graph.</p>
    ///   - [`build_number(Option<String>)`](crate::operation::get_graph::GetGraphOutput::build_number): <p>The build number of the graph.</p>
    /// - On failure, responds with [`SdkError<GetGraphError>`](crate::operation::get_graph::GetGraphError)
    pub fn get_graph(&self) -> crate::operation::get_graph::builders::GetGraphFluentBuilder {
        crate::operation::get_graph::builders::GetGraphFluentBuilder::new(self.handle.clone())
    }
}
