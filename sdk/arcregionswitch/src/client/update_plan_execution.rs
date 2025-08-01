// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdatePlanExecution`](crate::operation::update_plan_execution::builders::UpdatePlanExecutionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`plan_arn(impl Into<String>)`](crate::operation::update_plan_execution::builders::UpdatePlanExecutionFluentBuilder::plan_arn) / [`set_plan_arn(Option<String>)`](crate::operation::update_plan_execution::builders::UpdatePlanExecutionFluentBuilder::set_plan_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the plan with the execution to update.</p><br>
    ///   - [`execution_id(impl Into<String>)`](crate::operation::update_plan_execution::builders::UpdatePlanExecutionFluentBuilder::execution_id) / [`set_execution_id(Option<String>)`](crate::operation::update_plan_execution::builders::UpdatePlanExecutionFluentBuilder::set_execution_id):<br>required: **true**<br><p>The execution identifier of a plan execution.</p><br>
    ///   - [`action(UpdatePlanExecutionAction)`](crate::operation::update_plan_execution::builders::UpdatePlanExecutionFluentBuilder::action) / [`set_action(Option<UpdatePlanExecutionAction>)`](crate::operation::update_plan_execution::builders::UpdatePlanExecutionFluentBuilder::set_action):<br>required: **true**<br><p>The action specified for a plan execution, for example, Switch to Graceful or Pause.</p><br>
    ///   - [`comment(impl Into<String>)`](crate::operation::update_plan_execution::builders::UpdatePlanExecutionFluentBuilder::comment) / [`set_comment(Option<String>)`](crate::operation::update_plan_execution::builders::UpdatePlanExecutionFluentBuilder::set_comment):<br>required: **false**<br><p>An optional comment about the plan execution.</p><br>
    /// - On success, responds with [`UpdatePlanExecutionOutput`](crate::operation::update_plan_execution::UpdatePlanExecutionOutput)
    /// - On failure, responds with [`SdkError<UpdatePlanExecutionError>`](crate::operation::update_plan_execution::UpdatePlanExecutionError)
    pub fn update_plan_execution(&self) -> crate::operation::update_plan_execution::builders::UpdatePlanExecutionFluentBuilder {
        crate::operation::update_plan_execution::builders::UpdatePlanExecutionFluentBuilder::new(self.handle.clone())
    }
}
