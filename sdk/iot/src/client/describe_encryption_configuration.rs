// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeEncryptionConfiguration`](crate::operation::describe_encryption_configuration::builders::DescribeEncryptionConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::describe_encryption_configuration::builders::DescribeEncryptionConfigurationFluentBuilder::send) it.
    /// - On success, responds with [`DescribeEncryptionConfigurationOutput`](crate::operation::describe_encryption_configuration::DescribeEncryptionConfigurationOutput) with field(s):
    ///   - [`encryption_type(Option<EncryptionType>)`](crate::operation::describe_encryption_configuration::DescribeEncryptionConfigurationOutput::encryption_type): <p>The type of the Amazon Web Services Key Management Service (KMS) key.</p>
    ///   - [`kms_key_arn(Option<String>)`](crate::operation::describe_encryption_configuration::DescribeEncryptionConfigurationOutput::kms_key_arn): <p>The Amazon Resource Name (ARN) of the IAM role assumed by Amazon Web Services IoT Core to call KMS on behalf of the customer.</p>
    ///   - [`kms_access_role_arn(Option<String>)`](crate::operation::describe_encryption_configuration::DescribeEncryptionConfigurationOutput::kms_access_role_arn): <p>The ARN of the customer-managed KMS key.</p>
    ///   - [`configuration_details(Option<ConfigurationDetails>)`](crate::operation::describe_encryption_configuration::DescribeEncryptionConfigurationOutput::configuration_details): <p>The encryption configuration details that include the status information of the KMS key and the KMS access role.</p>
    ///   - [`last_modified_date(Option<DateTime>)`](crate::operation::describe_encryption_configuration::DescribeEncryptionConfigurationOutput::last_modified_date): <p>The date when encryption configuration is last updated.</p>
    /// - On failure, responds with [`SdkError<DescribeEncryptionConfigurationError>`](crate::operation::describe_encryption_configuration::DescribeEncryptionConfigurationError)
    pub fn describe_encryption_configuration(
        &self,
    ) -> crate::operation::describe_encryption_configuration::builders::DescribeEncryptionConfigurationFluentBuilder {
        crate::operation::describe_encryption_configuration::builders::DescribeEncryptionConfigurationFluentBuilder::new(self.handle.clone())
    }
}
