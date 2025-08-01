// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateEncryptionConfigurationInput {
    /// <p>The type of the Amazon Web Services Key Management Service (KMS) key.</p>
    pub encryption_type: ::std::option::Option<crate::types::EncryptionType>,
    /// <p>The ARN of the customer-managed KMS key.</p>
    pub kms_key_arn: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role assumed by Amazon Web Services IoT Core to call KMS on behalf of the customer.</p>
    pub kms_access_role_arn: ::std::option::Option<::std::string::String>,
}
impl UpdateEncryptionConfigurationInput {
    /// <p>The type of the Amazon Web Services Key Management Service (KMS) key.</p>
    pub fn encryption_type(&self) -> ::std::option::Option<&crate::types::EncryptionType> {
        self.encryption_type.as_ref()
    }
    /// <p>The ARN of the customer-managed KMS key.</p>
    pub fn kms_key_arn(&self) -> ::std::option::Option<&str> {
        self.kms_key_arn.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role assumed by Amazon Web Services IoT Core to call KMS on behalf of the customer.</p>
    pub fn kms_access_role_arn(&self) -> ::std::option::Option<&str> {
        self.kms_access_role_arn.as_deref()
    }
}
impl UpdateEncryptionConfigurationInput {
    /// Creates a new builder-style object to manufacture [`UpdateEncryptionConfigurationInput`](crate::operation::update_encryption_configuration::UpdateEncryptionConfigurationInput).
    pub fn builder() -> crate::operation::update_encryption_configuration::builders::UpdateEncryptionConfigurationInputBuilder {
        crate::operation::update_encryption_configuration::builders::UpdateEncryptionConfigurationInputBuilder::default()
    }
}

/// A builder for [`UpdateEncryptionConfigurationInput`](crate::operation::update_encryption_configuration::UpdateEncryptionConfigurationInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct UpdateEncryptionConfigurationInputBuilder {
    pub(crate) encryption_type: ::std::option::Option<crate::types::EncryptionType>,
    pub(crate) kms_key_arn: ::std::option::Option<::std::string::String>,
    pub(crate) kms_access_role_arn: ::std::option::Option<::std::string::String>,
}
impl UpdateEncryptionConfigurationInputBuilder {
    /// <p>The type of the Amazon Web Services Key Management Service (KMS) key.</p>
    /// This field is required.
    pub fn encryption_type(mut self, input: crate::types::EncryptionType) -> Self {
        self.encryption_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of the Amazon Web Services Key Management Service (KMS) key.</p>
    pub fn set_encryption_type(mut self, input: ::std::option::Option<crate::types::EncryptionType>) -> Self {
        self.encryption_type = input;
        self
    }
    /// <p>The type of the Amazon Web Services Key Management Service (KMS) key.</p>
    pub fn get_encryption_type(&self) -> &::std::option::Option<crate::types::EncryptionType> {
        &self.encryption_type
    }
    /// <p>The ARN of the customer-managed KMS key.</p>
    pub fn kms_key_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.kms_key_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the customer-managed KMS key.</p>
    pub fn set_kms_key_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.kms_key_arn = input;
        self
    }
    /// <p>The ARN of the customer-managed KMS key.</p>
    pub fn get_kms_key_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.kms_key_arn
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role assumed by Amazon Web Services IoT Core to call KMS on behalf of the customer.</p>
    pub fn kms_access_role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.kms_access_role_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role assumed by Amazon Web Services IoT Core to call KMS on behalf of the customer.</p>
    pub fn set_kms_access_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.kms_access_role_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role assumed by Amazon Web Services IoT Core to call KMS on behalf of the customer.</p>
    pub fn get_kms_access_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.kms_access_role_arn
    }
    /// Consumes the builder and constructs a [`UpdateEncryptionConfigurationInput`](crate::operation::update_encryption_configuration::UpdateEncryptionConfigurationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::update_encryption_configuration::UpdateEncryptionConfigurationInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::update_encryption_configuration::UpdateEncryptionConfigurationInput {
            encryption_type: self.encryption_type,
            kms_key_arn: self.kms_key_arn,
            kms_access_role_arn: self.kms_access_role_arn,
        })
    }
}
