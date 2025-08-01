// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SubmitServiceJobOutput {
    /// <p>The Amazon Resource Name (ARN) for the service job.</p>
    pub job_arn: ::std::option::Option<::std::string::String>,
    /// <p>The name of the service job.</p>
    pub job_name: ::std::option::Option<::std::string::String>,
    /// <p>The unique identifier for the service job.</p>
    pub job_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl SubmitServiceJobOutput {
    /// <p>The Amazon Resource Name (ARN) for the service job.</p>
    pub fn job_arn(&self) -> ::std::option::Option<&str> {
        self.job_arn.as_deref()
    }
    /// <p>The name of the service job.</p>
    pub fn job_name(&self) -> ::std::option::Option<&str> {
        self.job_name.as_deref()
    }
    /// <p>The unique identifier for the service job.</p>
    pub fn job_id(&self) -> ::std::option::Option<&str> {
        self.job_id.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for SubmitServiceJobOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl SubmitServiceJobOutput {
    /// Creates a new builder-style object to manufacture [`SubmitServiceJobOutput`](crate::operation::submit_service_job::SubmitServiceJobOutput).
    pub fn builder() -> crate::operation::submit_service_job::builders::SubmitServiceJobOutputBuilder {
        crate::operation::submit_service_job::builders::SubmitServiceJobOutputBuilder::default()
    }
}

/// A builder for [`SubmitServiceJobOutput`](crate::operation::submit_service_job::SubmitServiceJobOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct SubmitServiceJobOutputBuilder {
    pub(crate) job_arn: ::std::option::Option<::std::string::String>,
    pub(crate) job_name: ::std::option::Option<::std::string::String>,
    pub(crate) job_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl SubmitServiceJobOutputBuilder {
    /// <p>The Amazon Resource Name (ARN) for the service job.</p>
    pub fn job_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.job_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) for the service job.</p>
    pub fn set_job_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.job_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) for the service job.</p>
    pub fn get_job_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.job_arn
    }
    /// <p>The name of the service job.</p>
    /// This field is required.
    pub fn job_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.job_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the service job.</p>
    pub fn set_job_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.job_name = input;
        self
    }
    /// <p>The name of the service job.</p>
    pub fn get_job_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.job_name
    }
    /// <p>The unique identifier for the service job.</p>
    /// This field is required.
    pub fn job_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.job_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier for the service job.</p>
    pub fn set_job_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.job_id = input;
        self
    }
    /// <p>The unique identifier for the service job.</p>
    pub fn get_job_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.job_id
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`SubmitServiceJobOutput`](crate::operation::submit_service_job::SubmitServiceJobOutput).
    pub fn build(self) -> crate::operation::submit_service_job::SubmitServiceJobOutput {
        crate::operation::submit_service_job::SubmitServiceJobOutput {
            job_arn: self.job_arn,
            job_name: self.job_name,
            job_id: self.job_id,
            _request_id: self._request_id,
        }
    }
}
