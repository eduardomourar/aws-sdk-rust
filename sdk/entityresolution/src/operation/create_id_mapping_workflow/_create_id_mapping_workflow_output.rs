// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateIdMappingWorkflowOutput {
    /// <p>The name of the workflow.</p>
    pub workflow_name: ::std::string::String,
    /// <p>The ARN (Amazon Resource Name) that Entity Resolution generated for the <code>IDMappingWorkflow</code>.</p>
    pub workflow_arn: ::std::string::String,
    /// <p>A description of the workflow.</p>
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>A list of <code>InputSource</code> objects, which have the fields <code>InputSourceARN</code> and <code>SchemaName</code>.</p>
    pub input_source_config: ::std::vec::Vec<crate::types::IdMappingWorkflowInputSource>,
    /// <p>A list of <code>IdMappingWorkflowOutputSource</code> objects, each of which contains fields <code>outputS3Path</code> and <code>KMSArn</code>.</p>
    pub output_source_config: ::std::option::Option<::std::vec::Vec<crate::types::IdMappingWorkflowOutputSource>>,
    /// <p>An object which defines the ID mapping technique and any additional configurations.</p>
    pub id_mapping_techniques: ::std::option::Option<crate::types::IdMappingTechniques>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role. Entity Resolution assumes this role to create resources on your behalf as part of workflow execution.</p>
    pub role_arn: ::std::string::String,
    _request_id: Option<String>,
}
impl CreateIdMappingWorkflowOutput {
    /// <p>The name of the workflow.</p>
    pub fn workflow_name(&self) -> &str {
        use std::ops::Deref;
        self.workflow_name.deref()
    }
    /// <p>The ARN (Amazon Resource Name) that Entity Resolution generated for the <code>IDMappingWorkflow</code>.</p>
    pub fn workflow_arn(&self) -> &str {
        use std::ops::Deref;
        self.workflow_arn.deref()
    }
    /// <p>A description of the workflow.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>A list of <code>InputSource</code> objects, which have the fields <code>InputSourceARN</code> and <code>SchemaName</code>.</p>
    pub fn input_source_config(&self) -> &[crate::types::IdMappingWorkflowInputSource] {
        use std::ops::Deref;
        self.input_source_config.deref()
    }
    /// <p>A list of <code>IdMappingWorkflowOutputSource</code> objects, each of which contains fields <code>outputS3Path</code> and <code>KMSArn</code>.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.output_source_config.is_none()`.
    pub fn output_source_config(&self) -> &[crate::types::IdMappingWorkflowOutputSource] {
        self.output_source_config.as_deref().unwrap_or_default()
    }
    /// <p>An object which defines the ID mapping technique and any additional configurations.</p>
    pub fn id_mapping_techniques(&self) -> ::std::option::Option<&crate::types::IdMappingTechniques> {
        self.id_mapping_techniques.as_ref()
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role. Entity Resolution assumes this role to create resources on your behalf as part of workflow execution.</p>
    pub fn role_arn(&self) -> &str {
        use std::ops::Deref;
        self.role_arn.deref()
    }
}
impl ::aws_types::request_id::RequestId for CreateIdMappingWorkflowOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateIdMappingWorkflowOutput {
    /// Creates a new builder-style object to manufacture [`CreateIdMappingWorkflowOutput`](crate::operation::create_id_mapping_workflow::CreateIdMappingWorkflowOutput).
    pub fn builder() -> crate::operation::create_id_mapping_workflow::builders::CreateIdMappingWorkflowOutputBuilder {
        crate::operation::create_id_mapping_workflow::builders::CreateIdMappingWorkflowOutputBuilder::default()
    }
}

/// A builder for [`CreateIdMappingWorkflowOutput`](crate::operation::create_id_mapping_workflow::CreateIdMappingWorkflowOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct CreateIdMappingWorkflowOutputBuilder {
    pub(crate) workflow_name: ::std::option::Option<::std::string::String>,
    pub(crate) workflow_arn: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) input_source_config: ::std::option::Option<::std::vec::Vec<crate::types::IdMappingWorkflowInputSource>>,
    pub(crate) output_source_config: ::std::option::Option<::std::vec::Vec<crate::types::IdMappingWorkflowOutputSource>>,
    pub(crate) id_mapping_techniques: ::std::option::Option<crate::types::IdMappingTechniques>,
    pub(crate) role_arn: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CreateIdMappingWorkflowOutputBuilder {
    /// <p>The name of the workflow.</p>
    /// This field is required.
    pub fn workflow_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.workflow_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the workflow.</p>
    pub fn set_workflow_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.workflow_name = input;
        self
    }
    /// <p>The name of the workflow.</p>
    pub fn get_workflow_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.workflow_name
    }
    /// <p>The ARN (Amazon Resource Name) that Entity Resolution generated for the <code>IDMappingWorkflow</code>.</p>
    /// This field is required.
    pub fn workflow_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.workflow_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN (Amazon Resource Name) that Entity Resolution generated for the <code>IDMappingWorkflow</code>.</p>
    pub fn set_workflow_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.workflow_arn = input;
        self
    }
    /// <p>The ARN (Amazon Resource Name) that Entity Resolution generated for the <code>IDMappingWorkflow</code>.</p>
    pub fn get_workflow_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.workflow_arn
    }
    /// <p>A description of the workflow.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A description of the workflow.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>A description of the workflow.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }
    /// Appends an item to `input_source_config`.
    ///
    /// To override the contents of this collection use [`set_input_source_config`](Self::set_input_source_config).
    ///
    /// <p>A list of <code>InputSource</code> objects, which have the fields <code>InputSourceARN</code> and <code>SchemaName</code>.</p>
    pub fn input_source_config(mut self, input: crate::types::IdMappingWorkflowInputSource) -> Self {
        let mut v = self.input_source_config.unwrap_or_default();
        v.push(input);
        self.input_source_config = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of <code>InputSource</code> objects, which have the fields <code>InputSourceARN</code> and <code>SchemaName</code>.</p>
    pub fn set_input_source_config(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::IdMappingWorkflowInputSource>>) -> Self {
        self.input_source_config = input;
        self
    }
    /// <p>A list of <code>InputSource</code> objects, which have the fields <code>InputSourceARN</code> and <code>SchemaName</code>.</p>
    pub fn get_input_source_config(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::IdMappingWorkflowInputSource>> {
        &self.input_source_config
    }
    /// Appends an item to `output_source_config`.
    ///
    /// To override the contents of this collection use [`set_output_source_config`](Self::set_output_source_config).
    ///
    /// <p>A list of <code>IdMappingWorkflowOutputSource</code> objects, each of which contains fields <code>outputS3Path</code> and <code>KMSArn</code>.</p>
    pub fn output_source_config(mut self, input: crate::types::IdMappingWorkflowOutputSource) -> Self {
        let mut v = self.output_source_config.unwrap_or_default();
        v.push(input);
        self.output_source_config = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of <code>IdMappingWorkflowOutputSource</code> objects, each of which contains fields <code>outputS3Path</code> and <code>KMSArn</code>.</p>
    pub fn set_output_source_config(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::IdMappingWorkflowOutputSource>>) -> Self {
        self.output_source_config = input;
        self
    }
    /// <p>A list of <code>IdMappingWorkflowOutputSource</code> objects, each of which contains fields <code>outputS3Path</code> and <code>KMSArn</code>.</p>
    pub fn get_output_source_config(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::IdMappingWorkflowOutputSource>> {
        &self.output_source_config
    }
    /// <p>An object which defines the ID mapping technique and any additional configurations.</p>
    /// This field is required.
    pub fn id_mapping_techniques(mut self, input: crate::types::IdMappingTechniques) -> Self {
        self.id_mapping_techniques = ::std::option::Option::Some(input);
        self
    }
    /// <p>An object which defines the ID mapping technique and any additional configurations.</p>
    pub fn set_id_mapping_techniques(mut self, input: ::std::option::Option<crate::types::IdMappingTechniques>) -> Self {
        self.id_mapping_techniques = input;
        self
    }
    /// <p>An object which defines the ID mapping technique and any additional configurations.</p>
    pub fn get_id_mapping_techniques(&self) -> &::std::option::Option<crate::types::IdMappingTechniques> {
        &self.id_mapping_techniques
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role. Entity Resolution assumes this role to create resources on your behalf as part of workflow execution.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.role_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role. Entity Resolution assumes this role to create resources on your behalf as part of workflow execution.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.role_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role. Entity Resolution assumes this role to create resources on your behalf as part of workflow execution.</p>
    pub fn get_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.role_arn
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`CreateIdMappingWorkflowOutput`](crate::operation::create_id_mapping_workflow::CreateIdMappingWorkflowOutput).
    /// This method will fail if any of the following fields are not set:
    /// - [`workflow_name`](crate::operation::create_id_mapping_workflow::builders::CreateIdMappingWorkflowOutputBuilder::workflow_name)
    /// - [`workflow_arn`](crate::operation::create_id_mapping_workflow::builders::CreateIdMappingWorkflowOutputBuilder::workflow_arn)
    /// - [`input_source_config`](crate::operation::create_id_mapping_workflow::builders::CreateIdMappingWorkflowOutputBuilder::input_source_config)
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_id_mapping_workflow::CreateIdMappingWorkflowOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_id_mapping_workflow::CreateIdMappingWorkflowOutput {
            workflow_name: self.workflow_name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "workflow_name",
                    "workflow_name was not specified but it is required when building CreateIdMappingWorkflowOutput",
                )
            })?,
            workflow_arn: self.workflow_arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "workflow_arn",
                    "workflow_arn was not specified but it is required when building CreateIdMappingWorkflowOutput",
                )
            })?,
            description: self.description,
            input_source_config: self.input_source_config.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "input_source_config",
                    "input_source_config was not specified but it is required when building CreateIdMappingWorkflowOutput",
                )
            })?,
            output_source_config: self.output_source_config,
            id_mapping_techniques: self.id_mapping_techniques,
            role_arn: self.role_arn.unwrap_or_default(),
            _request_id: self._request_id,
        })
    }
}
