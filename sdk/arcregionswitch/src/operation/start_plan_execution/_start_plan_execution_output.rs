// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StartPlanExecutionOutput {
    /// <p>The execution identifier of a plan execution.</p>
    pub execution_id: ::std::option::Option<::std::string::String>,
    /// <p>The details of the Region switch plan.</p>
    pub plan: ::std::option::Option<::std::string::String>,
    /// <p>The version of the plan, a unique number generated by Region switch.</p>
    pub plan_version: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Web Services Region to activate.</p>
    pub activate_region: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Web Services Region to deactivate.</p>
    pub deactivate_region: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl StartPlanExecutionOutput {
    /// <p>The execution identifier of a plan execution.</p>
    pub fn execution_id(&self) -> ::std::option::Option<&str> {
        self.execution_id.as_deref()
    }
    /// <p>The details of the Region switch plan.</p>
    pub fn plan(&self) -> ::std::option::Option<&str> {
        self.plan.as_deref()
    }
    /// <p>The version of the plan, a unique number generated by Region switch.</p>
    pub fn plan_version(&self) -> ::std::option::Option<&str> {
        self.plan_version.as_deref()
    }
    /// <p>The Amazon Web Services Region to activate.</p>
    pub fn activate_region(&self) -> ::std::option::Option<&str> {
        self.activate_region.as_deref()
    }
    /// <p>The Amazon Web Services Region to deactivate.</p>
    pub fn deactivate_region(&self) -> ::std::option::Option<&str> {
        self.deactivate_region.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for StartPlanExecutionOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl StartPlanExecutionOutput {
    /// Creates a new builder-style object to manufacture [`StartPlanExecutionOutput`](crate::operation::start_plan_execution::StartPlanExecutionOutput).
    pub fn builder() -> crate::operation::start_plan_execution::builders::StartPlanExecutionOutputBuilder {
        crate::operation::start_plan_execution::builders::StartPlanExecutionOutputBuilder::default()
    }
}

/// A builder for [`StartPlanExecutionOutput`](crate::operation::start_plan_execution::StartPlanExecutionOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct StartPlanExecutionOutputBuilder {
    pub(crate) execution_id: ::std::option::Option<::std::string::String>,
    pub(crate) plan: ::std::option::Option<::std::string::String>,
    pub(crate) plan_version: ::std::option::Option<::std::string::String>,
    pub(crate) activate_region: ::std::option::Option<::std::string::String>,
    pub(crate) deactivate_region: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl StartPlanExecutionOutputBuilder {
    /// <p>The execution identifier of a plan execution.</p>
    pub fn execution_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.execution_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The execution identifier of a plan execution.</p>
    pub fn set_execution_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.execution_id = input;
        self
    }
    /// <p>The execution identifier of a plan execution.</p>
    pub fn get_execution_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.execution_id
    }
    /// <p>The details of the Region switch plan.</p>
    pub fn plan(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.plan = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The details of the Region switch plan.</p>
    pub fn set_plan(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.plan = input;
        self
    }
    /// <p>The details of the Region switch plan.</p>
    pub fn get_plan(&self) -> &::std::option::Option<::std::string::String> {
        &self.plan
    }
    /// <p>The version of the plan, a unique number generated by Region switch.</p>
    pub fn plan_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.plan_version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The version of the plan, a unique number generated by Region switch.</p>
    pub fn set_plan_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.plan_version = input;
        self
    }
    /// <p>The version of the plan, a unique number generated by Region switch.</p>
    pub fn get_plan_version(&self) -> &::std::option::Option<::std::string::String> {
        &self.plan_version
    }
    /// <p>The Amazon Web Services Region to activate.</p>
    pub fn activate_region(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.activate_region = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Web Services Region to activate.</p>
    pub fn set_activate_region(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.activate_region = input;
        self
    }
    /// <p>The Amazon Web Services Region to activate.</p>
    pub fn get_activate_region(&self) -> &::std::option::Option<::std::string::String> {
        &self.activate_region
    }
    /// <p>The Amazon Web Services Region to deactivate.</p>
    pub fn deactivate_region(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.deactivate_region = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Web Services Region to deactivate.</p>
    pub fn set_deactivate_region(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.deactivate_region = input;
        self
    }
    /// <p>The Amazon Web Services Region to deactivate.</p>
    pub fn get_deactivate_region(&self) -> &::std::option::Option<::std::string::String> {
        &self.deactivate_region
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`StartPlanExecutionOutput`](crate::operation::start_plan_execution::StartPlanExecutionOutput).
    pub fn build(self) -> crate::operation::start_plan_execution::StartPlanExecutionOutput {
        crate::operation::start_plan_execution::StartPlanExecutionOutput {
            execution_id: self.execution_id,
            plan: self.plan,
            plan_version: self.plan_version,
            activate_region: self.activate_region,
            deactivate_region: self.deactivate_region,
            _request_id: self._request_id,
        }
    }
}
