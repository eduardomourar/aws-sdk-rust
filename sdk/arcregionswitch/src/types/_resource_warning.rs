// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents a warning about a resource in a Region switch plan.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ResourceWarning {
    /// <p>The workflow for the resource warning.</p>
    pub workflow: ::std::option::Option<crate::types::MinimalWorkflow>,
    /// <p>The version for the resource warning.</p>
    pub version: ::std::string::String,
    /// <p>The name of the step for the resource warning.</p>
    pub step_name: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    pub resource_arn: ::std::option::Option<::std::string::String>,
    /// <p>The status of the resource warning.</p>
    pub warning_status: crate::types::ResourceWarningStatus,
    /// <p>The timestamp when the warning was last updated.</p>
    pub warning_updated_time: ::aws_smithy_types::DateTime,
    /// <p>The warning message about what needs to be corrected.</p>
    pub warning_message: ::std::string::String,
}
impl ResourceWarning {
    /// <p>The workflow for the resource warning.</p>
    pub fn workflow(&self) -> ::std::option::Option<&crate::types::MinimalWorkflow> {
        self.workflow.as_ref()
    }
    /// <p>The version for the resource warning.</p>
    pub fn version(&self) -> &str {
        use std::ops::Deref;
        self.version.deref()
    }
    /// <p>The name of the step for the resource warning.</p>
    pub fn step_name(&self) -> ::std::option::Option<&str> {
        self.step_name.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    pub fn resource_arn(&self) -> ::std::option::Option<&str> {
        self.resource_arn.as_deref()
    }
    /// <p>The status of the resource warning.</p>
    pub fn warning_status(&self) -> &crate::types::ResourceWarningStatus {
        &self.warning_status
    }
    /// <p>The timestamp when the warning was last updated.</p>
    pub fn warning_updated_time(&self) -> &::aws_smithy_types::DateTime {
        &self.warning_updated_time
    }
    /// <p>The warning message about what needs to be corrected.</p>
    pub fn warning_message(&self) -> &str {
        use std::ops::Deref;
        self.warning_message.deref()
    }
}
impl ResourceWarning {
    /// Creates a new builder-style object to manufacture [`ResourceWarning`](crate::types::ResourceWarning).
    pub fn builder() -> crate::types::builders::ResourceWarningBuilder {
        crate::types::builders::ResourceWarningBuilder::default()
    }
}

/// A builder for [`ResourceWarning`](crate::types::ResourceWarning).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ResourceWarningBuilder {
    pub(crate) workflow: ::std::option::Option<crate::types::MinimalWorkflow>,
    pub(crate) version: ::std::option::Option<::std::string::String>,
    pub(crate) step_name: ::std::option::Option<::std::string::String>,
    pub(crate) resource_arn: ::std::option::Option<::std::string::String>,
    pub(crate) warning_status: ::std::option::Option<crate::types::ResourceWarningStatus>,
    pub(crate) warning_updated_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) warning_message: ::std::option::Option<::std::string::String>,
}
impl ResourceWarningBuilder {
    /// <p>The workflow for the resource warning.</p>
    pub fn workflow(mut self, input: crate::types::MinimalWorkflow) -> Self {
        self.workflow = ::std::option::Option::Some(input);
        self
    }
    /// <p>The workflow for the resource warning.</p>
    pub fn set_workflow(mut self, input: ::std::option::Option<crate::types::MinimalWorkflow>) -> Self {
        self.workflow = input;
        self
    }
    /// <p>The workflow for the resource warning.</p>
    pub fn get_workflow(&self) -> &::std::option::Option<crate::types::MinimalWorkflow> {
        &self.workflow
    }
    /// <p>The version for the resource warning.</p>
    /// This field is required.
    pub fn version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The version for the resource warning.</p>
    pub fn set_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.version = input;
        self
    }
    /// <p>The version for the resource warning.</p>
    pub fn get_version(&self) -> &::std::option::Option<::std::string::String> {
        &self.version
    }
    /// <p>The name of the step for the resource warning.</p>
    pub fn step_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.step_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the step for the resource warning.</p>
    pub fn set_step_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.step_name = input;
        self
    }
    /// <p>The name of the step for the resource warning.</p>
    pub fn get_step_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.step_name
    }
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    pub fn resource_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    pub fn set_resource_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    pub fn get_resource_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.resource_arn
    }
    /// <p>The status of the resource warning.</p>
    /// This field is required.
    pub fn warning_status(mut self, input: crate::types::ResourceWarningStatus) -> Self {
        self.warning_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the resource warning.</p>
    pub fn set_warning_status(mut self, input: ::std::option::Option<crate::types::ResourceWarningStatus>) -> Self {
        self.warning_status = input;
        self
    }
    /// <p>The status of the resource warning.</p>
    pub fn get_warning_status(&self) -> &::std::option::Option<crate::types::ResourceWarningStatus> {
        &self.warning_status
    }
    /// <p>The timestamp when the warning was last updated.</p>
    /// This field is required.
    pub fn warning_updated_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.warning_updated_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The timestamp when the warning was last updated.</p>
    pub fn set_warning_updated_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.warning_updated_time = input;
        self
    }
    /// <p>The timestamp when the warning was last updated.</p>
    pub fn get_warning_updated_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.warning_updated_time
    }
    /// <p>The warning message about what needs to be corrected.</p>
    /// This field is required.
    pub fn warning_message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.warning_message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The warning message about what needs to be corrected.</p>
    pub fn set_warning_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.warning_message = input;
        self
    }
    /// <p>The warning message about what needs to be corrected.</p>
    pub fn get_warning_message(&self) -> &::std::option::Option<::std::string::String> {
        &self.warning_message
    }
    /// Consumes the builder and constructs a [`ResourceWarning`](crate::types::ResourceWarning).
    /// This method will fail if any of the following fields are not set:
    /// - [`version`](crate::types::builders::ResourceWarningBuilder::version)
    /// - [`warning_status`](crate::types::builders::ResourceWarningBuilder::warning_status)
    /// - [`warning_updated_time`](crate::types::builders::ResourceWarningBuilder::warning_updated_time)
    /// - [`warning_message`](crate::types::builders::ResourceWarningBuilder::warning_message)
    pub fn build(self) -> ::std::result::Result<crate::types::ResourceWarning, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::ResourceWarning {
            workflow: self.workflow,
            version: self.version.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "version",
                    "version was not specified but it is required when building ResourceWarning",
                )
            })?,
            step_name: self.step_name,
            resource_arn: self.resource_arn,
            warning_status: self.warning_status.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "warning_status",
                    "warning_status was not specified but it is required when building ResourceWarning",
                )
            })?,
            warning_updated_time: self.warning_updated_time.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "warning_updated_time",
                    "warning_updated_time was not specified but it is required when building ResourceWarning",
                )
            })?,
            warning_message: self.warning_message.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "warning_message",
                    "warning_message was not specified but it is required when building ResourceWarning",
                )
            })?,
        })
    }
}
