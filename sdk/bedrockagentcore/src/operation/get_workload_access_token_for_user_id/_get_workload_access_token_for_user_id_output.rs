// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct GetWorkloadAccessTokenForUserIdOutput {
    /// <p>The workload access token of the named workload.</p>
    pub workload_access_token: ::std::string::String,
    _request_id: Option<String>,
}
impl GetWorkloadAccessTokenForUserIdOutput {
    /// <p>The workload access token of the named workload.</p>
    pub fn workload_access_token(&self) -> &str {
        use std::ops::Deref;
        self.workload_access_token.deref()
    }
}
impl ::std::fmt::Debug for GetWorkloadAccessTokenForUserIdOutput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("GetWorkloadAccessTokenForUserIdOutput");
        formatter.field("workload_access_token", &"*** Sensitive Data Redacted ***");
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
impl ::aws_types::request_id::RequestId for GetWorkloadAccessTokenForUserIdOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetWorkloadAccessTokenForUserIdOutput {
    /// Creates a new builder-style object to manufacture [`GetWorkloadAccessTokenForUserIdOutput`](crate::operation::get_workload_access_token_for_user_id::GetWorkloadAccessTokenForUserIdOutput).
    pub fn builder() -> crate::operation::get_workload_access_token_for_user_id::builders::GetWorkloadAccessTokenForUserIdOutputBuilder {
        crate::operation::get_workload_access_token_for_user_id::builders::GetWorkloadAccessTokenForUserIdOutputBuilder::default()
    }
}

/// A builder for [`GetWorkloadAccessTokenForUserIdOutput`](crate::operation::get_workload_access_token_for_user_id::GetWorkloadAccessTokenForUserIdOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
#[non_exhaustive]
pub struct GetWorkloadAccessTokenForUserIdOutputBuilder {
    pub(crate) workload_access_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetWorkloadAccessTokenForUserIdOutputBuilder {
    /// <p>The workload access token of the named workload.</p>
    /// This field is required.
    pub fn workload_access_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.workload_access_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The workload access token of the named workload.</p>
    pub fn set_workload_access_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.workload_access_token = input;
        self
    }
    /// <p>The workload access token of the named workload.</p>
    pub fn get_workload_access_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.workload_access_token
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetWorkloadAccessTokenForUserIdOutput`](crate::operation::get_workload_access_token_for_user_id::GetWorkloadAccessTokenForUserIdOutput).
    /// This method will fail if any of the following fields are not set:
    /// - [`workload_access_token`](crate::operation::get_workload_access_token_for_user_id::builders::GetWorkloadAccessTokenForUserIdOutputBuilder::workload_access_token)
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_workload_access_token_for_user_id::GetWorkloadAccessTokenForUserIdOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_workload_access_token_for_user_id::GetWorkloadAccessTokenForUserIdOutput {
                workload_access_token: self.workload_access_token.ok_or_else(|| {
                    ::aws_smithy_types::error::operation::BuildError::missing_field(
                        "workload_access_token",
                        "workload_access_token was not specified but it is required when building GetWorkloadAccessTokenForUserIdOutput",
                    )
                })?,
                _request_id: self._request_id,
            },
        )
    }
}
impl ::std::fmt::Debug for GetWorkloadAccessTokenForUserIdOutputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("GetWorkloadAccessTokenForUserIdOutputBuilder");
        formatter.field("workload_access_token", &"*** Sensitive Data Redacted ***");
        formatter.field("_request_id", &self._request_id);
        formatter.finish()
    }
}
