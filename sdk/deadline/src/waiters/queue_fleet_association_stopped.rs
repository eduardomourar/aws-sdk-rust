// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

///
/// Fluent builder for the `queue_fleet_association_stopped` waiter.
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
pub struct QueueFleetAssociationStoppedFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_queue_fleet_association::builders::GetQueueFleetAssociationInputBuilder,
}
impl QueueFleetAssociationStoppedFluentBuilder {
    /// Creates a new `QueueFleetAssociationStoppedFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the GetQueueFleetAssociation as a reference.
    pub fn as_input(&self) -> &crate::operation::get_queue_fleet_association::builders::GetQueueFleetAssociationInputBuilder {
        &self.inner
    }
    /// Wait until a QueueFleetAssociation is stopped. Use this after setting the status to STOP_SCHEDULING_AND_COMPLETE_TASKS or STOP_SCHEDULING_AND_CANCEL_TASKS to wait for a QueueFleetAssociation to reach STOPPED
    pub async fn wait(
        self,
        max_wait: ::std::time::Duration,
    ) -> ::std::result::Result<
        crate::waiters::queue_fleet_association_stopped::QueueFleetAssociationStoppedFinalPoll,
        crate::waiters::queue_fleet_association_stopped::WaitUntilQueueFleetAssociationStoppedError,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::waiters::error::WaiterError::construction_failure)?;
        let runtime_plugins = crate::operation::get_queue_fleet_association::GetQueueFleetAssociation::operation_runtime_plugins(
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
            &crate::operation::get_queue_fleet_association::GetQueueFleetAssociationOutput,
            &crate::operation::get_queue_fleet_association::GetQueueFleetAssociationError,
        >| {
            // Matches: {"output":{"path":"status","expected":"STOPPED","comparator":"stringEquals"}}
            if crate::waiters::matchers::match_get_queue_fleet_association_752e7f99f3bc7dc47(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Success;
            }
            ::aws_smithy_runtime::client::waiters::AcceptorState::NoAcceptorsMatched
        };
        let operation = move || {
            let input = input.clone();
            let runtime_plugins = runtime_plugins.clone();
            async move { crate::operation::get_queue_fleet_association::GetQueueFleetAssociation::orchestrate(&runtime_plugins, input).await }
        };
        let orchestrator = ::aws_smithy_runtime::client::waiters::WaiterOrchestrator::builder()
            .min_delay(::std::time::Duration::from_secs(10))
            .max_delay(::std::time::Duration::from_secs(600))
            .max_wait(max_wait)
            .time_source(time_source)
            .sleep_impl(sleep_impl)
            .acceptor(acceptor)
            .operation(operation)
            .build();
        ::aws_smithy_runtime::client::waiters::attach_waiter_tracing_span(orchestrator.orchestrate()).await
    }
    /// <p>The farm ID of the farm that contains the queue-fleet association.</p>
    pub fn farm_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.farm_id(input.into());
        self
    }
    /// <p>The farm ID of the farm that contains the queue-fleet association.</p>
    pub fn set_farm_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_farm_id(input);
        self
    }
    /// <p>The farm ID of the farm that contains the queue-fleet association.</p>
    pub fn get_farm_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_farm_id()
    }
    /// <p>The queue ID for the queue-fleet association.</p>
    pub fn queue_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.queue_id(input.into());
        self
    }
    /// <p>The queue ID for the queue-fleet association.</p>
    pub fn set_queue_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_queue_id(input);
        self
    }
    /// <p>The queue ID for the queue-fleet association.</p>
    pub fn get_queue_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_queue_id()
    }
    /// <p>The fleet ID for the queue-fleet association.</p>
    pub fn fleet_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.fleet_id(input.into());
        self
    }
    /// <p>The fleet ID for the queue-fleet association.</p>
    pub fn set_fleet_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_fleet_id(input);
        self
    }
    /// <p>The fleet ID for the queue-fleet association.</p>
    pub fn get_fleet_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_fleet_id()
    }
}

/// Successful return type for the `queue_fleet_association_stopped` waiter.
pub type QueueFleetAssociationStoppedFinalPoll = ::aws_smithy_runtime_api::client::waiters::FinalPoll<
    crate::operation::get_queue_fleet_association::GetQueueFleetAssociationOutput,
    ::aws_smithy_runtime_api::client::result::SdkError<
        crate::operation::get_queue_fleet_association::GetQueueFleetAssociationError,
        ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    >,
>;

/// Error type for the `queue_fleet_association_stopped` waiter.
pub type WaitUntilQueueFleetAssociationStoppedError = ::aws_smithy_runtime_api::client::waiters::error::WaiterError<
    crate::operation::get_queue_fleet_association::GetQueueFleetAssociationOutput,
    crate::operation::get_queue_fleet_association::GetQueueFleetAssociationError,
>;
