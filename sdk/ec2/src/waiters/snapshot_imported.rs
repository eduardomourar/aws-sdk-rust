// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

///
/// Fluent builder for the `snapshot_imported` waiter.
///
/// This builder is intended to be used similar to the other fluent builders for
/// normal operations on the client. However, instead of a `send` method, it has
/// a `wait` method that takes a maximum amount of time to wait.
///
/// Construct this fluent builder using the client by importing the
/// [`Waiters`](crate::client::Waiters) trait and calling the methods
/// prefixed with `wait_until`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct SnapshotImportedFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_import_snapshot_tasks::builders::DescribeImportSnapshotTasksInputBuilder,
}
impl SnapshotImportedFluentBuilder {
    /// Creates a new `SnapshotImportedFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the DescribeImportSnapshotTasks as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_import_snapshot_tasks::builders::DescribeImportSnapshotTasksInputBuilder {
        &self.inner
    }
    /// Wait for `snapshot_imported`
    pub async fn wait(
        self,
        max_wait: ::std::time::Duration,
    ) -> ::std::result::Result<
        crate::waiters::snapshot_imported::SnapshotImportedFinalPoll,
        crate::waiters::snapshot_imported::WaitUntilSnapshotImportedError,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::waiters::error::WaiterError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_import_snapshot_tasks::DescribeImportSnapshotTasks::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            ::std::option::Option::None,
        )
        .with_operation_plugin(crate::sdk_feature_tracker::waiter::WaiterFeatureTrackerRuntimePlugin::new());
        let mut cfg = ::aws_smithy_types::config_bag::ConfigBag::base();
        let runtime_components_builder = runtime_plugins
            .apply_client_configuration(&mut cfg)
            .map_err(::aws_smithy_runtime_api::client::waiters::error::WaiterError::construction_failure)?;
        let time_components = runtime_components_builder.into_time_components();
        let sleep_impl = time_components.sleep_impl().expect("a sleep impl is required by waiters");
        let time_source = time_components.time_source().expect("a time source is required by waiters");

        let acceptor = move |result: ::std::result::Result<
            &crate::operation::describe_import_snapshot_tasks::DescribeImportSnapshotTasksOutput,
            &crate::operation::describe_import_snapshot_tasks::DescribeImportSnapshotTasksError,
        >| {
            // Matches: {"output":{"path":"ImportSnapshotTasks[].SnapshotTaskDetail.Status","expected":"completed","comparator":"allStringEquals"}}
            if crate::waiters::matchers::match_describe_import_snapshot_tasks_48f4f251081a266e0(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Success;
            }
            // Matches: {"output":{"path":"ImportSnapshotTasks[].SnapshotTaskDetail.Status","expected":"error","comparator":"anyStringEquals"}}
            if crate::waiters::matchers::match_describe_import_snapshot_tasks_6cd4957a19b889087(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Failure;
            }
            ::aws_smithy_runtime::client::waiters::AcceptorState::NoAcceptorsMatched
        };
        let operation = move || {
            let input = input.clone();
            let runtime_plugins = runtime_plugins.clone();
            async move { crate::operation::describe_import_snapshot_tasks::DescribeImportSnapshotTasks::orchestrate(&runtime_plugins, input).await }
        };
        let orchestrator = ::aws_smithy_runtime::client::waiters::WaiterOrchestrator::builder()
            .min_delay(::std::time::Duration::from_secs(15))
            .max_delay(::std::time::Duration::from_secs(120))
            .max_wait(max_wait)
            .time_source(time_source)
            .sleep_impl(sleep_impl)
            .acceptor(acceptor)
            .operation(operation)
            .build();
        ::aws_smithy_runtime::client::waiters::attach_waiter_tracing_span(orchestrator.orchestrate()).await
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
    ///
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>The filters.</p>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>The filters.</p>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>The filters.</p>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Filter>> {
        self.inner.get_filters()
    }
    ///
    /// Appends an item to `ImportTaskIds`.
    ///
    /// To override the contents of this collection use [`set_import_task_ids`](Self::set_import_task_ids).
    ///
    /// <p>A list of import snapshot task IDs.</p>
    pub fn import_task_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.import_task_ids(input.into());
        self
    }
    /// <p>A list of import snapshot task IDs.</p>
    pub fn set_import_task_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_import_task_ids(input);
        self
    }
    /// <p>A list of import snapshot task IDs.</p>
    pub fn get_import_task_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_import_task_ids()
    }
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>A token that indicates the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>A token that indicates the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>A token that indicates the next page of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}

/// Successful return type for the `snapshot_imported` waiter.
pub type SnapshotImportedFinalPoll = ::aws_smithy_runtime_api::client::waiters::FinalPoll<
    crate::operation::describe_import_snapshot_tasks::DescribeImportSnapshotTasksOutput,
    ::aws_smithy_runtime_api::client::result::SdkError<
        crate::operation::describe_import_snapshot_tasks::DescribeImportSnapshotTasksError,
        ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    >,
>;

/// Error type for the `snapshot_imported` waiter.
pub type WaitUntilSnapshotImportedError = ::aws_smithy_runtime_api::client::waiters::error::WaiterError<
    crate::operation::describe_import_snapshot_tasks::DescribeImportSnapshotTasksOutput,
    crate::operation::describe_import_snapshot_tasks::DescribeImportSnapshotTasksError,
>;
