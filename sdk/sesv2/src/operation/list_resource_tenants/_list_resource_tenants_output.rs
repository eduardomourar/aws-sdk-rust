// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about tenants associated with a specific resource.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListResourceTenantsOutput {
    /// <p>An array that contains information about each tenant associated with the resource.</p>
    pub resource_tenants: ::std::option::Option<::std::vec::Vec<crate::types::ResourceTenantMetadata>>,
    /// <p>A token that indicates that there are additional tenants to list. To view additional tenants, issue another request to <code>ListResourceTenants</code>, and pass this token in the <code>NextToken</code> parameter.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListResourceTenantsOutput {
    /// <p>An array that contains information about each tenant associated with the resource.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.resource_tenants.is_none()`.
    pub fn resource_tenants(&self) -> &[crate::types::ResourceTenantMetadata] {
        self.resource_tenants.as_deref().unwrap_or_default()
    }
    /// <p>A token that indicates that there are additional tenants to list. To view additional tenants, issue another request to <code>ListResourceTenants</code>, and pass this token in the <code>NextToken</code> parameter.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for ListResourceTenantsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListResourceTenantsOutput {
    /// Creates a new builder-style object to manufacture [`ListResourceTenantsOutput`](crate::operation::list_resource_tenants::ListResourceTenantsOutput).
    pub fn builder() -> crate::operation::list_resource_tenants::builders::ListResourceTenantsOutputBuilder {
        crate::operation::list_resource_tenants::builders::ListResourceTenantsOutputBuilder::default()
    }
}

/// A builder for [`ListResourceTenantsOutput`](crate::operation::list_resource_tenants::ListResourceTenantsOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ListResourceTenantsOutputBuilder {
    pub(crate) resource_tenants: ::std::option::Option<::std::vec::Vec<crate::types::ResourceTenantMetadata>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl ListResourceTenantsOutputBuilder {
    /// Appends an item to `resource_tenants`.
    ///
    /// To override the contents of this collection use [`set_resource_tenants`](Self::set_resource_tenants).
    ///
    /// <p>An array that contains information about each tenant associated with the resource.</p>
    pub fn resource_tenants(mut self, input: crate::types::ResourceTenantMetadata) -> Self {
        let mut v = self.resource_tenants.unwrap_or_default();
        v.push(input);
        self.resource_tenants = ::std::option::Option::Some(v);
        self
    }
    /// <p>An array that contains information about each tenant associated with the resource.</p>
    pub fn set_resource_tenants(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ResourceTenantMetadata>>) -> Self {
        self.resource_tenants = input;
        self
    }
    /// <p>An array that contains information about each tenant associated with the resource.</p>
    pub fn get_resource_tenants(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ResourceTenantMetadata>> {
        &self.resource_tenants
    }
    /// <p>A token that indicates that there are additional tenants to list. To view additional tenants, issue another request to <code>ListResourceTenants</code>, and pass this token in the <code>NextToken</code> parameter.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A token that indicates that there are additional tenants to list. To view additional tenants, issue another request to <code>ListResourceTenants</code>, and pass this token in the <code>NextToken</code> parameter.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>A token that indicates that there are additional tenants to list. To view additional tenants, issue another request to <code>ListResourceTenants</code>, and pass this token in the <code>NextToken</code> parameter.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListResourceTenantsOutput`](crate::operation::list_resource_tenants::ListResourceTenantsOutput).
    pub fn build(self) -> crate::operation::list_resource_tenants::ListResourceTenantsOutput {
        crate::operation::list_resource_tenants::ListResourceTenantsOutput {
            resource_tenants: self.resource_tenants,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
