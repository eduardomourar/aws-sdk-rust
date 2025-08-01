// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the response to a successful <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_GetRole.html">GetRole</a> request.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetRoleOutput {
    /// <p>A structure containing details about the IAM role.</p>
    pub role: ::std::option::Option<crate::types::Role>,
    _request_id: Option<String>,
}
impl GetRoleOutput {
    /// <p>A structure containing details about the IAM role.</p>
    pub fn role(&self) -> ::std::option::Option<&crate::types::Role> {
        self.role.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for GetRoleOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetRoleOutput {
    /// Creates a new builder-style object to manufacture [`GetRoleOutput`](crate::operation::get_role::GetRoleOutput).
    pub fn builder() -> crate::operation::get_role::builders::GetRoleOutputBuilder {
        crate::operation::get_role::builders::GetRoleOutputBuilder::default()
    }
}

/// A builder for [`GetRoleOutput`](crate::operation::get_role::GetRoleOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetRoleOutputBuilder {
    pub(crate) role: ::std::option::Option<crate::types::Role>,
    _request_id: Option<String>,
}
impl GetRoleOutputBuilder {
    /// <p>A structure containing details about the IAM role.</p>
    /// This field is required.
    pub fn role(mut self, input: crate::types::Role) -> Self {
        self.role = ::std::option::Option::Some(input);
        self
    }
    /// <p>A structure containing details about the IAM role.</p>
    pub fn set_role(mut self, input: ::std::option::Option<crate::types::Role>) -> Self {
        self.role = input;
        self
    }
    /// <p>A structure containing details about the IAM role.</p>
    pub fn get_role(&self) -> &::std::option::Option<crate::types::Role> {
        &self.role
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetRoleOutput`](crate::operation::get_role::GetRoleOutput).
    pub fn build(self) -> crate::operation::get_role::GetRoleOutput {
        crate::operation::get_role::GetRoleOutput {
            role: self.role,
            _request_id: self._request_id,
        }
    }
}
