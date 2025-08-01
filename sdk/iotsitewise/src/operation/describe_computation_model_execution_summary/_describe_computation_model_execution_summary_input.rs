// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeComputationModelExecutionSummaryInput {
    /// <p>The ID of the computation model.</p>
    pub computation_model_id: ::std::option::Option<::std::string::String>,
    /// <p>The type of the resolved resource.</p>
    pub resolve_to_resource_type: ::std::option::Option<crate::types::ResolveToResourceType>,
    /// <p>The ID of the resolved resource.</p>
    pub resolve_to_resource_id: ::std::option::Option<::std::string::String>,
}
impl DescribeComputationModelExecutionSummaryInput {
    /// <p>The ID of the computation model.</p>
    pub fn computation_model_id(&self) -> ::std::option::Option<&str> {
        self.computation_model_id.as_deref()
    }
    /// <p>The type of the resolved resource.</p>
    pub fn resolve_to_resource_type(&self) -> ::std::option::Option<&crate::types::ResolveToResourceType> {
        self.resolve_to_resource_type.as_ref()
    }
    /// <p>The ID of the resolved resource.</p>
    pub fn resolve_to_resource_id(&self) -> ::std::option::Option<&str> {
        self.resolve_to_resource_id.as_deref()
    }
}
impl DescribeComputationModelExecutionSummaryInput {
    /// Creates a new builder-style object to manufacture [`DescribeComputationModelExecutionSummaryInput`](crate::operation::describe_computation_model_execution_summary::DescribeComputationModelExecutionSummaryInput).
    pub fn builder() -> crate::operation::describe_computation_model_execution_summary::builders::DescribeComputationModelExecutionSummaryInputBuilder
    {
        crate::operation::describe_computation_model_execution_summary::builders::DescribeComputationModelExecutionSummaryInputBuilder::default()
    }
}

/// A builder for [`DescribeComputationModelExecutionSummaryInput`](crate::operation::describe_computation_model_execution_summary::DescribeComputationModelExecutionSummaryInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DescribeComputationModelExecutionSummaryInputBuilder {
    pub(crate) computation_model_id: ::std::option::Option<::std::string::String>,
    pub(crate) resolve_to_resource_type: ::std::option::Option<crate::types::ResolveToResourceType>,
    pub(crate) resolve_to_resource_id: ::std::option::Option<::std::string::String>,
}
impl DescribeComputationModelExecutionSummaryInputBuilder {
    /// <p>The ID of the computation model.</p>
    /// This field is required.
    pub fn computation_model_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.computation_model_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the computation model.</p>
    pub fn set_computation_model_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.computation_model_id = input;
        self
    }
    /// <p>The ID of the computation model.</p>
    pub fn get_computation_model_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.computation_model_id
    }
    /// <p>The type of the resolved resource.</p>
    pub fn resolve_to_resource_type(mut self, input: crate::types::ResolveToResourceType) -> Self {
        self.resolve_to_resource_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of the resolved resource.</p>
    pub fn set_resolve_to_resource_type(mut self, input: ::std::option::Option<crate::types::ResolveToResourceType>) -> Self {
        self.resolve_to_resource_type = input;
        self
    }
    /// <p>The type of the resolved resource.</p>
    pub fn get_resolve_to_resource_type(&self) -> &::std::option::Option<crate::types::ResolveToResourceType> {
        &self.resolve_to_resource_type
    }
    /// <p>The ID of the resolved resource.</p>
    pub fn resolve_to_resource_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resolve_to_resource_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the resolved resource.</p>
    pub fn set_resolve_to_resource_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resolve_to_resource_id = input;
        self
    }
    /// <p>The ID of the resolved resource.</p>
    pub fn get_resolve_to_resource_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.resolve_to_resource_id
    }
    /// Consumes the builder and constructs a [`DescribeComputationModelExecutionSummaryInput`](crate::operation::describe_computation_model_execution_summary::DescribeComputationModelExecutionSummaryInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_computation_model_execution_summary::DescribeComputationModelExecutionSummaryInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_computation_model_execution_summary::DescribeComputationModelExecutionSummaryInput {
                computation_model_id: self.computation_model_id,
                resolve_to_resource_type: self.resolve_to_resource_type,
                resolve_to_resource_id: self.resolve_to_resource_id,
            },
        )
    }
}
