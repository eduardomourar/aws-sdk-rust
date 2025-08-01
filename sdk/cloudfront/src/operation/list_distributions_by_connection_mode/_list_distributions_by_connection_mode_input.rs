// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListDistributionsByConnectionModeInput {
    /// <p>The marker for the next set of distributions to retrieve.</p>
    pub marker: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of distributions to return.</p>
    pub max_items: ::std::option::Option<i32>,
    /// <p>This field specifies whether the connection mode is through a standard distribution (direct) or a multi-tenant distribution with distribution tenants (tenant-only).</p>
    pub connection_mode: ::std::option::Option<crate::types::ConnectionMode>,
}
impl ListDistributionsByConnectionModeInput {
    /// <p>The marker for the next set of distributions to retrieve.</p>
    pub fn marker(&self) -> ::std::option::Option<&str> {
        self.marker.as_deref()
    }
    /// <p>The maximum number of distributions to return.</p>
    pub fn max_items(&self) -> ::std::option::Option<i32> {
        self.max_items
    }
    /// <p>This field specifies whether the connection mode is through a standard distribution (direct) or a multi-tenant distribution with distribution tenants (tenant-only).</p>
    pub fn connection_mode(&self) -> ::std::option::Option<&crate::types::ConnectionMode> {
        self.connection_mode.as_ref()
    }
}
impl ListDistributionsByConnectionModeInput {
    /// Creates a new builder-style object to manufacture [`ListDistributionsByConnectionModeInput`](crate::operation::list_distributions_by_connection_mode::ListDistributionsByConnectionModeInput).
    pub fn builder() -> crate::operation::list_distributions_by_connection_mode::builders::ListDistributionsByConnectionModeInputBuilder {
        crate::operation::list_distributions_by_connection_mode::builders::ListDistributionsByConnectionModeInputBuilder::default()
    }
}

/// A builder for [`ListDistributionsByConnectionModeInput`](crate::operation::list_distributions_by_connection_mode::ListDistributionsByConnectionModeInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ListDistributionsByConnectionModeInputBuilder {
    pub(crate) marker: ::std::option::Option<::std::string::String>,
    pub(crate) max_items: ::std::option::Option<i32>,
    pub(crate) connection_mode: ::std::option::Option<crate::types::ConnectionMode>,
}
impl ListDistributionsByConnectionModeInputBuilder {
    /// <p>The marker for the next set of distributions to retrieve.</p>
    pub fn marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.marker = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The marker for the next set of distributions to retrieve.</p>
    pub fn set_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.marker = input;
        self
    }
    /// <p>The marker for the next set of distributions to retrieve.</p>
    pub fn get_marker(&self) -> &::std::option::Option<::std::string::String> {
        &self.marker
    }
    /// <p>The maximum number of distributions to return.</p>
    pub fn max_items(mut self, input: i32) -> Self {
        self.max_items = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of distributions to return.</p>
    pub fn set_max_items(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_items = input;
        self
    }
    /// <p>The maximum number of distributions to return.</p>
    pub fn get_max_items(&self) -> &::std::option::Option<i32> {
        &self.max_items
    }
    /// <p>This field specifies whether the connection mode is through a standard distribution (direct) or a multi-tenant distribution with distribution tenants (tenant-only).</p>
    /// This field is required.
    pub fn connection_mode(mut self, input: crate::types::ConnectionMode) -> Self {
        self.connection_mode = ::std::option::Option::Some(input);
        self
    }
    /// <p>This field specifies whether the connection mode is through a standard distribution (direct) or a multi-tenant distribution with distribution tenants (tenant-only).</p>
    pub fn set_connection_mode(mut self, input: ::std::option::Option<crate::types::ConnectionMode>) -> Self {
        self.connection_mode = input;
        self
    }
    /// <p>This field specifies whether the connection mode is through a standard distribution (direct) or a multi-tenant distribution with distribution tenants (tenant-only).</p>
    pub fn get_connection_mode(&self) -> &::std::option::Option<crate::types::ConnectionMode> {
        &self.connection_mode
    }
    /// Consumes the builder and constructs a [`ListDistributionsByConnectionModeInput`](crate::operation::list_distributions_by_connection_mode::ListDistributionsByConnectionModeInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::list_distributions_by_connection_mode::ListDistributionsByConnectionModeInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::list_distributions_by_connection_mode::ListDistributionsByConnectionModeInput {
                marker: self.marker,
                max_items: self.max_items,
                connection_mode: self.connection_mode,
            },
        )
    }
}
