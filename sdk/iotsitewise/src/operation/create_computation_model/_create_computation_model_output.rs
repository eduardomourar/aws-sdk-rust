// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateComputationModelOutput {
    /// <p>The ID of the computation model.</p>
    pub computation_model_id: ::std::string::String,
    /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> of the computation model, which has the following format.</p>
    /// <p><code>arn:${Partition}:iotsitewise:${Region}:${Account}:computation-model/${ComputationModelId}</code></p>
    pub computation_model_arn: ::std::string::String,
    /// <p>The status of the computation model, containing a state (CREATING after successfully calling this operation) and any error messages.</p>
    pub computation_model_status: ::std::option::Option<crate::types::ComputationModelStatus>,
    _request_id: Option<String>,
}
impl CreateComputationModelOutput {
    /// <p>The ID of the computation model.</p>
    pub fn computation_model_id(&self) -> &str {
        use std::ops::Deref;
        self.computation_model_id.deref()
    }
    /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> of the computation model, which has the following format.</p>
    /// <p><code>arn:${Partition}:iotsitewise:${Region}:${Account}:computation-model/${ComputationModelId}</code></p>
    pub fn computation_model_arn(&self) -> &str {
        use std::ops::Deref;
        self.computation_model_arn.deref()
    }
    /// <p>The status of the computation model, containing a state (CREATING after successfully calling this operation) and any error messages.</p>
    pub fn computation_model_status(&self) -> ::std::option::Option<&crate::types::ComputationModelStatus> {
        self.computation_model_status.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for CreateComputationModelOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateComputationModelOutput {
    /// Creates a new builder-style object to manufacture [`CreateComputationModelOutput`](crate::operation::create_computation_model::CreateComputationModelOutput).
    pub fn builder() -> crate::operation::create_computation_model::builders::CreateComputationModelOutputBuilder {
        crate::operation::create_computation_model::builders::CreateComputationModelOutputBuilder::default()
    }
}

/// A builder for [`CreateComputationModelOutput`](crate::operation::create_computation_model::CreateComputationModelOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct CreateComputationModelOutputBuilder {
    pub(crate) computation_model_id: ::std::option::Option<::std::string::String>,
    pub(crate) computation_model_arn: ::std::option::Option<::std::string::String>,
    pub(crate) computation_model_status: ::std::option::Option<crate::types::ComputationModelStatus>,
    _request_id: Option<String>,
}
impl CreateComputationModelOutputBuilder {
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
    /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> of the computation model, which has the following format.</p>
    /// <p><code>arn:${Partition}:iotsitewise:${Region}:${Account}:computation-model/${ComputationModelId}</code></p>
    /// This field is required.
    pub fn computation_model_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.computation_model_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> of the computation model, which has the following format.</p>
    /// <p><code>arn:${Partition}:iotsitewise:${Region}:${Account}:computation-model/${ComputationModelId}</code></p>
    pub fn set_computation_model_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.computation_model_arn = input;
        self
    }
    /// <p>The <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">ARN</a> of the computation model, which has the following format.</p>
    /// <p><code>arn:${Partition}:iotsitewise:${Region}:${Account}:computation-model/${ComputationModelId}</code></p>
    pub fn get_computation_model_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.computation_model_arn
    }
    /// <p>The status of the computation model, containing a state (CREATING after successfully calling this operation) and any error messages.</p>
    /// This field is required.
    pub fn computation_model_status(mut self, input: crate::types::ComputationModelStatus) -> Self {
        self.computation_model_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the computation model, containing a state (CREATING after successfully calling this operation) and any error messages.</p>
    pub fn set_computation_model_status(mut self, input: ::std::option::Option<crate::types::ComputationModelStatus>) -> Self {
        self.computation_model_status = input;
        self
    }
    /// <p>The status of the computation model, containing a state (CREATING after successfully calling this operation) and any error messages.</p>
    pub fn get_computation_model_status(&self) -> &::std::option::Option<crate::types::ComputationModelStatus> {
        &self.computation_model_status
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`CreateComputationModelOutput`](crate::operation::create_computation_model::CreateComputationModelOutput).
    /// This method will fail if any of the following fields are not set:
    /// - [`computation_model_id`](crate::operation::create_computation_model::builders::CreateComputationModelOutputBuilder::computation_model_id)
    /// - [`computation_model_arn`](crate::operation::create_computation_model::builders::CreateComputationModelOutputBuilder::computation_model_arn)
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_computation_model::CreateComputationModelOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_computation_model::CreateComputationModelOutput {
            computation_model_id: self.computation_model_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "computation_model_id",
                    "computation_model_id was not specified but it is required when building CreateComputationModelOutput",
                )
            })?,
            computation_model_arn: self.computation_model_arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "computation_model_arn",
                    "computation_model_arn was not specified but it is required when building CreateComputationModelOutput",
                )
            })?,
            computation_model_status: self.computation_model_status,
            _request_id: self._request_id,
        })
    }
}
