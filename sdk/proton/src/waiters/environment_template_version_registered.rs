// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

///
/// Fluent builder for the `environment_template_version_registered` waiter.
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
pub struct EnvironmentTemplateVersionRegisteredFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_environment_template_version::builders::GetEnvironmentTemplateVersionInputBuilder,
}
impl EnvironmentTemplateVersionRegisteredFluentBuilder {
    /// Creates a new `EnvironmentTemplateVersionRegisteredFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the GetEnvironmentTemplateVersion as a reference.
    pub fn as_input(&self) -> &crate::operation::get_environment_template_version::builders::GetEnvironmentTemplateVersionInputBuilder {
        &self.inner
    }
    /// Wait until an EnvironmentTemplateVersion is registered. Use this after invoking CreateEnvironmentTemplateVersion
    pub async fn wait(
        self,
        max_wait: ::std::time::Duration,
    ) -> ::std::result::Result<
        crate::waiters::environment_template_version_registered::EnvironmentTemplateVersionRegisteredFinalPoll,
        crate::waiters::environment_template_version_registered::WaitUntilEnvironmentTemplateVersionRegisteredError,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::waiters::error::WaiterError::construction_failure)?;
        let runtime_plugins = crate::operation::get_environment_template_version::GetEnvironmentTemplateVersion::operation_runtime_plugins(
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
            &crate::operation::get_environment_template_version::GetEnvironmentTemplateVersionOutput,
            &crate::operation::get_environment_template_version::GetEnvironmentTemplateVersionError,
        >| {
            // Matches: {"output":{"path":"environmentTemplateVersion.status","expected":"DRAFT","comparator":"stringEquals"}}
            if crate::waiters::matchers::match_get_environment_template_version_a5b3f466aeb30c257(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Success;
            }
            // Matches: {"output":{"path":"environmentTemplateVersion.status","expected":"PUBLISHED","comparator":"stringEquals"}}
            if crate::waiters::matchers::match_get_environment_template_version_0828932c125f5b1cd(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Success;
            }
            // Matches: {"output":{"path":"environmentTemplateVersion.status","expected":"REGISTRATION_FAILED","comparator":"stringEquals"}}
            if crate::waiters::matchers::match_get_environment_template_version_b4971a1e3bf3ec4bc(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Failure;
            }
            ::aws_smithy_runtime::client::waiters::AcceptorState::NoAcceptorsMatched
        };
        let operation = move || {
            let input = input.clone();
            let runtime_plugins = runtime_plugins.clone();
            async move { crate::operation::get_environment_template_version::GetEnvironmentTemplateVersion::orchestrate(&runtime_plugins, input).await }
        };
        let orchestrator = ::aws_smithy_runtime::client::waiters::WaiterOrchestrator::builder()
            .min_delay(::std::time::Duration::from_secs(2))
            .max_delay(::std::time::Duration::from_secs(300))
            .max_wait(max_wait)
            .time_source(time_source)
            .sleep_impl(sleep_impl)
            .acceptor(acceptor)
            .operation(operation)
            .build();
        ::aws_smithy_runtime::client::waiters::attach_waiter_tracing_span(orchestrator.orchestrate()).await
    }
    /// <p>The name of the environment template a version of which you want to get detailed data for.</p>
    pub fn template_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.template_name(input.into());
        self
    }
    /// <p>The name of the environment template a version of which you want to get detailed data for.</p>
    pub fn set_template_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_template_name(input);
        self
    }
    /// <p>The name of the environment template a version of which you want to get detailed data for.</p>
    pub fn get_template_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_template_name()
    }
    /// <p>To get environment template major version detail data, include <code>major Version</code>.</p>
    pub fn major_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.major_version(input.into());
        self
    }
    /// <p>To get environment template major version detail data, include <code>major Version</code>.</p>
    pub fn set_major_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_major_version(input);
        self
    }
    /// <p>To get environment template major version detail data, include <code>major Version</code>.</p>
    pub fn get_major_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_major_version()
    }
    /// <p>To get environment template minor version detail data, include <code>minorVersion</code>.</p>
    pub fn minor_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.minor_version(input.into());
        self
    }
    /// <p>To get environment template minor version detail data, include <code>minorVersion</code>.</p>
    pub fn set_minor_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_minor_version(input);
        self
    }
    /// <p>To get environment template minor version detail data, include <code>minorVersion</code>.</p>
    pub fn get_minor_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_minor_version()
    }
}

/// Successful return type for the `environment_template_version_registered` waiter.
pub type EnvironmentTemplateVersionRegisteredFinalPoll = ::aws_smithy_runtime_api::client::waiters::FinalPoll<
    crate::operation::get_environment_template_version::GetEnvironmentTemplateVersionOutput,
    ::aws_smithy_runtime_api::client::result::SdkError<
        crate::operation::get_environment_template_version::GetEnvironmentTemplateVersionError,
        ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    >,
>;

/// Error type for the `environment_template_version_registered` waiter.
pub type WaitUntilEnvironmentTemplateVersionRegisteredError = ::aws_smithy_runtime_api::client::waiters::error::WaiterError<
    crate::operation::get_environment_template_version::GetEnvironmentTemplateVersionOutput,
    crate::operation::get_environment_template_version::GetEnvironmentTemplateVersionError,
>;
