// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Represents a request to delete an association between a tenant and a resource.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteTenantResourceAssociationInput {
    /// <p>The name of the tenant to remove the resource association from.</p>
    pub tenant_name: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the resource to remove from the tenant association.</p>
    pub resource_arn: ::std::option::Option<::std::string::String>,
}
impl DeleteTenantResourceAssociationInput {
    /// <p>The name of the tenant to remove the resource association from.</p>
    pub fn tenant_name(&self) -> ::std::option::Option<&str> {
        self.tenant_name.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the resource to remove from the tenant association.</p>
    pub fn resource_arn(&self) -> ::std::option::Option<&str> {
        self.resource_arn.as_deref()
    }
}
impl DeleteTenantResourceAssociationInput {
    /// Creates a new builder-style object to manufacture [`DeleteTenantResourceAssociationInput`](crate::operation::delete_tenant_resource_association::DeleteTenantResourceAssociationInput).
    pub fn builder() -> crate::operation::delete_tenant_resource_association::builders::DeleteTenantResourceAssociationInputBuilder {
        crate::operation::delete_tenant_resource_association::builders::DeleteTenantResourceAssociationInputBuilder::default()
    }
}

/// A builder for [`DeleteTenantResourceAssociationInput`](crate::operation::delete_tenant_resource_association::DeleteTenantResourceAssociationInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DeleteTenantResourceAssociationInputBuilder {
    pub(crate) tenant_name: ::std::option::Option<::std::string::String>,
    pub(crate) resource_arn: ::std::option::Option<::std::string::String>,
}
impl DeleteTenantResourceAssociationInputBuilder {
    /// <p>The name of the tenant to remove the resource association from.</p>
    /// This field is required.
    pub fn tenant_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.tenant_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the tenant to remove the resource association from.</p>
    pub fn set_tenant_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.tenant_name = input;
        self
    }
    /// <p>The name of the tenant to remove the resource association from.</p>
    pub fn get_tenant_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.tenant_name
    }
    /// <p>The Amazon Resource Name (ARN) of the resource to remove from the tenant association.</p>
    /// This field is required.
    pub fn resource_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the resource to remove from the tenant association.</p>
    pub fn set_resource_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the resource to remove from the tenant association.</p>
    pub fn get_resource_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.resource_arn
    }
    /// Consumes the builder and constructs a [`DeleteTenantResourceAssociationInput`](crate::operation::delete_tenant_resource_association::DeleteTenantResourceAssociationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_tenant_resource_association::DeleteTenantResourceAssociationInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::delete_tenant_resource_association::DeleteTenantResourceAssociationInput {
                tenant_name: self.tenant_name,
                resource_arn: self.resource_arn,
            },
        )
    }
}
