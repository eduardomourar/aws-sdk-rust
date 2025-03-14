// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::restore_graph_from_snapshot::_restore_graph_from_snapshot_output::RestoreGraphFromSnapshotOutputBuilder;

pub use crate::operation::restore_graph_from_snapshot::_restore_graph_from_snapshot_input::RestoreGraphFromSnapshotInputBuilder;

impl crate::operation::restore_graph_from_snapshot::builders::RestoreGraphFromSnapshotInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::restore_graph_from_snapshot::RestoreGraphFromSnapshotOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::restore_graph_from_snapshot::RestoreGraphFromSnapshotError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.restore_graph_from_snapshot();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `RestoreGraphFromSnapshot`.
///
/// <p>Restores a graph from a snapshot.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RestoreGraphFromSnapshotFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::restore_graph_from_snapshot::builders::RestoreGraphFromSnapshotInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::restore_graph_from_snapshot::RestoreGraphFromSnapshotOutput,
        crate::operation::restore_graph_from_snapshot::RestoreGraphFromSnapshotError,
    > for RestoreGraphFromSnapshotFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::restore_graph_from_snapshot::RestoreGraphFromSnapshotOutput,
            crate::operation::restore_graph_from_snapshot::RestoreGraphFromSnapshotError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl RestoreGraphFromSnapshotFluentBuilder {
    /// Creates a new `RestoreGraphFromSnapshotFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the RestoreGraphFromSnapshot as a reference.
    pub fn as_input(&self) -> &crate::operation::restore_graph_from_snapshot::builders::RestoreGraphFromSnapshotInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::restore_graph_from_snapshot::RestoreGraphFromSnapshotOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::restore_graph_from_snapshot::RestoreGraphFromSnapshotError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::restore_graph_from_snapshot::RestoreGraphFromSnapshot::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::restore_graph_from_snapshot::RestoreGraphFromSnapshot::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::restore_graph_from_snapshot::RestoreGraphFromSnapshotOutput,
        crate::operation::restore_graph_from_snapshot::RestoreGraphFromSnapshotError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
        self.set_config_override(::std::option::Option::Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The ID of the snapshot in question.</p>
    pub fn snapshot_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.snapshot_identifier(input.into());
        self
    }
    /// <p>The ID of the snapshot in question.</p>
    pub fn set_snapshot_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_snapshot_identifier(input);
        self
    }
    /// <p>The ID of the snapshot in question.</p>
    pub fn get_snapshot_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_snapshot_identifier()
    }
    /// <p>A name for the new Neptune Analytics graph to be created from the snapshot.</p>
    /// <p>The name must contain from 1 to 63 letters, numbers, or hyphens, and its first character must be a letter. It cannot end with a hyphen or contain two consecutive hyphens. Only lowercase letters are allowed.</p>
    pub fn graph_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.graph_name(input.into());
        self
    }
    /// <p>A name for the new Neptune Analytics graph to be created from the snapshot.</p>
    /// <p>The name must contain from 1 to 63 letters, numbers, or hyphens, and its first character must be a letter. It cannot end with a hyphen or contain two consecutive hyphens. Only lowercase letters are allowed.</p>
    pub fn set_graph_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_graph_name(input);
        self
    }
    /// <p>A name for the new Neptune Analytics graph to be created from the snapshot.</p>
    /// <p>The name must contain from 1 to 63 letters, numbers, or hyphens, and its first character must be a letter. It cannot end with a hyphen or contain two consecutive hyphens. Only lowercase letters are allowed.</p>
    pub fn get_graph_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_graph_name()
    }
    /// <p>The provisioned memory-optimized Neptune Capacity Units (m-NCUs) to use for the graph.</p>
    /// <p>Min = 16</p>
    pub fn provisioned_memory(mut self, input: i32) -> Self {
        self.inner = self.inner.provisioned_memory(input);
        self
    }
    /// <p>The provisioned memory-optimized Neptune Capacity Units (m-NCUs) to use for the graph.</p>
    /// <p>Min = 16</p>
    pub fn set_provisioned_memory(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_provisioned_memory(input);
        self
    }
    /// <p>The provisioned memory-optimized Neptune Capacity Units (m-NCUs) to use for the graph.</p>
    /// <p>Min = 16</p>
    pub fn get_provisioned_memory(&self) -> &::std::option::Option<i32> {
        self.inner.get_provisioned_memory()
    }
    /// <p>A value that indicates whether the graph has deletion protection enabled. The graph can't be deleted when deletion protection is enabled.</p>
    pub fn deletion_protection(mut self, input: bool) -> Self {
        self.inner = self.inner.deletion_protection(input);
        self
    }
    /// <p>A value that indicates whether the graph has deletion protection enabled. The graph can't be deleted when deletion protection is enabled.</p>
    pub fn set_deletion_protection(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_deletion_protection(input);
        self
    }
    /// <p>A value that indicates whether the graph has deletion protection enabled. The graph can't be deleted when deletion protection is enabled.</p>
    pub fn get_deletion_protection(&self) -> &::std::option::Option<bool> {
        self.inner.get_deletion_protection()
    }
    ///
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Adds metadata tags to the snapshot. These tags can also be used with cost allocation reporting, or used in a Condition statement in an IAM policy.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>Adds metadata tags to the snapshot. These tags can also be used with cost allocation reporting, or used in a Condition statement in an IAM policy.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>Adds metadata tags to the snapshot. These tags can also be used with cost allocation reporting, or used in a Condition statement in an IAM policy.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_tags()
    }
    /// <p>The number of replicas in other AZs. Min =0, Max = 2, Default =1</p><important>
    /// <p>Additional charges equivalent to the m-NCUs selected for the graph apply for each replica.</p>
    /// </important>
    pub fn replica_count(mut self, input: i32) -> Self {
        self.inner = self.inner.replica_count(input);
        self
    }
    /// <p>The number of replicas in other AZs. Min =0, Max = 2, Default =1</p><important>
    /// <p>Additional charges equivalent to the m-NCUs selected for the graph apply for each replica.</p>
    /// </important>
    pub fn set_replica_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_replica_count(input);
        self
    }
    /// <p>The number of replicas in other AZs. Min =0, Max = 2, Default =1</p><important>
    /// <p>Additional charges equivalent to the m-NCUs selected for the graph apply for each replica.</p>
    /// </important>
    pub fn get_replica_count(&self) -> &::std::option::Option<i32> {
        self.inner.get_replica_count()
    }
    /// <p>Specifies whether or not the graph can be reachable over the internet. All access to graphs is IAM authenticated. (<code>true</code> to enable, or <code>false</code> to disable).</p>
    pub fn public_connectivity(mut self, input: bool) -> Self {
        self.inner = self.inner.public_connectivity(input);
        self
    }
    /// <p>Specifies whether or not the graph can be reachable over the internet. All access to graphs is IAM authenticated. (<code>true</code> to enable, or <code>false</code> to disable).</p>
    pub fn set_public_connectivity(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_public_connectivity(input);
        self
    }
    /// <p>Specifies whether or not the graph can be reachable over the internet. All access to graphs is IAM authenticated. (<code>true</code> to enable, or <code>false</code> to disable).</p>
    pub fn get_public_connectivity(&self) -> &::std::option::Option<bool> {
        self.inner.get_public_connectivity()
    }
}
