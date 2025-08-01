// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_plan_execution::_start_plan_execution_output::StartPlanExecutionOutputBuilder;

pub use crate::operation::start_plan_execution::_start_plan_execution_input::StartPlanExecutionInputBuilder;

impl crate::operation::start_plan_execution::builders::StartPlanExecutionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::start_plan_execution::StartPlanExecutionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_plan_execution::StartPlanExecutionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.start_plan_execution();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StartPlanExecution`.
///
/// <p>Starts the execution of a Region switch plan. You can execute a plan in either PRACTICE or RECOVERY mode.</p>
/// <p>In PRACTICE mode, the execution simulates the steps without making actual changes to your application's traffic routing. In RECOVERY mode, the execution performs actual changes to shift traffic between Regions.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartPlanExecutionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_plan_execution::builders::StartPlanExecutionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::start_plan_execution::StartPlanExecutionOutput,
        crate::operation::start_plan_execution::StartPlanExecutionError,
    > for StartPlanExecutionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::start_plan_execution::StartPlanExecutionOutput,
            crate::operation::start_plan_execution::StartPlanExecutionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl StartPlanExecutionFluentBuilder {
    /// Creates a new `StartPlanExecutionFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StartPlanExecution as a reference.
    pub fn as_input(&self) -> &crate::operation::start_plan_execution::builders::StartPlanExecutionInputBuilder {
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
        crate::operation::start_plan_execution::StartPlanExecutionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_plan_execution::StartPlanExecutionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::start_plan_execution::StartPlanExecution::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::start_plan_execution::StartPlanExecution::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::start_plan_execution::StartPlanExecutionOutput,
        crate::operation::start_plan_execution::StartPlanExecutionError,
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
    /// <p>The Amazon Resource Name (ARN) of the plan to execute.</p>
    pub fn plan_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.plan_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the plan to execute.</p>
    pub fn set_plan_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_plan_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the plan to execute.</p>
    pub fn get_plan_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_plan_arn()
    }
    /// <p>The Amazon Web Services Region to target with this execution. This is the Region that traffic will be shifted to or from, depending on the action.</p>
    pub fn target_region(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.target_region(input.into());
        self
    }
    /// <p>The Amazon Web Services Region to target with this execution. This is the Region that traffic will be shifted to or from, depending on the action.</p>
    pub fn set_target_region(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_target_region(input);
        self
    }
    /// <p>The Amazon Web Services Region to target with this execution. This is the Region that traffic will be shifted to or from, depending on the action.</p>
    pub fn get_target_region(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_target_region()
    }
    /// <p>The action to perform. Valid values are ACTIVATE (to shift traffic to the target Region) or DEACTIVATE (to shift traffic away from the target Region).</p>
    pub fn action(mut self, input: crate::types::ExecutionAction) -> Self {
        self.inner = self.inner.action(input);
        self
    }
    /// <p>The action to perform. Valid values are ACTIVATE (to shift traffic to the target Region) or DEACTIVATE (to shift traffic away from the target Region).</p>
    pub fn set_action(mut self, input: ::std::option::Option<crate::types::ExecutionAction>) -> Self {
        self.inner = self.inner.set_action(input);
        self
    }
    /// <p>The action to perform. Valid values are ACTIVATE (to shift traffic to the target Region) or DEACTIVATE (to shift traffic away from the target Region).</p>
    pub fn get_action(&self) -> &::std::option::Option<crate::types::ExecutionAction> {
        self.inner.get_action()
    }
    /// <p>The plan execution mode. Valid values are <code>Practice</code>, for testing without making actual changes, or <code>Recovery</code>, for actual traffic shifting and application recovery.</p>
    pub fn mode(mut self, input: crate::types::ExecutionMode) -> Self {
        self.inner = self.inner.mode(input);
        self
    }
    /// <p>The plan execution mode. Valid values are <code>Practice</code>, for testing without making actual changes, or <code>Recovery</code>, for actual traffic shifting and application recovery.</p>
    pub fn set_mode(mut self, input: ::std::option::Option<crate::types::ExecutionMode>) -> Self {
        self.inner = self.inner.set_mode(input);
        self
    }
    /// <p>The plan execution mode. Valid values are <code>Practice</code>, for testing without making actual changes, or <code>Recovery</code>, for actual traffic shifting and application recovery.</p>
    pub fn get_mode(&self) -> &::std::option::Option<crate::types::ExecutionMode> {
        self.inner.get_mode()
    }
    /// <p>An optional comment explaining why the plan execution is being started.</p>
    pub fn comment(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.comment(input.into());
        self
    }
    /// <p>An optional comment explaining why the plan execution is being started.</p>
    pub fn set_comment(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_comment(input);
        self
    }
    /// <p>An optional comment explaining why the plan execution is being started.</p>
    pub fn get_comment(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_comment()
    }
    /// <p>A boolean value indicating whether to use the latest version of the plan. If set to false, you must specify a specific version.</p>
    pub fn latest_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.latest_version(input.into());
        self
    }
    /// <p>A boolean value indicating whether to use the latest version of the plan. If set to false, you must specify a specific version.</p>
    pub fn set_latest_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_latest_version(input);
        self
    }
    /// <p>A boolean value indicating whether to use the latest version of the plan. If set to false, you must specify a specific version.</p>
    pub fn get_latest_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_latest_version()
    }
}
