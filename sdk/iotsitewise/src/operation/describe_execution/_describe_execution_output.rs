// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeExecutionOutput {
    /// <p>The ID of the execution.</p>
    pub execution_id: ::std::string::String,
    /// <p>The type of action exectued.</p>
    pub action_type: ::std::option::Option<::std::string::String>,
    /// <p>The resource the action will be taken on.</p>
    pub target_resource: ::std::option::Option<crate::types::TargetResource>,
    /// <p>The version of the target resource.</p>
    pub target_resource_version: ::std::string::String,
    /// <p>The detailed resource this execution resolves to.</p>
    pub resolve_to: ::std::option::Option<crate::types::ResolveTo>,
    /// <p>The time the process started.</p>
    pub execution_start_time: ::aws_smithy_types::DateTime,
    /// <p>The time the process ended.</p>
    pub execution_end_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The status of the execution process.</p>
    pub execution_status: ::std::option::Option<crate::types::ExecutionStatus>,
    /// <p>The result of the execution.</p>
    pub execution_result: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    /// <p>Provides detailed information about the execution of your anomaly detection models. This includes model metrics and training timestamps for both training and inference actions.</p>
    /// <ul>
    /// <li>
    /// <p>The training action (Amazon Web Services/ANOMALY_DETECTION_TRAINING), includes performance metrics that help you compare different versions of your anomaly detection models. These metrics provide insights into the model's performance during the training process.</p></li>
    /// <li>
    /// <p>The inference action (Amazon Web Services/ANOMALY_DETECTION_INFERENCE), includes information about the results of executing your anomaly detection models. This helps you understand the output of your models and assess their performance.</p></li>
    /// </ul>
    pub execution_details: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    /// <p>Entity version used for the execution.</p>
    pub execution_entity_version: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeExecutionOutput {
    /// <p>The ID of the execution.</p>
    pub fn execution_id(&self) -> &str {
        use std::ops::Deref;
        self.execution_id.deref()
    }
    /// <p>The type of action exectued.</p>
    pub fn action_type(&self) -> ::std::option::Option<&str> {
        self.action_type.as_deref()
    }
    /// <p>The resource the action will be taken on.</p>
    pub fn target_resource(&self) -> ::std::option::Option<&crate::types::TargetResource> {
        self.target_resource.as_ref()
    }
    /// <p>The version of the target resource.</p>
    pub fn target_resource_version(&self) -> &str {
        use std::ops::Deref;
        self.target_resource_version.deref()
    }
    /// <p>The detailed resource this execution resolves to.</p>
    pub fn resolve_to(&self) -> ::std::option::Option<&crate::types::ResolveTo> {
        self.resolve_to.as_ref()
    }
    /// <p>The time the process started.</p>
    pub fn execution_start_time(&self) -> &::aws_smithy_types::DateTime {
        &self.execution_start_time
    }
    /// <p>The time the process ended.</p>
    pub fn execution_end_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.execution_end_time.as_ref()
    }
    /// <p>The status of the execution process.</p>
    pub fn execution_status(&self) -> ::std::option::Option<&crate::types::ExecutionStatus> {
        self.execution_status.as_ref()
    }
    /// <p>The result of the execution.</p>
    pub fn execution_result(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.execution_result.as_ref()
    }
    /// <p>Provides detailed information about the execution of your anomaly detection models. This includes model metrics and training timestamps for both training and inference actions.</p>
    /// <ul>
    /// <li>
    /// <p>The training action (Amazon Web Services/ANOMALY_DETECTION_TRAINING), includes performance metrics that help you compare different versions of your anomaly detection models. These metrics provide insights into the model's performance during the training process.</p></li>
    /// <li>
    /// <p>The inference action (Amazon Web Services/ANOMALY_DETECTION_INFERENCE), includes information about the results of executing your anomaly detection models. This helps you understand the output of your models and assess their performance.</p></li>
    /// </ul>
    pub fn execution_details(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.execution_details.as_ref()
    }
    /// <p>Entity version used for the execution.</p>
    pub fn execution_entity_version(&self) -> ::std::option::Option<&str> {
        self.execution_entity_version.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for DescribeExecutionOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeExecutionOutput {
    /// Creates a new builder-style object to manufacture [`DescribeExecutionOutput`](crate::operation::describe_execution::DescribeExecutionOutput).
    pub fn builder() -> crate::operation::describe_execution::builders::DescribeExecutionOutputBuilder {
        crate::operation::describe_execution::builders::DescribeExecutionOutputBuilder::default()
    }
}

/// A builder for [`DescribeExecutionOutput`](crate::operation::describe_execution::DescribeExecutionOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DescribeExecutionOutputBuilder {
    pub(crate) execution_id: ::std::option::Option<::std::string::String>,
    pub(crate) action_type: ::std::option::Option<::std::string::String>,
    pub(crate) target_resource: ::std::option::Option<crate::types::TargetResource>,
    pub(crate) target_resource_version: ::std::option::Option<::std::string::String>,
    pub(crate) resolve_to: ::std::option::Option<crate::types::ResolveTo>,
    pub(crate) execution_start_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) execution_end_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) execution_status: ::std::option::Option<crate::types::ExecutionStatus>,
    pub(crate) execution_result: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    pub(crate) execution_details: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    pub(crate) execution_entity_version: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeExecutionOutputBuilder {
    /// <p>The ID of the execution.</p>
    /// This field is required.
    pub fn execution_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.execution_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the execution.</p>
    pub fn set_execution_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.execution_id = input;
        self
    }
    /// <p>The ID of the execution.</p>
    pub fn get_execution_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.execution_id
    }
    /// <p>The type of action exectued.</p>
    pub fn action_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.action_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The type of action exectued.</p>
    pub fn set_action_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.action_type = input;
        self
    }
    /// <p>The type of action exectued.</p>
    pub fn get_action_type(&self) -> &::std::option::Option<::std::string::String> {
        &self.action_type
    }
    /// <p>The resource the action will be taken on.</p>
    /// This field is required.
    pub fn target_resource(mut self, input: crate::types::TargetResource) -> Self {
        self.target_resource = ::std::option::Option::Some(input);
        self
    }
    /// <p>The resource the action will be taken on.</p>
    pub fn set_target_resource(mut self, input: ::std::option::Option<crate::types::TargetResource>) -> Self {
        self.target_resource = input;
        self
    }
    /// <p>The resource the action will be taken on.</p>
    pub fn get_target_resource(&self) -> &::std::option::Option<crate::types::TargetResource> {
        &self.target_resource
    }
    /// <p>The version of the target resource.</p>
    /// This field is required.
    pub fn target_resource_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.target_resource_version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The version of the target resource.</p>
    pub fn set_target_resource_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.target_resource_version = input;
        self
    }
    /// <p>The version of the target resource.</p>
    pub fn get_target_resource_version(&self) -> &::std::option::Option<::std::string::String> {
        &self.target_resource_version
    }
    /// <p>The detailed resource this execution resolves to.</p>
    pub fn resolve_to(mut self, input: crate::types::ResolveTo) -> Self {
        self.resolve_to = ::std::option::Option::Some(input);
        self
    }
    /// <p>The detailed resource this execution resolves to.</p>
    pub fn set_resolve_to(mut self, input: ::std::option::Option<crate::types::ResolveTo>) -> Self {
        self.resolve_to = input;
        self
    }
    /// <p>The detailed resource this execution resolves to.</p>
    pub fn get_resolve_to(&self) -> &::std::option::Option<crate::types::ResolveTo> {
        &self.resolve_to
    }
    /// <p>The time the process started.</p>
    /// This field is required.
    pub fn execution_start_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.execution_start_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time the process started.</p>
    pub fn set_execution_start_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.execution_start_time = input;
        self
    }
    /// <p>The time the process started.</p>
    pub fn get_execution_start_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.execution_start_time
    }
    /// <p>The time the process ended.</p>
    pub fn execution_end_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.execution_end_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time the process ended.</p>
    pub fn set_execution_end_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.execution_end_time = input;
        self
    }
    /// <p>The time the process ended.</p>
    pub fn get_execution_end_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.execution_end_time
    }
    /// <p>The status of the execution process.</p>
    /// This field is required.
    pub fn execution_status(mut self, input: crate::types::ExecutionStatus) -> Self {
        self.execution_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the execution process.</p>
    pub fn set_execution_status(mut self, input: ::std::option::Option<crate::types::ExecutionStatus>) -> Self {
        self.execution_status = input;
        self
    }
    /// <p>The status of the execution process.</p>
    pub fn get_execution_status(&self) -> &::std::option::Option<crate::types::ExecutionStatus> {
        &self.execution_status
    }
    /// Adds a key-value pair to `execution_result`.
    ///
    /// To override the contents of this collection use [`set_execution_result`](Self::set_execution_result).
    ///
    /// <p>The result of the execution.</p>
    pub fn execution_result(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.execution_result.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.execution_result = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>The result of the execution.</p>
    pub fn set_execution_result(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    ) -> Self {
        self.execution_result = input;
        self
    }
    /// <p>The result of the execution.</p>
    pub fn get_execution_result(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        &self.execution_result
    }
    /// Adds a key-value pair to `execution_details`.
    ///
    /// To override the contents of this collection use [`set_execution_details`](Self::set_execution_details).
    ///
    /// <p>Provides detailed information about the execution of your anomaly detection models. This includes model metrics and training timestamps for both training and inference actions.</p>
    /// <ul>
    /// <li>
    /// <p>The training action (Amazon Web Services/ANOMALY_DETECTION_TRAINING), includes performance metrics that help you compare different versions of your anomaly detection models. These metrics provide insights into the model's performance during the training process.</p></li>
    /// <li>
    /// <p>The inference action (Amazon Web Services/ANOMALY_DETECTION_INFERENCE), includes information about the results of executing your anomaly detection models. This helps you understand the output of your models and assess their performance.</p></li>
    /// </ul>
    pub fn execution_details(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        let mut hash_map = self.execution_details.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.execution_details = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>Provides detailed information about the execution of your anomaly detection models. This includes model metrics and training timestamps for both training and inference actions.</p>
    /// <ul>
    /// <li>
    /// <p>The training action (Amazon Web Services/ANOMALY_DETECTION_TRAINING), includes performance metrics that help you compare different versions of your anomaly detection models. These metrics provide insights into the model's performance during the training process.</p></li>
    /// <li>
    /// <p>The inference action (Amazon Web Services/ANOMALY_DETECTION_INFERENCE), includes information about the results of executing your anomaly detection models. This helps you understand the output of your models and assess their performance.</p></li>
    /// </ul>
    pub fn set_execution_details(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    ) -> Self {
        self.execution_details = input;
        self
    }
    /// <p>Provides detailed information about the execution of your anomaly detection models. This includes model metrics and training timestamps for both training and inference actions.</p>
    /// <ul>
    /// <li>
    /// <p>The training action (Amazon Web Services/ANOMALY_DETECTION_TRAINING), includes performance metrics that help you compare different versions of your anomaly detection models. These metrics provide insights into the model's performance during the training process.</p></li>
    /// <li>
    /// <p>The inference action (Amazon Web Services/ANOMALY_DETECTION_INFERENCE), includes information about the results of executing your anomaly detection models. This helps you understand the output of your models and assess their performance.</p></li>
    /// </ul>
    pub fn get_execution_details(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        &self.execution_details
    }
    /// <p>Entity version used for the execution.</p>
    pub fn execution_entity_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.execution_entity_version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Entity version used for the execution.</p>
    pub fn set_execution_entity_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.execution_entity_version = input;
        self
    }
    /// <p>Entity version used for the execution.</p>
    pub fn get_execution_entity_version(&self) -> &::std::option::Option<::std::string::String> {
        &self.execution_entity_version
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DescribeExecutionOutput`](crate::operation::describe_execution::DescribeExecutionOutput).
    /// This method will fail if any of the following fields are not set:
    /// - [`execution_id`](crate::operation::describe_execution::builders::DescribeExecutionOutputBuilder::execution_id)
    /// - [`target_resource_version`](crate::operation::describe_execution::builders::DescribeExecutionOutputBuilder::target_resource_version)
    /// - [`execution_start_time`](crate::operation::describe_execution::builders::DescribeExecutionOutputBuilder::execution_start_time)
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::describe_execution::DescribeExecutionOutput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::describe_execution::DescribeExecutionOutput {
            execution_id: self.execution_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "execution_id",
                    "execution_id was not specified but it is required when building DescribeExecutionOutput",
                )
            })?,
            action_type: self.action_type,
            target_resource: self.target_resource,
            target_resource_version: self.target_resource_version.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "target_resource_version",
                    "target_resource_version was not specified but it is required when building DescribeExecutionOutput",
                )
            })?,
            resolve_to: self.resolve_to,
            execution_start_time: self.execution_start_time.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "execution_start_time",
                    "execution_start_time was not specified but it is required when building DescribeExecutionOutput",
                )
            })?,
            execution_end_time: self.execution_end_time,
            execution_status: self.execution_status,
            execution_result: self.execution_result,
            execution_details: self.execution_details,
            execution_entity_version: self.execution_entity_version,
            _request_id: self._request_id,
        })
    }
}
