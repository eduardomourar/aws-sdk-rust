// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateFileSystemInput {
    /// <p>A string of up to 64 ASCII characters. Amazon EFS uses this to ensure idempotent creation.</p>
    pub creation_token: ::std::option::Option<::std::string::String>,
    /// <p>The performance mode of the file system. We recommend <code>generalPurpose</code> performance mode for all file systems. File systems using the <code>maxIO</code> performance mode can scale to higher levels of aggregate throughput and operations per second with a tradeoff of slightly higher latencies for most file operations. The performance mode can't be changed after the file system has been created. The <code>maxIO</code> mode is not supported on One Zone file systems.</p><important>
    /// <p>Due to the higher per-operation latencies with Max I/O, we recommend using General Purpose performance mode for all file systems.</p>
    /// </important>
    /// <p>Default is <code>generalPurpose</code>.</p>
    pub performance_mode: ::std::option::Option<crate::types::PerformanceMode>,
    /// <p>A Boolean value that, if true, creates an encrypted file system. When creating an encrypted file system, you have the option of specifying an existing Key Management Service key (KMS key). If you don't specify a KMS key, then the default KMS key for Amazon EFS, <code>/aws/elasticfilesystem</code>, is used to protect the encrypted file system.</p>
    pub encrypted: ::std::option::Option<bool>,
    /// <p>The ID of the KMS key that you want to use to protect the encrypted file system. This parameter is required only if you want to use a non-default KMS key. If this parameter is not specified, the default KMS key for Amazon EFS is used. You can specify a KMS key ID using the following formats:</p>
    /// <ul>
    /// <li>
    /// <p>Key ID - A unique identifier of the key, for example <code>1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p></li>
    /// <li>
    /// <p>ARN - An Amazon Resource Name (ARN) for the key, for example <code>arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p></li>
    /// <li>
    /// <p>Key alias - A previously created display name for a key, for example <code>alias/projectKey1</code>.</p></li>
    /// <li>
    /// <p>Key alias ARN - An ARN for a key alias, for example <code>arn:aws:kms:us-west-2:444455556666:alias/projectKey1</code>.</p></li>
    /// </ul>
    /// <p>If you use <code>KmsKeyId</code>, you must set the <code>CreateFileSystemRequest$Encrypted</code> parameter to true.</p><important>
    /// <p>EFS accepts only symmetric KMS keys. You cannot use asymmetric KMS keys with Amazon EFS file systems.</p>
    /// </important>
    pub kms_key_id: ::std::option::Option<::std::string::String>,
    /// <p>Specifies the throughput mode for the file system. The mode can be <code>bursting</code>, <code>provisioned</code>, or <code>elastic</code>. If you set <code>ThroughputMode</code> to <code>provisioned</code>, you must also set a value for <code>ProvisionedThroughputInMibps</code>. After you create the file system, you can decrease your file system's Provisioned throughput or change between the throughput modes, with certain time restrictions. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/performance.html#provisioned-throughput">Specifying throughput with provisioned mode</a> in the <i>Amazon EFS User Guide</i>.</p>
    /// <p>Default is <code>bursting</code>.</p>
    pub throughput_mode: ::std::option::Option<crate::types::ThroughputMode>,
    /// <p>The throughput, measured in mebibytes per second (MiBps), that you want to provision for a file system that you're creating. Required if <code>ThroughputMode</code> is set to <code>provisioned</code>. Valid values are 1-3414 MiBps, with the upper limit depending on Region. To increase this limit, contact Amazon Web ServicesSupport. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/limits.html#soft-limits">Amazon EFS quotas that you can increase</a> in the <i>Amazon EFS User Guide</i>.</p>
    pub provisioned_throughput_in_mibps: ::std::option::Option<f64>,
    /// <p>For One Zone file systems, specify the Amazon Web Services Availability Zone in which to create the file system. Use the format <code>us-east-1a</code> to specify the Availability Zone. For more information about One Zone file systems, see <a href="https://docs.aws.amazon.com/efs/latest/ug/availability-durability.html#file-system-type">EFS file system types</a> in the <i>Amazon EFS User Guide</i>.</p><note>
    /// <p>One Zone file systems are not available in all Availability Zones in Amazon Web Services Regions where Amazon EFS is available.</p>
    /// </note>
    pub availability_zone_name: ::std::option::Option<::std::string::String>,
    /// <p>Specifies whether automatic backups are enabled on the file system that you are creating. Set the value to <code>true</code> to enable automatic backups. If you are creating a One Zone file system, automatic backups are enabled by default. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/awsbackup.html#automatic-backups">Automatic backups</a> in the <i>Amazon EFS User Guide</i>.</p>
    /// <p>Default is <code>false</code>. However, if you specify an <code>AvailabilityZoneName</code>, the default is <code>true</code>.</p><note>
    /// <p>Backup is not available in all Amazon Web Services Regions where Amazon EFS is available.</p>
    /// </note>
    pub backup: ::std::option::Option<bool>,
    /// <p>Use to create one or more tags associated with the file system. Each tag is a user-defined key-value pair. Name your file system on creation by including a <code>"Key":"Name","Value":"{value}"</code> key-value pair. Each key must be unique. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a> in the <i>Amazon Web Services General Reference Guide</i>.</p>
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl CreateFileSystemInput {
    /// <p>A string of up to 64 ASCII characters. Amazon EFS uses this to ensure idempotent creation.</p>
    pub fn creation_token(&self) -> ::std::option::Option<&str> {
        self.creation_token.as_deref()
    }
    /// <p>The performance mode of the file system. We recommend <code>generalPurpose</code> performance mode for all file systems. File systems using the <code>maxIO</code> performance mode can scale to higher levels of aggregate throughput and operations per second with a tradeoff of slightly higher latencies for most file operations. The performance mode can't be changed after the file system has been created. The <code>maxIO</code> mode is not supported on One Zone file systems.</p><important>
    /// <p>Due to the higher per-operation latencies with Max I/O, we recommend using General Purpose performance mode for all file systems.</p>
    /// </important>
    /// <p>Default is <code>generalPurpose</code>.</p>
    pub fn performance_mode(&self) -> ::std::option::Option<&crate::types::PerformanceMode> {
        self.performance_mode.as_ref()
    }
    /// <p>A Boolean value that, if true, creates an encrypted file system. When creating an encrypted file system, you have the option of specifying an existing Key Management Service key (KMS key). If you don't specify a KMS key, then the default KMS key for Amazon EFS, <code>/aws/elasticfilesystem</code>, is used to protect the encrypted file system.</p>
    pub fn encrypted(&self) -> ::std::option::Option<bool> {
        self.encrypted
    }
    /// <p>The ID of the KMS key that you want to use to protect the encrypted file system. This parameter is required only if you want to use a non-default KMS key. If this parameter is not specified, the default KMS key for Amazon EFS is used. You can specify a KMS key ID using the following formats:</p>
    /// <ul>
    /// <li>
    /// <p>Key ID - A unique identifier of the key, for example <code>1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p></li>
    /// <li>
    /// <p>ARN - An Amazon Resource Name (ARN) for the key, for example <code>arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p></li>
    /// <li>
    /// <p>Key alias - A previously created display name for a key, for example <code>alias/projectKey1</code>.</p></li>
    /// <li>
    /// <p>Key alias ARN - An ARN for a key alias, for example <code>arn:aws:kms:us-west-2:444455556666:alias/projectKey1</code>.</p></li>
    /// </ul>
    /// <p>If you use <code>KmsKeyId</code>, you must set the <code>CreateFileSystemRequest$Encrypted</code> parameter to true.</p><important>
    /// <p>EFS accepts only symmetric KMS keys. You cannot use asymmetric KMS keys with Amazon EFS file systems.</p>
    /// </important>
    pub fn kms_key_id(&self) -> ::std::option::Option<&str> {
        self.kms_key_id.as_deref()
    }
    /// <p>Specifies the throughput mode for the file system. The mode can be <code>bursting</code>, <code>provisioned</code>, or <code>elastic</code>. If you set <code>ThroughputMode</code> to <code>provisioned</code>, you must also set a value for <code>ProvisionedThroughputInMibps</code>. After you create the file system, you can decrease your file system's Provisioned throughput or change between the throughput modes, with certain time restrictions. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/performance.html#provisioned-throughput">Specifying throughput with provisioned mode</a> in the <i>Amazon EFS User Guide</i>.</p>
    /// <p>Default is <code>bursting</code>.</p>
    pub fn throughput_mode(&self) -> ::std::option::Option<&crate::types::ThroughputMode> {
        self.throughput_mode.as_ref()
    }
    /// <p>The throughput, measured in mebibytes per second (MiBps), that you want to provision for a file system that you're creating. Required if <code>ThroughputMode</code> is set to <code>provisioned</code>. Valid values are 1-3414 MiBps, with the upper limit depending on Region. To increase this limit, contact Amazon Web ServicesSupport. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/limits.html#soft-limits">Amazon EFS quotas that you can increase</a> in the <i>Amazon EFS User Guide</i>.</p>
    pub fn provisioned_throughput_in_mibps(&self) -> ::std::option::Option<f64> {
        self.provisioned_throughput_in_mibps
    }
    /// <p>For One Zone file systems, specify the Amazon Web Services Availability Zone in which to create the file system. Use the format <code>us-east-1a</code> to specify the Availability Zone. For more information about One Zone file systems, see <a href="https://docs.aws.amazon.com/efs/latest/ug/availability-durability.html#file-system-type">EFS file system types</a> in the <i>Amazon EFS User Guide</i>.</p><note>
    /// <p>One Zone file systems are not available in all Availability Zones in Amazon Web Services Regions where Amazon EFS is available.</p>
    /// </note>
    pub fn availability_zone_name(&self) -> ::std::option::Option<&str> {
        self.availability_zone_name.as_deref()
    }
    /// <p>Specifies whether automatic backups are enabled on the file system that you are creating. Set the value to <code>true</code> to enable automatic backups. If you are creating a One Zone file system, automatic backups are enabled by default. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/awsbackup.html#automatic-backups">Automatic backups</a> in the <i>Amazon EFS User Guide</i>.</p>
    /// <p>Default is <code>false</code>. However, if you specify an <code>AvailabilityZoneName</code>, the default is <code>true</code>.</p><note>
    /// <p>Backup is not available in all Amazon Web Services Regions where Amazon EFS is available.</p>
    /// </note>
    pub fn backup(&self) -> ::std::option::Option<bool> {
        self.backup
    }
    /// <p>Use to create one or more tags associated with the file system. Each tag is a user-defined key-value pair. Name your file system on creation by including a <code>"Key":"Name","Value":"{value}"</code> key-value pair. Each key must be unique. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a> in the <i>Amazon Web Services General Reference Guide</i>.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.tags.is_none()`.
    pub fn tags(&self) -> &[crate::types::Tag] {
        self.tags.as_deref().unwrap_or_default()
    }
}
impl CreateFileSystemInput {
    /// Creates a new builder-style object to manufacture [`CreateFileSystemInput`](crate::operation::create_file_system::CreateFileSystemInput).
    pub fn builder() -> crate::operation::create_file_system::builders::CreateFileSystemInputBuilder {
        crate::operation::create_file_system::builders::CreateFileSystemInputBuilder::default()
    }
}

