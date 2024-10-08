// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

///
/// Fluent builder for the `import_task_cancelled` waiter.
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
pub struct ImportTaskCancelledFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_import_task::builders::GetImportTaskInputBuilder,
}
impl ImportTaskCancelledFluentBuilder {
    /// Creates a new `ImportTaskCancelledFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the GetImportTask as a reference.
    pub fn as_input(&self) -> &crate::operation::get_import_task::builders::GetImportTaskInputBuilder {
        &self.inner
    }
    /// Wait until Import Task is Cancelled
    pub async fn wait(
        self,
        max_wait: ::std::time::Duration,
    ) -> ::std::result::Result<
        crate::waiters::import_task_cancelled::ImportTaskCancelledFinalPoll,
        crate::waiters::import_task_cancelled::WaitUntilImportTaskCancelledError,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::waiters::error::WaiterError::construction_failure)?;
        let runtime_plugins = crate::operation::get_import_task::GetImportTask::operation_runtime_plugins(
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
            &crate::operation::get_import_task::GetImportTaskOutput,
            &crate::operation::get_import_task::GetImportTaskError,
        >| {
            // Matches: {"output":{"path":"status != 'CANCELLING' && status != 'CANCELLED'","expected":"true","comparator":"booleanEquals"}}
            if crate::waiters::matchers::match_get_import_task_3f820f817a7d877c1(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Failure;
            }
            // Matches: {"output":{"path":"status","expected":"CANCELLED","comparator":"stringEquals"}}
            if crate::waiters::matchers::match_get_import_task_4ddbeb25cc5547dc0(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Success;
            }
            ::aws_smithy_runtime::client::waiters::AcceptorState::NoAcceptorsMatched
        };
        let operation = move || {
            let input = input.clone();
            let runtime_plugins = runtime_plugins.clone();
            async move { crate::operation::get_import_task::GetImportTask::orchestrate(&runtime_plugins, input).await }
        };
        let orchestrator = ::aws_smithy_runtime::client::waiters::WaiterOrchestrator::builder()
            .min_delay(::std::time::Duration::from_secs(60))
            .max_delay(::std::time::Duration::from_secs(3600))
            .max_wait(max_wait)
            .time_source(time_source)
            .sleep_impl(sleep_impl)
            .acceptor(acceptor)
            .operation(operation)
            .build();
        ::aws_smithy_runtime::client::waiters::attach_waiter_tracing_span(orchestrator.orchestrate()).await
    }
    /// <p>The unique identifier of the import task.</p>
    pub fn task_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.task_identifier(input.into());
        self
    }
    /// <p>The unique identifier of the import task.</p>
    pub fn set_task_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_task_identifier(input);
        self
    }
    /// <p>The unique identifier of the import task.</p>
    pub fn get_task_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_task_identifier()
    }
}

/// Successful return type for the `import_task_cancelled` waiter.
pub type ImportTaskCancelledFinalPoll = ::aws_smithy_runtime_api::client::waiters::FinalPoll<
    crate::operation::get_import_task::GetImportTaskOutput,
    ::aws_smithy_runtime_api::client::result::SdkError<
        crate::operation::get_import_task::GetImportTaskError,
        ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    >,
>;

/// Error type for the `import_task_cancelled` waiter.
pub type WaitUntilImportTaskCancelledError = ::aws_smithy_runtime_api::client::waiters::error::WaiterError<
    crate::operation::get_import_task::GetImportTaskOutput,
    crate::operation::get_import_task::GetImportTaskError,
>;
