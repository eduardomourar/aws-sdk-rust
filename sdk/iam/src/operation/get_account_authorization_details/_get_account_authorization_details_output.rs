// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the response to a successful <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_GetAccountAuthorizationDetails.html">GetAccountAuthorizationDetails</a> request.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetAccountAuthorizationDetailsOutput {
    /// <p>A list containing information about IAM users.</p>
    pub user_detail_list: ::std::option::Option<::std::vec::Vec<crate::types::UserDetail>>,
    /// <p>A list containing information about IAM groups.</p>
    pub group_detail_list: ::std::option::Option<::std::vec::Vec<crate::types::GroupDetail>>,
    /// <p>A list containing information about IAM roles.</p>
    pub role_detail_list: ::std::option::Option<::std::vec::Vec<crate::types::RoleDetail>>,
    /// <p>A list containing information about managed policies.</p>
    pub policies: ::std::option::Option<::std::vec::Vec<crate::types::ManagedPolicyDetail>>,
    /// <p>A flag that indicates whether there are more items to return. If your results were truncated, you can make a subsequent pagination request using the <code>Marker</code> request parameter to retrieve more items. Note that IAM might return fewer than the <code>MaxItems</code> number of results even when there are more results available. We recommend that you check <code>IsTruncated</code> after every call to ensure that you receive all your results.</p>
    pub is_truncated: bool,
    /// <p>When <code>IsTruncated</code> is <code>true</code>, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent pagination request.</p>
    pub marker: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetAccountAuthorizationDetailsOutput {
    /// <p>A list containing information about IAM users.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.user_detail_list.is_none()`.
    pub fn user_detail_list(&self) -> &[crate::types::UserDetail] {
        self.user_detail_list.as_deref().unwrap_or_default()
    }
    /// <p>A list containing information about IAM groups.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.group_detail_list.is_none()`.
    pub fn group_detail_list(&self) -> &[crate::types::GroupDetail] {
        self.group_detail_list.as_deref().unwrap_or_default()
    }
    /// <p>A list containing information about IAM roles.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.role_detail_list.is_none()`.
    pub fn role_detail_list(&self) -> &[crate::types::RoleDetail] {
        self.role_detail_list.as_deref().unwrap_or_default()
    }
    /// <p>A list containing information about managed policies.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.policies.is_none()`.
    pub fn policies(&self) -> &[crate::types::ManagedPolicyDetail] {
        self.policies.as_deref().unwrap_or_default()
    }
    /// <p>A flag that indicates whether there are more items to return. If your results were truncated, you can make a subsequent pagination request using the <code>Marker</code> request parameter to retrieve more items. Note that IAM might return fewer than the <code>MaxItems</code> number of results even when there are more results available. We recommend that you check <code>IsTruncated</code> after every call to ensure that you receive all your results.</p>
    pub fn is_truncated(&self) -> bool {
        self.is_truncated
    }
    /// <p>When <code>IsTruncated</code> is <code>true</code>, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent pagination request.</p>
    pub fn marker(&self) -> ::std::option::Option<&str> {
        self.marker.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for GetAccountAuthorizationDetailsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetAccountAuthorizationDetailsOutput {
    /// Creates a new builder-style object to manufacture [`GetAccountAuthorizationDetailsOutput`](crate::operation::get_account_authorization_details::GetAccountAuthorizationDetailsOutput).
    pub fn builder() -> crate::operation::get_account_authorization_details::builders::GetAccountAuthorizationDetailsOutputBuilder {
        crate::operation::get_account_authorization_details::builders::GetAccountAuthorizationDetailsOutputBuilder::default()
    }
}

/// A builder for [`GetAccountAuthorizationDetailsOutput`](crate::operation::get_account_authorization_details::GetAccountAuthorizationDetailsOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetAccountAuthorizationDetailsOutputBuilder {
    pub(crate) user_detail_list: ::std::option::Option<::std::vec::Vec<crate::types::UserDetail>>,
    pub(crate) group_detail_list: ::std::option::Option<::std::vec::Vec<crate::types::GroupDetail>>,
    pub(crate) role_detail_list: ::std::option::Option<::std::vec::Vec<crate::types::RoleDetail>>,
    pub(crate) policies: ::std::option::Option<::std::vec::Vec<crate::types::ManagedPolicyDetail>>,
    pub(crate) is_truncated: ::std::option::Option<bool>,
    pub(crate) marker: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetAccountAuthorizationDetailsOutputBuilder {
    /// Appends an item to `user_detail_list`.
    ///
    /// To override the contents of this collection use [`set_user_detail_list`](Self::set_user_detail_list).
    ///
    /// <p>A list containing information about IAM users.</p>
    pub fn user_detail_list(mut self, input: crate::types::UserDetail) -> Self {
        let mut v = self.user_detail_list.unwrap_or_default();
        v.push(input);
        self.user_detail_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list containing information about IAM users.</p>
    pub fn set_user_detail_list(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::UserDetail>>) -> Self {
        self.user_detail_list = input;
        self
    }
    /// <p>A list containing information about IAM users.</p>
    pub fn get_user_detail_list(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::UserDetail>> {
        &self.user_detail_list
    }
    /// Appends an item to `group_detail_list`.
    ///
    /// To override the contents of this collection use [`set_group_detail_list`](Self::set_group_detail_list).
    ///
    /// <p>A list containing information about IAM groups.</p>
    pub fn group_detail_list(mut self, input: crate::types::GroupDetail) -> Self {
        let mut v = self.group_detail_list.unwrap_or_default();
        v.push(input);
        self.group_detail_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list containing information about IAM groups.</p>
    pub fn set_group_detail_list(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::GroupDetail>>) -> Self {
        self.group_detail_list = input;
        self
    }
    /// <p>A list containing information about IAM groups.</p>
    pub fn get_group_detail_list(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::GroupDetail>> {
        &self.group_detail_list
    }
    /// Appends an item to `role_detail_list`.
    ///
    /// To override the contents of this collection use [`set_role_detail_list`](Self::set_role_detail_list).
    ///
    /// <p>A list containing information about IAM roles.</p>
    pub fn role_detail_list(mut self, input: crate::types::RoleDetail) -> Self {
        let mut v = self.role_detail_list.unwrap_or_default();
        v.push(input);
        self.role_detail_list = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list containing information about IAM roles.</p>
    pub fn set_role_detail_list(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::RoleDetail>>) -> Self {
        self.role_detail_list = input;
        self
    }
    /// <p>A list containing information about IAM roles.</p>
    pub fn get_role_detail_list(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::RoleDetail>> {
        &self.role_detail_list
    }
    /// Appends an item to `policies`.
    ///
    /// To override the contents of this collection use [`set_policies`](Self::set_policies).
    ///
    /// <p>A list containing information about managed policies.</p>
    pub fn policies(mut self, input: crate::types::ManagedPolicyDetail) -> Self {
        let mut v = self.policies.unwrap_or_default();
        v.push(input);
        self.policies = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list containing information about managed policies.</p>
    pub fn set_policies(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ManagedPolicyDetail>>) -> Self {
        self.policies = input;
        self
    }
    /// <p>A list containing information about managed policies.</p>
    pub fn get_policies(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ManagedPolicyDetail>> {
        &self.policies
    }
    /// <p>A flag that indicates whether there are more items to return. If your results were truncated, you can make a subsequent pagination request using the <code>Marker</code> request parameter to retrieve more items. Note that IAM might return fewer than the <code>MaxItems</code> number of results even when there are more results available. We recommend that you check <code>IsTruncated</code> after every call to ensure that you receive all your results.</p>
    pub fn is_truncated(mut self, input: bool) -> Self {
        self.is_truncated = ::std::option::Option::Some(input);
        self
    }
    /// <p>A flag that indicates whether there are more items to return. If your results were truncated, you can make a subsequent pagination request using the <code>Marker</code> request parameter to retrieve more items. Note that IAM might return fewer than the <code>MaxItems</code> number of results even when there are more results available. We recommend that you check <code>IsTruncated</code> after every call to ensure that you receive all your results.</p>
    pub fn set_is_truncated(mut self, input: ::std::option::Option<bool>) -> Self {
        self.is_truncated = input;
        self
    }
    /// <p>A flag that indicates whether there are more items to return. If your results were truncated, you can make a subsequent pagination request using the <code>Marker</code> request parameter to retrieve more items. Note that IAM might return fewer than the <code>MaxItems</code> number of results even when there are more results available. We recommend that you check <code>IsTruncated</code> after every call to ensure that you receive all your results.</p>
    pub fn get_is_truncated(&self) -> &::std::option::Option<bool> {
        &self.is_truncated
    }
    /// <p>When <code>IsTruncated</code> is <code>true</code>, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent pagination request.</p>
    pub fn marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.marker = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>When <code>IsTruncated</code> is <code>true</code>, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent pagination request.</p>
    pub fn set_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.marker = input;
        self
    }
    /// <p>When <code>IsTruncated</code> is <code>true</code>, this element is present and contains the value to use for the <code>Marker</code> parameter in a subsequent pagination request.</p>
    pub fn get_marker(&self) -> &::std::option::Option<::std::string::String> {
        &self.marker
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetAccountAuthorizationDetailsOutput`](crate::operation::get_account_authorization_details::GetAccountAuthorizationDetailsOutput).
    pub fn build(self) -> crate::operation::get_account_authorization_details::GetAccountAuthorizationDetailsOutput {
        crate::operation::get_account_authorization_details::GetAccountAuthorizationDetailsOutput {
            user_detail_list: self.user_detail_list,
            group_detail_list: self.group_detail_list,
            role_detail_list: self.role_detail_list,
            policies: self.policies,
            is_truncated: self.is_truncated.unwrap_or_default(),
            marker: self.marker,
            _request_id: self._request_id,
        }
    }
}
