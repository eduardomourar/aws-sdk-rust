// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeResourcePoliciesInput {
    /// <p>The token for the next set of items to return. The token expires after 24 hours.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of resource policies to be displayed with one call of this API.</p>
    pub limit: ::std::option::Option<i32>,
    /// <p>The ARN of the CloudWatch Logs resource for which to query the resource policy.</p>
    pub resource_arn: ::std::option::Option<::std::string::String>,
    /// <p>Specifies the scope of the resource policy. Valid values are <code>ACCOUNT</code> or <code>RESOURCE</code>. When not specified, defaults to <code>ACCOUNT</code>.</p>
    pub policy_scope: ::std::option::Option<crate::types::PolicyScope>,
}
impl DescribeResourcePoliciesInput {
    /// <p>The token for the next set of items to return. The token expires after 24 hours.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum number of resource policies to be displayed with one call of this API.</p>
    pub fn limit(&self) -> ::std::option::Option<i32> {
        self.limit
    }
    /// <p>The ARN of the CloudWatch Logs resource for which to query the resource policy.</p>
    pub fn resource_arn(&self) -> ::std::option::Option<&str> {
        self.resource_arn.as_deref()
    }
    /// <p>Specifies the scope of the resource policy. Valid values are <code>ACCOUNT</code> or <code>RESOURCE</code>. When not specified, defaults to <code>ACCOUNT</code>.</p>
    pub fn policy_scope(&self) -> ::std::option::Option<&crate::types::PolicyScope> {
        self.policy_scope.as_ref()
    }
}
impl DescribeResourcePoliciesInput {
    /// Creates a new builder-style object to manufacture [`DescribeResourcePoliciesInput`](crate::operation::describe_resource_policies::DescribeResourcePoliciesInput).
    pub fn builder() -> crate::operation::describe_resource_policies::builders::DescribeResourcePoliciesInputBuilder {
        crate::operation::describe_resource_policies::builders::DescribeResourcePoliciesInputBuilder::default()
    }
}

/// A builder for [`DescribeResourcePoliciesInput`](crate::operation::describe_resource_policies::DescribeResourcePoliciesInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DescribeResourcePoliciesInputBuilder {
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) limit: ::std::option::Option<i32>,
    pub(crate) resource_arn: ::std::option::Option<::std::string::String>,
    pub(crate) policy_scope: ::std::option::Option<crate::types::PolicyScope>,
}
impl DescribeResourcePoliciesInputBuilder {
    /// <p>The token for the next set of items to return. The token expires after 24 hours.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token for the next set of items to return. The token expires after 24 hours.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The token for the next set of items to return. The token expires after 24 hours.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    /// <p>The maximum number of resource policies to be displayed with one call of this API.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.limit = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of resource policies to be displayed with one call of this API.</p>
    pub fn set_limit(mut self, input: ::std::option::Option<i32>) -> Self {
        self.limit = input;
        self
    }
    /// <p>The maximum number of resource policies to be displayed with one call of this API.</p>
    pub fn get_limit(&self) -> &::std::option::Option<i32> {
        &self.limit
    }
    /// <p>The ARN of the CloudWatch Logs resource for which to query the resource policy.</p>
    pub fn resource_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the CloudWatch Logs resource for which to query the resource policy.</p>
    pub fn set_resource_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource_arn = input;
        self
    }
    /// <p>The ARN of the CloudWatch Logs resource for which to query the resource policy.</p>
    pub fn get_resource_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.resource_arn
    }
    /// <p>Specifies the scope of the resource policy. Valid values are <code>ACCOUNT</code> or <code>RESOURCE</code>. When not specified, defaults to <code>ACCOUNT</code>.</p>
    pub fn policy_scope(mut self, input: crate::types::PolicyScope) -> Self {
        self.policy_scope = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the scope of the resource policy. Valid values are <code>ACCOUNT</code> or <code>RESOURCE</code>. When not specified, defaults to <code>ACCOUNT</code>.</p>
    pub fn set_policy_scope(mut self, input: ::std::option::Option<crate::types::PolicyScope>) -> Self {
        self.policy_scope = input;
        self
    }
    /// <p>Specifies the scope of the resource policy. Valid values are <code>ACCOUNT</code> or <code>RESOURCE</code>. When not specified, defaults to <code>ACCOUNT</code>.</p>
    pub fn get_policy_scope(&self) -> &::std::option::Option<crate::types::PolicyScope> {
        &self.policy_scope
    }
    /// Consumes the builder and constructs a [`DescribeResourcePoliciesInput`](crate::operation::describe_resource_policies::DescribeResourcePoliciesInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_resource_policies::DescribeResourcePoliciesInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_resource_policies::DescribeResourcePoliciesInput {
            next_token: self.next_token,
            limit: self.limit,
            resource_arn: self.resource_arn,
            policy_scope: self.policy_scope,
        })
    }
}
