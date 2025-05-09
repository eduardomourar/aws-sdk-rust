// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_snapshot::_create_snapshot_output::CreateSnapshotOutputBuilder;

pub use crate::operation::create_snapshot::_create_snapshot_input::CreateSnapshotInputBuilder;

impl crate::operation::create_snapshot::builders::CreateSnapshotInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_snapshot::CreateSnapshotOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_snapshot::CreateSnapshotError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_snapshot();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateSnapshot`.
///
/// <p>Creates a snapshot of all databases in a namespace. For more information about snapshots, see <a href="https://docs.aws.amazon.com/redshift/latest/mgmt/serverless-snapshots-recovery-points.html"> Working with snapshots and recovery points</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateSnapshotFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_snapshot::builders::CreateSnapshotInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_snapshot::CreateSnapshotOutput,
        crate::operation::create_snapshot::CreateSnapshotError,
    > for CreateSnapshotFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_snapshot::CreateSnapshotOutput,
            crate::operation::create_snapshot::CreateSnapshotError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateSnapshotFluentBuilder {
    /// Creates a new `CreateSnapshotFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateSnapshot as a reference.
    pub fn as_input(&self) -> &crate::operation::create_snapshot::builders::CreateSnapshotInputBuilder {
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
        crate::operation::create_snapshot::CreateSnapshotOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_snapshot::CreateSnapshotError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_snapshot::CreateSnapshot::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_snapshot::CreateSnapshot::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_snapshot::CreateSnapshotOutput,
        crate::operation::create_snapshot::CreateSnapshotError,
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
    /// <p>The namespace to create a snapshot for.</p>
    pub fn namespace_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.namespace_name(input.into());
        self
    }
    /// <p>The namespace to create a snapshot for.</p>
    pub fn set_namespace_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_namespace_name(input);
        self
    }
    /// <p>The namespace to create a snapshot for.</p>
    pub fn get_namespace_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_namespace_name()
    }
    /// <p>The name of the snapshot.</p>
    pub fn snapshot_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.snapshot_name(input.into());
        self
    }
    /// <p>The name of the snapshot.</p>
    pub fn set_snapshot_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_snapshot_name(input);
        self
    }
    /// <p>The name of the snapshot.</p>
    pub fn get_snapshot_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_snapshot_name()
    }
    /// <p>How long to retain the created snapshot.</p>
    pub fn retention_period(mut self, input: i32) -> Self {
        self.inner = self.inner.retention_period(input);
        self
    }
    /// <p>How long to retain the created snapshot.</p>
    pub fn set_retention_period(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_retention_period(input);
        self
    }
    /// <p>How long to retain the created snapshot.</p>
    pub fn get_retention_period(&self) -> &::std::option::Option<i32> {
        self.inner.get_retention_period()
    }
    ///
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>An array of <a href="https://docs.aws.amazon.com/redshift-serverless/latest/APIReference/API_Tag.html">Tag objects</a> to associate with the snapshot.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>An array of <a href="https://docs.aws.amazon.com/redshift-serverless/latest/APIReference/API_Tag.html">Tag objects</a> to associate with the snapshot.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>An array of <a href="https://docs.aws.amazon.com/redshift-serverless/latest/APIReference/API_Tag.html">Tag objects</a> to associate with the snapshot.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
}