/// A builder for [`CreateFileSystemInput`](crate::operation::create_file_system::CreateFileSystemInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct CreateFileSystemInputBuilder {
    pub(crate) creation_token: ::std::option::Option<::std::string::String>,
    pub(crate) performance_mode: ::std::option::Option<crate::types::PerformanceMode>,
    pub(crate) encrypted: ::std::option::Option<bool>,
    pub(crate) kms_key_id: ::std::option::Option<::std::string::String>,
    pub(crate) throughput_mode: ::std::option::Option<crate::types::ThroughputMode>,
    pub(crate) provisioned_throughput_in_mibps: ::std::option::Option<f64>,
    pub(crate) availability_zone_name: ::std::option::Option<::std::string::String>,
    pub(crate) backup: ::std::option::Option<bool>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl CreateFileSystemInputBuilder {
    /// <p>A string of up to 64 ASCII characters. Amazon EFS uses this to ensure idempotent creation.</p>
    /// This field is required.
    pub fn creation_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.creation_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A string of up to 64 ASCII characters. Amazon EFS uses this to ensure idempotent creation.</p>
    pub fn set_creation_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.creation_token = input;
        self
    }
    /// <p>A string of up to 64 ASCII characters. Amazon EFS uses this to ensure idempotent creation.</p>
    pub fn get_creation_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.creation_token
    }
    /// <p>The performance mode of the file system. We recommend <code>generalPurpose</code> performance mode for all file systems. File systems using the <code>maxIO</code> performance mode can scale to higher levels of aggregate throughput and operations per second with a tradeoff of slightly higher latencies for most file operations. The performance mode can't be changed after the file system has been created. The <code>maxIO</code> mode is not supported on One Zone file systems.</p><important>
    /// <p>Due to the higher per-operation latencies with Max I/O, we recommend using General Purpose performance mode for all file systems.</p>
    /// </important>
    /// <p>Default is <code>generalPurpose</code>.</p>
    pub fn performance_mode(mut self, input: crate::types::PerformanceMode) -> Self {
        self.performance_mode = ::std::option::Option::Some(input);
        self
    }
    /// <p>The performance mode of the file system. We recommend <code>generalPurpose</code> performance mode for all file systems. File systems using the <code>maxIO</code> performance mode can scale to higher levels of aggregate throughput and operations per second with a tradeoff of slightly higher latencies for most file operations. The performance mode can't be changed after the file system has been created. The <code>maxIO</code> mode is not supported on One Zone file systems.</p><important>
    /// <p>Due to the higher per-operation latencies with Max I/O, we recommend using General Purpose performance mode for all file systems.</p>
    /// </important>
    /// <p>Default is <code>generalPurpose</code>.</p>
    pub fn set_performance_mode(mut self, input: ::std::option::Option<crate::types::PerformanceMode>) -> Self {
        self.performance_mode = input;
        self
    }
    /// <p>The performance mode of the file system. We recommend <code>generalPurpose</code> performance mode for all file systems. File systems using the <code>maxIO</code> performance mode can scale to higher levels of aggregate throughput and operations per second with a tradeoff of slightly higher latencies for most file operations. The performance mode can't be changed after the file system has been created. The <code>maxIO</code> mode is not supported on One Zone file systems.</p><important>
    /// <p>Due to the higher per-operation latencies with Max I/O, we recommend using General Purpose performance mode for all file systems.</p>
    /// </important>
    /// <p>Default is <code>generalPurpose</code>.</p>
    pub fn get_performance_mode(&self) -> &::std::option::Option<crate::types::PerformanceMode> {
        &self.performance_mode
    }
    /// <p>A Boolean value that, if true, creates an encrypted file system. When creating an encrypted file system, you have the option of specifying an existing Key Management Service key (KMS key). If you don't specify a KMS key, then the default KMS key for Amazon EFS, <code>/aws/elasticfilesystem</code>, is used to protect the encrypted file system.</p>
    pub fn encrypted(mut self, input: bool) -> Self {
        self.encrypted = ::std::option::Option::Some(input);
        self
    }
    /// <p>A Boolean value that, if true, creates an encrypted file system. When creating an encrypted file system, you have the option of specifying an existing Key Management Service key (KMS key). If you don't specify a KMS key, then the default KMS key for Amazon EFS, <code>/aws/elasticfilesystem</code>, is used to protect the encrypted file system.</p>
    pub fn set_encrypted(mut self, input: ::std::option::Option<bool>) -> Self {
        self.encrypted = input;
        self
    }
    /// <p>A Boolean value that, if true, creates an encrypted file system. When creating an encrypted file system, you have the option of specifying an existing Key Management Service key (KMS key). If you don't specify a KMS key, then the default KMS key for Amazon EFS, <code>/aws/elasticfilesystem</code>, is used to protect the encrypted file system.</p>
    pub fn get_encrypted(&self) -> &::std::option::Option<bool> {
        &self.encrypted
    }
    /// <p>The ID of the KMS key that you want to use to protect the encrypted file system. This parameter is required only if you want to use a non-default KMS key. If this parameter is not specified, the default KMS key for Amazon EFS is used. You can specify a KMS key ID using the following formats:</p>
    /// <ul>
    /// <li>
    /// <p>Key ID - A unique identifier of the key, for example <code>1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p></li>
    /// <li>
    /// <p>ARN - An Amazon Resource Name (ARN) for the key, for example <code>arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p></li>
    /// <li>
    /// <p>Key alias - A previously created display name for a key, for example <code>alias/projectKey1</code>.</p></li>
    /// <li>
    /// <p>Key alias ARN - An ARN for a key alias, for example <code>arn:aws:kms:us-west-2:444455556666:alias/projectKey1</code>.</p></li>
    /// </ul>
    /// <p>If you use <code>KmsKeyId</code>, you must set the <code>CreateFileSystemRequest$Encrypted</code> parameter to true.</p><important>
    /// <p>EFS accepts only symmetric KMS keys. You cannot use asymmetric KMS keys with Amazon EFS file systems.</p>
    /// </important>
    pub fn kms_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.kms_key_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the KMS key that you want to use to protect the encrypted file system. This parameter is required only if you want to use a non-default KMS key. If this parameter is not specified, the default KMS key for Amazon EFS is used. You can specify a KMS key ID using the following formats:</p>
    /// <ul>
    /// <li>
    /// <p>Key ID - A unique identifier of the key, for example <code>1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p></li>
    /// <li>
    /// <p>ARN - An Amazon Resource Name (ARN) for the key, for example <code>arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p></li>
    /// <li>
    /// <p>Key alias - A previously created display name for a key, for example <code>alias/projectKey1</code>.</p></li>
    /// <li>
    /// <p>Key alias ARN - An ARN for a key alias, for example <code>arn:aws:kms:us-west-2:444455556666:alias/projectKey1</code>.</p></li>
    /// </ul>
    /// <p>If you use <code>KmsKeyId</code>, you must set the <code>CreateFileSystemRequest$Encrypted</code> parameter to true.</p><important>
    /// <p>EFS accepts only symmetric KMS keys. You cannot use asymmetric KMS keys with Amazon EFS file systems.</p>
    /// </important>
    pub fn set_kms_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.kms_key_id = input;
        self
    }
    /// <p>The ID of the KMS key that you want to use to protect the encrypted file system. This parameter is required only if you want to use a non-default KMS key. If this parameter is not specified, the default KMS key for Amazon EFS is used. You can specify a KMS key ID using the following formats:</p>
    /// <ul>
    /// <li>
    /// <p>Key ID - A unique identifier of the key, for example <code>1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p></li>
    /// <li>
    /// <p>ARN - An Amazon Resource Name (ARN) for the key, for example <code>arn:aws:kms:us-west-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p></li>
    /// <li>
    /// <p>Key alias - A previously created display name for a key, for example <code>alias/projectKey1</code>.</p></li>
    /// <li>
    /// <p>Key alias ARN - An ARN for a key alias, for example <code>arn:aws:kms:us-west-2:444455556666:alias/projectKey1</code>.</p></li>
    /// </ul>
    /// <p>If you use <code>KmsKeyId</code>, you must set the <code>CreateFileSystemRequest$Encrypted</code> parameter to true.</p><important>
    /// <p>EFS accepts only symmetric KMS keys. You cannot use asymmetric KMS keys with Amazon EFS file systems.</p>
    /// </important>
    pub fn get_kms_key_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.kms_key_id
    }
    /// <p>Specifies the throughput mode for the file system. The mode can be <code>bursting</code>, <code>provisioned</code>, or <code>elastic</code>. If you set <code>ThroughputMode</code> to <code>provisioned</code>, you must also set a value for <code>ProvisionedThroughputInMibps</code>. After you create the file system, you can decrease your file system's Provisioned throughput or change between the throughput modes, with certain time restrictions. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/performance.html#provisioned-throughput">Specifying throughput with provisioned mode</a> in the <i>Amazon EFS User Guide</i>.</p>
    /// <p>Default is <code>bursting</code>.</p>
    pub fn throughput_mode(mut self, input: crate::types::ThroughputMode) -> Self {
        self.throughput_mode = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the throughput mode for the file system. The mode can be <code>bursting</code>, <code>provisioned</code>, or <code>elastic</code>. If you set <code>ThroughputMode</code> to <code>provisioned</code>, you must also set a value for <code>ProvisionedThroughputInMibps</code>. After you create the file system, you can decrease your file system's Provisioned throughput or change between the throughput modes, with certain time restrictions. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/performance.html#provisioned-throughput">Specifying throughput with provisioned mode</a> in the <i>Amazon EFS User Guide</i>.</p>
    /// <p>Default is <code>bursting</code>.</p>
    pub fn set_throughput_mode(mut self, input: ::std::option::Option<crate::types::ThroughputMode>) -> Self {
        self.throughput_mode = input;
        self
    }
    /// <p>Specifies the throughput mode for the file system. The mode can be <code>bursting</code>, <code>provisioned</code>, or <code>elastic</code>. If you set <code>ThroughputMode</code> to <code>provisioned</code>, you must also set a value for <code>ProvisionedThroughputInMibps</code>. After you create the file system, you can decrease your file system's Provisioned throughput or change between the throughput modes, with certain time restrictions. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/performance.html#provisioned-throughput">Specifying throughput with provisioned mode</a> in the <i>Amazon EFS User Guide</i>.</p>
    /// <p>Default is <code>bursting</code>.</p>
    pub fn get_throughput_mode(&self) -> &::std::option::Option<crate::types::ThroughputMode> {
        &self.throughput_mode
    }
    /// <p>The throughput, measured in mebibytes per second (MiBps), that you want to provision for a file system that you're creating. Required if <code>ThroughputMode</code> is set to <code>provisioned</code>. Valid values are 1-3414 MiBps, with the upper limit depending on Region. To increase this limit, contact Amazon Web ServicesSupport. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/limits.html#soft-limits">Amazon EFS quotas that you can increase</a> in the <i>Amazon EFS User Guide</i>.</p>
    pub fn provisioned_throughput_in_mibps(mut self, input: f64) -> Self {
        self.provisioned_throughput_in_mibps = ::std::option::Option::Some(input);
        self
    }
    /// <p>The throughput, measured in mebibytes per second (MiBps), that you want to provision for a file system that you're creating. Required if <code>ThroughputMode</code> is set to <code>provisioned</code>. Valid values are 1-3414 MiBps, with the upper limit depending on Region. To increase this limit, contact Amazon Web ServicesSupport. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/limits.html#soft-limits">Amazon EFS quotas that you can increase</a> in the <i>Amazon EFS User Guide</i>.</p>
    pub fn set_provisioned_throughput_in_mibps(mut self, input: ::std::option::Option<f64>) -> Self {
        self.provisioned_throughput_in_mibps = input;
        self
    }
    /// <p>The throughput, measured in mebibytes per second (MiBps), that you want to provision for a file system that you're creating. Required if <code>ThroughputMode</code> is set to <code>provisioned</code>. Valid values are 1-3414 MiBps, with the upper limit depending on Region. To increase this limit, contact Amazon Web ServicesSupport. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/limits.html#soft-limits">Amazon EFS quotas that you can increase</a> in the <i>Amazon EFS User Guide</i>.</p>
    pub fn get_provisioned_throughput_in_mibps(&self) -> &::std::option::Option<f64> {
        &self.provisioned_throughput_in_mibps
    }
    /// <p>For One Zone file systems, specify the Amazon Web Services Availability Zone in which to create the file system. Use the format <code>us-east-1a</code> to specify the Availability Zone. For more information about One Zone file systems, see <a href="https://docs.aws.amazon.com/efs/latest/ug/availability-durability.html#file-system-type">EFS file system types</a> in the <i>Amazon EFS User Guide</i>.</p><note>
    /// <p>One Zone file systems are not available in all Availability Zones in Amazon Web Services Regions where Amazon EFS is available.</p>
    /// </note>
    pub fn availability_zone_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.availability_zone_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>For One Zone file systems, specify the Amazon Web Services Availability Zone in which to create the file system. Use the format <code>us-east-1a</code> to specify the Availability Zone. For more information about One Zone file systems, see <a href="https://docs.aws.amazon.com/efs/latest/ug/availability-durability.html#file-system-type">EFS file system types</a> in the <i>Amazon EFS User Guide</i>.</p><note>
    /// <p>One Zone file systems are not available in all Availability Zones in Amazon Web Services Regions where Amazon EFS is available.</p>
    /// </note>
    pub fn set_availability_zone_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.availability_zone_name = input;
        self
    }
    /// <p>For One Zone file systems, specify the Amazon Web Services Availability Zone in which to create the file system. Use the format <code>us-east-1a</code> to specify the Availability Zone. For more information about One Zone file systems, see <a href="https://docs.aws.amazon.com/efs/latest/ug/availability-durability.html#file-system-type">EFS file system types</a> in the <i>Amazon EFS User Guide</i>.</p><note>
    /// <p>One Zone file systems are not available in all Availability Zones in Amazon Web Services Regions where Amazon EFS is available.</p>
    /// </note>
    pub fn get_availability_zone_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.availability_zone_name
    }
    /// <p>Specifies whether automatic backups are enabled on the file system that you are creating. Set the value to <code>true</code> to enable automatic backups. If you are creating a One Zone file system, automatic backups are enabled by default. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/awsbackup.html#automatic-backups">Automatic backups</a> in the <i>Amazon EFS User Guide</i>.</p>
    /// <p>Default is <code>false</code>. However, if you specify an <code>AvailabilityZoneName</code>, the default is <code>true</code>.</p><note>
    /// <p>Backup is not available in all Amazon Web Services Regions where Amazon EFS is available.</p>
    /// </note>
    pub fn backup(mut self, input: bool) -> Self {
        self.backup = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies whether automatic backups are enabled on the file system that you are creating. Set the value to <code>true</code> to enable automatic backups. If you are creating a One Zone file system, automatic backups are enabled by default. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/awsbackup.html#automatic-backups">Automatic backups</a> in the <i>Amazon EFS User Guide</i>.</p>
    /// <p>Default is <code>false</code>. However, if you specify an <code>AvailabilityZoneName</code>, the default is <code>true</code>.</p><note>
    /// <p>Backup is not available in all Amazon Web Services Regions where Amazon EFS is available.</p>
    /// </note>
    pub fn set_backup(mut self, input: ::std::option::Option<bool>) -> Self {
        self.backup = input;
        self
    }
    /// <p>Specifies whether automatic backups are enabled on the file system that you are creating. Set the value to <code>true</code> to enable automatic backups. If you are creating a One Zone file system, automatic backups are enabled by default. For more information, see <a href="https://docs.aws.amazon.com/efs/latest/ug/awsbackup.html#automatic-backups">Automatic backups</a> in the <i>Amazon EFS User Guide</i>.</p>
    /// <p>Default is <code>false</code>. However, if you specify an <code>AvailabilityZoneName</code>, the default is <code>true</code>.</p><note>
    /// <p>Backup is not available in all Amazon Web Services Regions where Amazon EFS is available.</p>
    /// </note>
    pub fn get_backup(&self) -> &::std::option::Option<bool> {
        &self.backup
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Use to create one or more tags associated with the file system. Each tag is a user-defined key-value pair. Name your file system on creation by including a <code>"Key":"Name","Value":"{value}"</code> key-value pair. Each key must be unique. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a> in the <i>Amazon Web Services General Reference Guide</i>.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>Use to create one or more tags associated with the file system. Each tag is a user-defined key-value pair. Name your file system on creation by including a <code>"Key":"Name","Value":"{value}"</code> key-value pair. Each key must be unique. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a> in the <i>Amazon Web Services General Reference Guide</i>.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.tags = input;
        self
    }
    /// <p>Use to create one or more tags associated with the file system. Each tag is a user-defined key-value pair. Name your file system on creation by including a <code>"Key":"Name","Value":"{value}"</code> key-value pair. Each key must be unique. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws_tagging.html">Tagging Amazon Web Services resources</a> in the <i>Amazon Web Services General Reference Guide</i>.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        &self.tags
    }
    /// Consumes the builder and constructs a [`CreateFileSystemInput`](crate::operation::create_file_system::CreateFileSystemInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::create_file_system::CreateFileSystemInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::create_file_system::CreateFileSystemInput {
            creation_token: self.creation_token,
            performance_mode: self.performance_mode,
            encrypted: self.encrypted,
            kms_key_id: self.kms_key_id,
            throughput_mode: self.throughput_mode,
            provisioned_throughput_in_mibps: self.provisioned_throughput_in_mibps,
            availability_zone_name: self.availability_zone_name,
            backup: self.backup,
            tags: self.tags,
        })
    }
}
