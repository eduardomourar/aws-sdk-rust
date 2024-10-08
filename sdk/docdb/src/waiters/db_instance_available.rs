// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

///
/// Fluent builder for the `db_instance_available` waiter.
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
pub struct DbInstanceAvailableFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_db_instances::builders::DescribeDbInstancesInputBuilder,
}
impl DbInstanceAvailableFluentBuilder {
    /// Creates a new `DbInstanceAvailableFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the DescribeDBInstances as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_db_instances::builders::DescribeDbInstancesInputBuilder {
        &self.inner
    }
    /// Wait for `db_instance_available`
    pub async fn wait(
        self,
        max_wait: ::std::time::Duration,
    ) -> ::std::result::Result<
        crate::waiters::db_instance_available::DbInstanceAvailableFinalPoll,
        crate::waiters::db_instance_available::WaitUntilDbInstanceAvailableError,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::waiters::error::WaiterError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_db_instances::DescribeDBInstances::operation_runtime_plugins(
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
            &crate::operation::describe_db_instances::DescribeDbInstancesOutput,
            &crate::operation::describe_db_instances::DescribeDBInstancesError,
        >| {
            // Matches: {"output":{"path":"DBInstances[].DBInstanceStatus","expected":"available","comparator":"allStringEquals"}}
            if crate::waiters::matchers::match_describe_db_instances_1a49e96ac4906b298(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Success;
            }
            // Matches: {"output":{"path":"DBInstances[].DBInstanceStatus","expected":"deleted","comparator":"anyStringEquals"}}
            if crate::waiters::matchers::match_describe_db_instances_77e0187886e602da9(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Failure;
            }
            // Matches: {"output":{"path":"DBInstances[].DBInstanceStatus","expected":"deleting","comparator":"anyStringEquals"}}
            if crate::waiters::matchers::match_describe_db_instances_5ec9c383b459d62a0(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Failure;
            }
            // Matches: {"output":{"path":"DBInstances[].DBInstanceStatus","expected":"failed","comparator":"anyStringEquals"}}
            if crate::waiters::matchers::match_describe_db_instances_545820a6bb186e52b(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Failure;
            }
            // Matches: {"output":{"path":"DBInstances[].DBInstanceStatus","expected":"incompatible-restore","comparator":"anyStringEquals"}}
            if crate::waiters::matchers::match_describe_db_instances_608daadbd71b65910(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Failure;
            }
            // Matches: {"output":{"path":"DBInstances[].DBInstanceStatus","expected":"incompatible-parameters","comparator":"anyStringEquals"}}
            if crate::waiters::matchers::match_describe_db_instances_fe252904aec70cd63(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Failure;
            }
            ::aws_smithy_runtime::client::waiters::AcceptorState::NoAcceptorsMatched
        };
        let operation = move || {
            let input = input.clone();
            let runtime_plugins = runtime_plugins.clone();
            async move { crate::operation::describe_db_instances::DescribeDBInstances::orchestrate(&runtime_plugins, input).await }
        };
        let orchestrator = ::aws_smithy_runtime::client::waiters::WaiterOrchestrator::builder()
            .min_delay(::std::time::Duration::from_secs(30))
            .max_delay(::std::time::Duration::from_secs(120))
            .max_wait(max_wait)
            .time_source(time_source)
            .sleep_impl(sleep_impl)
            .acceptor(acceptor)
            .operation(operation)
            .build();
        ::aws_smithy_runtime::client::waiters::attach_waiter_tracing_span(orchestrator.orchestrate()).await
    }
    /// <p>The user-provided instance identifier. If this parameter is specified, information from only the specific instance is returned. This parameter isn't case sensitive.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>If provided, must match the identifier of an existing <code>DBInstance</code>.</p></li>
    /// </ul>
    pub fn db_instance_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.db_instance_identifier(input.into());
        self
    }
    /// <p>The user-provided instance identifier. If this parameter is specified, information from only the specific instance is returned. This parameter isn't case sensitive.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>If provided, must match the identifier of an existing <code>DBInstance</code>.</p></li>
    /// </ul>
    pub fn set_db_instance_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_db_instance_identifier(input);
        self
    }
    /// <p>The user-provided instance identifier. If this parameter is specified, information from only the specific instance is returned. This parameter isn't case sensitive.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>If provided, must match the identifier of an existing <code>DBInstance</code>.</p></li>
    /// </ul>
    pub fn get_db_instance_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_db_instance_identifier()
    }
    ///
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>A filter that specifies one or more instances to describe.</p>
    /// <p>Supported filters:</p>
    /// <ul>
    /// <li>
    /// <p><code>db-cluster-id</code> - Accepts cluster identifiers and cluster Amazon Resource Names (ARNs). The results list includes only the information about the instances that are associated with the clusters that are identified by these ARNs.</p></li>
    /// <li>
    /// <p><code>db-instance-id</code> - Accepts instance identifiers and instance ARNs. The results list includes only the information about the instances that are identified by these ARNs.</p></li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>A filter that specifies one or more instances to describe.</p>
    /// <p>Supported filters:</p>
    /// <ul>
    /// <li>
    /// <p><code>db-cluster-id</code> - Accepts cluster identifiers and cluster Amazon Resource Names (ARNs). The results list includes only the information about the instances that are associated with the clusters that are identified by these ARNs.</p></li>
    /// <li>
    /// <p><code>db-instance-id</code> - Accepts instance identifiers and instance ARNs. The results list includes only the information about the instances that are identified by these ARNs.</p></li>
    /// </ul>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>A filter that specifies one or more instances to describe.</p>
    /// <p>Supported filters:</p>
    /// <ul>
    /// <li>
    /// <p><code>db-cluster-id</code> - Accepts cluster identifiers and cluster Amazon Resource Names (ARNs). The results list includes only the information about the instances that are associated with the clusters that are identified by these ARNs.</p></li>
    /// <li>
    /// <p><code>db-instance-id</code> - Accepts instance identifiers and instance ARNs. The results list includes only the information about the instances that are identified by these ARNs.</p></li>
    /// </ul>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Filter>> {
        self.inner.get_filters()
    }
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token (marker) is included in the response so that the remaining results can be retrieved.</p>
    /// <p>Default: 100</p>
    /// <p>Constraints: Minimum 20, maximum 100.</p>
    pub fn max_records(mut self, input: i32) -> Self {
        self.inner = self.inner.max_records(input);
        self
    }
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token (marker) is included in the response so that the remaining results can be retrieved.</p>
    /// <p>Default: 100</p>
    /// <p>Constraints: Minimum 20, maximum 100.</p>
    pub fn set_max_records(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_records(input);
        self
    }
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token (marker) is included in the response so that the remaining results can be retrieved.</p>
    /// <p>Default: 100</p>
    /// <p>Constraints: Minimum 20, maximum 100.</p>
    pub fn get_max_records(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_records()
    }
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub fn marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.marker(input.into());
        self
    }
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub fn set_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_marker(input);
        self
    }
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub fn get_marker(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_marker()
    }
}

/// Successful return type for the `db_instance_available` waiter.
pub type DbInstanceAvailableFinalPoll = ::aws_smithy_runtime_api::client::waiters::FinalPoll<
    crate::operation::describe_db_instances::DescribeDbInstancesOutput,
    ::aws_smithy_runtime_api::client::result::SdkError<
        crate::operation::describe_db_instances::DescribeDBInstancesError,
        ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    >,
>;

/// Error type for the `db_instance_available` waiter.
pub type WaitUntilDbInstanceAvailableError = ::aws_smithy_runtime_api::client::waiters::error::WaiterError<
    crate::operation::describe_db_instances::DescribeDbInstancesOutput,
    crate::operation::describe_db_instances::DescribeDBInstancesError,
>;
