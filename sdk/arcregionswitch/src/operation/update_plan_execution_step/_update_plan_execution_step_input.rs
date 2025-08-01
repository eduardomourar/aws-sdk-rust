// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdatePlanExecutionStepInput {
    /// <p>The Amazon Resource Name (ARN) of the plan containing the execution step to update.</p>
    pub plan_arn: ::std::option::Option<::std::string::String>,
    /// <p>The unique identifier of the plan execution containing the step to update.</p>
    pub execution_id: ::std::option::Option<::std::string::String>,
    /// <p>An optional comment about the plan execution.</p>
    pub comment: ::std::option::Option<::std::string::String>,
    /// <p>The name of the execution step to update.</p>
    pub step_name: ::std::option::Option<::std::string::String>,
    /// <p>The updated action to take for the step. This can be used to skip or retry a step.</p>
    pub action_to_take: ::std::option::Option<crate::types::UpdatePlanExecutionStepAction>,
}
impl UpdatePlanExecutionStepInput {
    /// <p>The Amazon Resource Name (ARN) of the plan containing the execution step to update.</p>
    pub fn plan_arn(&self) -> ::std::option::Option<&str> {
        self.plan_arn.as_deref()
    }
    /// <p>The unique identifier of the plan execution containing the step to update.</p>
    pub fn execution_id(&self) -> ::std::option::Option<&str> {
        self.execution_id.as_deref()
    }
    /// <p>An optional comment about the plan execution.</p>
    pub fn comment(&self) -> ::std::option::Option<&str> {
        self.comment.as_deref()
    }
    /// <p>The name of the execution step to update.</p>
    pub fn step_name(&self) -> ::std::option::Option<&str> {
        self.step_name.as_deref()
    }
    /// <p>The updated action to take for the step. This can be used to skip or retry a step.</p>
    pub fn action_to_take(&self) -> ::std::option::Option<&crate::types::UpdatePlanExecutionStepAction> {
        self.action_to_take.as_ref()
    }
}
impl UpdatePlanExecutionStepInput {
    /// Creates a new builder-style object to manufacture [`UpdatePlanExecutionStepInput`](crate::operation::update_plan_execution_step::UpdatePlanExecutionStepInput).
    pub fn builder() -> crate::operation::update_plan_execution_step::builders::UpdatePlanExecutionStepInputBuilder {
        crate::operation::update_plan_execution_step::builders::UpdatePlanExecutionStepInputBuilder::default()
    }
}

/// A builder for [`UpdatePlanExecutionStepInput`](crate::operation::update_plan_execution_step::UpdatePlanExecutionStepInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct UpdatePlanExecutionStepInputBuilder {
    pub(crate) plan_arn: ::std::option::Option<::std::string::String>,
    pub(crate) execution_id: ::std::option::Option<::std::string::String>,
    pub(crate) comment: ::std::option::Option<::std::string::String>,
    pub(crate) step_name: ::std::option::Option<::std::string::String>,
    pub(crate) action_to_take: ::std::option::Option<crate::types::UpdatePlanExecutionStepAction>,
}
impl UpdatePlanExecutionStepInputBuilder {
    /// <p>The Amazon Resource Name (ARN) of the plan containing the execution step to update.</p>
    /// This field is required.
    pub fn plan_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.plan_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the plan containing the execution step to update.</p>
    pub fn set_plan_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.plan_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the plan containing the execution step to update.</p>
    pub fn get_plan_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.plan_arn
    }
    /// <p>The unique identifier of the plan execution containing the step to update.</p>
    /// This field is required.
    pub fn execution_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.execution_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier of the plan execution containing the step to update.</p>
    pub fn set_execution_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.execution_id = input;
        self
    }
    /// <p>The unique identifier of the plan execution containing the step to update.</p>
    pub fn get_execution_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.execution_id
    }
    /// <p>An optional comment about the plan execution.</p>
    /// This field is required.
    pub fn comment(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.comment = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An optional comment about the plan execution.</p>
    pub fn set_comment(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.comment = input;
        self
    }
    /// <p>An optional comment about the plan execution.</p>
    pub fn get_comment(&self) -> &::std::option::Option<::std::string::String> {
        &self.comment
    }
    /// <p>The name of the execution step to update.</p>
    /// This field is required.
    pub fn step_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.step_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the execution step to update.</p>
    pub fn set_step_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.step_name = input;
        self
    }
    /// <p>The name of the execution step to update.</p>
    pub fn get_step_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.step_name
    }
    /// <p>The updated action to take for the step. This can be used to skip or retry a step.</p>
    /// This field is required.
    pub fn action_to_take(mut self, input: crate::types::UpdatePlanExecutionStepAction) -> Self {
        self.action_to_take = ::std::option::Option::Some(input);
        self
    }
    /// <p>The updated action to take for the step. This can be used to skip or retry a step.</p>
    pub fn set_action_to_take(mut self, input: ::std::option::Option<crate::types::UpdatePlanExecutionStepAction>) -> Self {
        self.action_to_take = input;
        self
    }
    /// <p>The updated action to take for the step. This can be used to skip or retry a step.</p>
    pub fn get_action_to_take(&self) -> &::std::option::Option<crate::types::UpdatePlanExecutionStepAction> {
        &self.action_to_take
    }
    /// Consumes the builder and constructs a [`UpdatePlanExecutionStepInput`](crate::operation::update_plan_execution_step::UpdatePlanExecutionStepInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_plan_execution_step::UpdatePlanExecutionStepInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::update_plan_execution_step::UpdatePlanExecutionStepInput {
            plan_arn: self.plan_arn,
            execution_id: self.execution_id,
            comment: self.comment,
            step_name: self.step_name,
            action_to_take: self.action_to_take,
        })
    }
}
