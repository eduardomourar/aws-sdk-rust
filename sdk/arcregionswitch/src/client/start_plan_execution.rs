// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StartPlanExecution`](crate::operation::start_plan_execution::builders::StartPlanExecutionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`plan_arn(impl Into<String>)`](crate::operation::start_plan_execution::builders::StartPlanExecutionFluentBuilder::plan_arn) / [`set_plan_arn(Option<String>)`](crate::operation::start_plan_execution::builders::StartPlanExecutionFluentBuilder::set_plan_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the plan to execute.</p><br>
    ///   - [`target_region(impl Into<String>)`](crate::operation::start_plan_execution::builders::StartPlanExecutionFluentBuilder::target_region) / [`set_target_region(Option<String>)`](crate::operation::start_plan_execution::builders::StartPlanExecutionFluentBuilder::set_target_region):<br>required: **true**<br><p>The Amazon Web Services Region to target with this execution. This is the Region that traffic will be shifted to or from, depending on the action.</p><br>
    ///   - [`action(ExecutionAction)`](crate::operation::start_plan_execution::builders::StartPlanExecutionFluentBuilder::action) / [`set_action(Option<ExecutionAction>)`](crate::operation::start_plan_execution::builders::StartPlanExecutionFluentBuilder::set_action):<br>required: **true**<br><p>The action to perform. Valid values are ACTIVATE (to shift traffic to the target Region) or DEACTIVATE (to shift traffic away from the target Region).</p><br>
    ///   - [`mode(ExecutionMode)`](crate::operation::start_plan_execution::builders::StartPlanExecutionFluentBuilder::mode) / [`set_mode(Option<ExecutionMode>)`](crate::operation::start_plan_execution::builders::StartPlanExecutionFluentBuilder::set_mode):<br>required: **false**<br><p>The plan execution mode. Valid values are <code>Practice</code>, for testing without making actual changes, or <code>Recovery</code>, for actual traffic shifting and application recovery.</p><br>
    ///   - [`comment(impl Into<String>)`](crate::operation::start_plan_execution::builders::StartPlanExecutionFluentBuilder::comment) / [`set_comment(Option<String>)`](crate::operation::start_plan_execution::builders::StartPlanExecutionFluentBuilder::set_comment):<br>required: **false**<br><p>An optional comment explaining why the plan execution is being started.</p><br>
    ///   - [`latest_version(impl Into<String>)`](crate::operation::start_plan_execution::builders::StartPlanExecutionFluentBuilder::latest_version) / [`set_latest_version(Option<String>)`](crate::operation::start_plan_execution::builders::StartPlanExecutionFluentBuilder::set_latest_version):<br>required: **false**<br><p>A boolean value indicating whether to use the latest version of the plan. If set to false, you must specify a specific version.</p><br>
    /// - On success, responds with [`StartPlanExecutionOutput`](crate::operation::start_plan_execution::StartPlanExecutionOutput) with field(s):
    ///   - [`execution_id(Option<String>)`](crate::operation::start_plan_execution::StartPlanExecutionOutput::execution_id): <p>The execution identifier of a plan execution.</p>
    ///   - [`plan(Option<String>)`](crate::operation::start_plan_execution::StartPlanExecutionOutput::plan): <p>The details of the Region switch plan.</p>
    ///   - [`plan_version(Option<String>)`](crate::operation::start_plan_execution::StartPlanExecutionOutput::plan_version): <p>The version of the plan, a unique number generated by Region switch.</p>
    ///   - [`activate_region(Option<String>)`](crate::operation::start_plan_execution::StartPlanExecutionOutput::activate_region): <p>The Amazon Web Services Region to activate.</p>
    ///   - [`deactivate_region(Option<String>)`](crate::operation::start_plan_execution::StartPlanExecutionOutput::deactivate_region): <p>The Amazon Web Services Region to deactivate.</p>
    /// - On failure, responds with [`SdkError<StartPlanExecutionError>`](crate::operation::start_plan_execution::StartPlanExecutionError)
    pub fn start_plan_execution(&self) -> crate::operation::start_plan_execution::builders::StartPlanExecutionFluentBuilder {
        crate::operation::start_plan_execution::builders::StartPlanExecutionFluentBuilder::new(self.handle.clone())
    }
}
