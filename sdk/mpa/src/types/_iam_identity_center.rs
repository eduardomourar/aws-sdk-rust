// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>IAM Identity Center credentials. For more information see, <a href="http://aws.amazon.com/identity-center/">IAM Identity Center</a> .</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct IamIdentityCenter {
    /// <p>Amazon Resource Name (ARN) for the IAM Identity Center instance.</p>
    pub instance_arn: ::std::string::String,
    /// <p>Amazon Web Services Region where the IAM Identity Center instance is located.</p>
    pub region: ::std::string::String,
}
impl IamIdentityCenter {
    /// <p>Amazon Resource Name (ARN) for the IAM Identity Center instance.</p>
    pub fn instance_arn(&self) -> &str {
        use std::ops::Deref;
        self.instance_arn.deref()
    }
    /// <p>Amazon Web Services Region where the IAM Identity Center instance is located.</p>
    pub fn region(&self) -> &str {
        use std::ops::Deref;
        self.region.deref()
    }
}
impl IamIdentityCenter {
    /// Creates a new builder-style object to manufacture [`IamIdentityCenter`](crate::types::IamIdentityCenter).
    pub fn builder() -> crate::types::builders::IamIdentityCenterBuilder {
        crate::types::builders::IamIdentityCenterBuilder::default()
    }
}

/// A builder for [`IamIdentityCenter`](crate::types::IamIdentityCenter).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct IamIdentityCenterBuilder {
    pub(crate) instance_arn: ::std::option::Option<::std::string::String>,
    pub(crate) region: ::std::option::Option<::std::string::String>,
}
impl IamIdentityCenterBuilder {
    /// <p>Amazon Resource Name (ARN) for the IAM Identity Center instance.</p>
    /// This field is required.
    pub fn instance_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.instance_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Amazon Resource Name (ARN) for the IAM Identity Center instance.</p>
    pub fn set_instance_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.instance_arn = input;
        self
    }
    /// <p>Amazon Resource Name (ARN) for the IAM Identity Center instance.</p>
    pub fn get_instance_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.instance_arn
    }
    /// <p>Amazon Web Services Region where the IAM Identity Center instance is located.</p>
    /// This field is required.
    pub fn region(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.region = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Amazon Web Services Region where the IAM Identity Center instance is located.</p>
    pub fn set_region(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.region = input;
        self
    }
    /// <p>Amazon Web Services Region where the IAM Identity Center instance is located.</p>
    pub fn get_region(&self) -> &::std::option::Option<::std::string::String> {
        &self.region
    }
    /// Consumes the builder and constructs a [`IamIdentityCenter`](crate::types::IamIdentityCenter).
    /// This method will fail if any of the following fields are not set:
    /// - [`instance_arn`](crate::types::builders::IamIdentityCenterBuilder::instance_arn)
    /// - [`region`](crate::types::builders::IamIdentityCenterBuilder::region)
    pub fn build(self) -> ::std::result::Result<crate::types::IamIdentityCenter, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::IamIdentityCenter {
            instance_arn: self.instance_arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "instance_arn",
                    "instance_arn was not specified but it is required when building IamIdentityCenter",
                )
            })?,
            region: self.region.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "region",
                    "region was not specified but it is required when building IamIdentityCenter",
                )
            })?,
        })
    }
}
