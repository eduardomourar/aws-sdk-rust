// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::enable_fast_snapshot_restores::_enable_fast_snapshot_restores_output::EnableFastSnapshotRestoresOutputBuilder;

pub use crate::operation::enable_fast_snapshot_restores::_enable_fast_snapshot_restores_input::EnableFastSnapshotRestoresInputBuilder;

impl crate::operation::enable_fast_snapshot_restores::builders::EnableFastSnapshotRestoresInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::enable_fast_snapshot_restores::EnableFastSnapshotRestoresOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::enable_fast_snapshot_restores::EnableFastSnapshotRestoresError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.enable_fast_snapshot_restores();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `EnableFastSnapshotRestores`.
///
/// <p>Enables fast snapshot restores for the specified snapshots in the specified Availability Zones.</p>
/// <p>You get the full benefit of fast snapshot restores after they enter the <code>enabled</code> state.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/ebs/latest/userguide/ebs-fast-snapshot-restore.html">Amazon EBS fast snapshot restore</a> in the <i>Amazon EBS User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct EnableFastSnapshotRestoresFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::enable_fast_snapshot_restores::builders::EnableFastSnapshotRestoresInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::enable_fast_snapshot_restores::EnableFastSnapshotRestoresOutput,
        crate::operation::enable_fast_snapshot_restores::EnableFastSnapshotRestoresError,
    > for EnableFastSnapshotRestoresFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::enable_fast_snapshot_restores::EnableFastSnapshotRestoresOutput,
            crate::operation::enable_fast_snapshot_restores::EnableFastSnapshotRestoresError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl EnableFastSnapshotRestoresFluentBuilder {
    /// Creates a new `EnableFastSnapshotRestoresFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the EnableFastSnapshotRestores as a reference.
    pub fn as_input(&self) -> &crate::operation::enable_fast_snapshot_restores::builders::EnableFastSnapshotRestoresInputBuilder {
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
        crate::operation::enable_fast_snapshot_restores::EnableFastSnapshotRestoresOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::enable_fast_snapshot_restores::EnableFastSnapshotRestoresError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::enable_fast_snapshot_restores::EnableFastSnapshotRestores::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::enable_fast_snapshot_restores::EnableFastSnapshotRestores::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::enable_fast_snapshot_restores::EnableFastSnapshotRestoresOutput,
        crate::operation::enable_fast_snapshot_restores::EnableFastSnapshotRestoresError,
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
    ///
    /// Appends an item to `AvailabilityZones`.
    ///
    /// To override the contents of this collection use [`set_availability_zones`](Self::set_availability_zones).
    ///
    /// <p>One or more Availability Zones. For example, <code>us-east-2a</code>.</p>
    pub fn availability_zones(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.availability_zones(input.into());
        self
    }
    /// <p>One or more Availability Zones. For example, <code>us-east-2a</code>.</p>
    pub fn set_availability_zones(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_availability_zones(input);
        self
    }
    /// <p>One or more Availability Zones. For example, <code>us-east-2a</code>.</p>
    pub fn get_availability_zones(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_availability_zones()
    }
    ///
    /// Appends an item to `SourceSnapshotIds`.
    ///
    /// To override the contents of this collection use [`set_source_snapshot_ids`](Self::set_source_snapshot_ids).
    ///
    /// <p>The IDs of one or more snapshots. For example, <code>snap-1234567890abcdef0</code>. You can specify a snapshot that was shared with you from another Amazon Web Services account.</p>
    pub fn source_snapshot_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.source_snapshot_ids(input.into());
        self
    }
    /// <p>The IDs of one or more snapshots. For example, <code>snap-1234567890abcdef0</code>. You can specify a snapshot that was shared with you from another Amazon Web Services account.</p>
    pub fn set_source_snapshot_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_source_snapshot_ids(input);
        self
    }
    /// <p>The IDs of one or more snapshots. For example, <code>snap-1234567890abcdef0</code>. You can specify a snapshot that was shared with you from another Amazon Web Services account.</p>
    pub fn get_source_snapshot_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_source_snapshot_ids()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
}
