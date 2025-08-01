// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeEncryptionConfigurationInput {}
impl DescribeEncryptionConfigurationInput {
    /// Creates a new builder-style object to manufacture [`DescribeEncryptionConfigurationInput`](crate::operation::describe_encryption_configuration::DescribeEncryptionConfigurationInput).
    pub fn builder() -> crate::operation::describe_encryption_configuration::builders::DescribeEncryptionConfigurationInputBuilder {
        crate::operation::describe_encryption_configuration::builders::DescribeEncryptionConfigurationInputBuilder::default()
    }
}

/// A builder for [`DescribeEncryptionConfigurationInput`](crate::operation::describe_encryption_configuration::DescribeEncryptionConfigurationInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct DescribeEncryptionConfigurationInputBuilder {}
impl DescribeEncryptionConfigurationInputBuilder {
    /// Consumes the builder and constructs a [`DescribeEncryptionConfigurationInput`](crate::operation::describe_encryption_configuration::DescribeEncryptionConfigurationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_encryption_configuration::DescribeEncryptionConfigurationInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_encryption_configuration::DescribeEncryptionConfigurationInput {})
    }
}
