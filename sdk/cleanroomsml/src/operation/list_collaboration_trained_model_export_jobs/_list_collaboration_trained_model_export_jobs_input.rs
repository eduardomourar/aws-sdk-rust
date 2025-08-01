// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListCollaborationTrainedModelExportJobsInput {
    /// <p>The token value retrieved from a previous call to access the next page of results.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The maximum size of the results that is returned per call.</p>
    pub max_results: ::std::option::Option<i32>,
    /// <p>The collaboration ID of the collaboration that contains the trained model export jobs that you are interested in.</p>
    pub collaboration_identifier: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the trained model that was used to create the export jobs that you are interested in.</p>
    pub trained_model_arn: ::std::option::Option<::std::string::String>,
    /// <p>The version identifier of the trained model to filter export jobs by. When specified, only export jobs for this specific version of the trained model are returned.</p>
    pub trained_model_version_identifier: ::std::option::Option<::std::string::String>,
}
impl ListCollaborationTrainedModelExportJobsInput {
    /// <p>The token value retrieved from a previous call to access the next page of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum size of the results that is returned per call.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>The collaboration ID of the collaboration that contains the trained model export jobs that you are interested in.</p>
    pub fn collaboration_identifier(&self) -> ::std::option::Option<&str> {
        self.collaboration_identifier.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the trained model that was used to create the export jobs that you are interested in.</p>
    pub fn trained_model_arn(&self) -> ::std::option::Option<&str> {
        self.trained_model_arn.as_deref()
    }
    /// <p>The version identifier of the trained model to filter export jobs by. When specified, only export jobs for this specific version of the trained model are returned.</p>
    pub fn trained_model_version_identifier(&self) -> ::std::option::Option<&str> {
        self.trained_model_version_identifier.as_deref()
    }
}
impl ListCollaborationTrainedModelExportJobsInput {
    /// Creates a new builder-style object to manufacture [`ListCollaborationTrainedModelExportJobsInput`](crate::operation::list_collaboration_trained_model_export_jobs::ListCollaborationTrainedModelExportJobsInput).
    pub fn builder() -> crate::operation::list_collaboration_trained_model_export_jobs::builders::ListCollaborationTrainedModelExportJobsInputBuilder
    {
        crate::operation::list_collaboration_trained_model_export_jobs::builders::ListCollaborationTrainedModelExportJobsInputBuilder::default()
    }
}

/// A builder for [`ListCollaborationTrainedModelExportJobsInput`](crate::operation::list_collaboration_trained_model_export_jobs::ListCollaborationTrainedModelExportJobsInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ListCollaborationTrainedModelExportJobsInputBuilder {
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) collaboration_identifier: ::std::option::Option<::std::string::String>,
    pub(crate) trained_model_arn: ::std::option::Option<::std::string::String>,
    pub(crate) trained_model_version_identifier: ::std::option::Option<::std::string::String>,
}
impl ListCollaborationTrainedModelExportJobsInputBuilder {
    /// <p>The token value retrieved from a previous call to access the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token value retrieved from a previous call to access the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The token value retrieved from a previous call to access the next page of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    /// <p>The maximum size of the results that is returned per call.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum size of the results that is returned per call.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The maximum size of the results that is returned per call.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        &self.max_results
    }
    /// <p>The collaboration ID of the collaboration that contains the trained model export jobs that you are interested in.</p>
    /// This field is required.
    pub fn collaboration_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.collaboration_identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The collaboration ID of the collaboration that contains the trained model export jobs that you are interested in.</p>
    pub fn set_collaboration_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.collaboration_identifier = input;
        self
    }
    /// <p>The collaboration ID of the collaboration that contains the trained model export jobs that you are interested in.</p>
    pub fn get_collaboration_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.collaboration_identifier
    }
    /// <p>The Amazon Resource Name (ARN) of the trained model that was used to create the export jobs that you are interested in.</p>
    /// This field is required.
    pub fn trained_model_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.trained_model_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the trained model that was used to create the export jobs that you are interested in.</p>
    pub fn set_trained_model_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.trained_model_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the trained model that was used to create the export jobs that you are interested in.</p>
    pub fn get_trained_model_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.trained_model_arn
    }
    /// <p>The version identifier of the trained model to filter export jobs by. When specified, only export jobs for this specific version of the trained model are returned.</p>
    pub fn trained_model_version_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.trained_model_version_identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The version identifier of the trained model to filter export jobs by. When specified, only export jobs for this specific version of the trained model are returned.</p>
    pub fn set_trained_model_version_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.trained_model_version_identifier = input;
        self
    }
    /// <p>The version identifier of the trained model to filter export jobs by. When specified, only export jobs for this specific version of the trained model are returned.</p>
    pub fn get_trained_model_version_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.trained_model_version_identifier
    }
    /// Consumes the builder and constructs a [`ListCollaborationTrainedModelExportJobsInput`](crate::operation::list_collaboration_trained_model_export_jobs::ListCollaborationTrainedModelExportJobsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_collaboration_trained_model_export_jobs::ListCollaborationTrainedModelExportJobsInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::list_collaboration_trained_model_export_jobs::ListCollaborationTrainedModelExportJobsInput {
                next_token: self.next_token,
                max_results: self.max_results,
                collaboration_identifier: self.collaboration_identifier,
                trained_model_arn: self.trained_model_arn,
                trained_model_version_identifier: self.trained_model_version_identifier,
            },
        )
    }
}
