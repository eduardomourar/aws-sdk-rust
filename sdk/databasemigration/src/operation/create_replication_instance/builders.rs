// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_replication_instance::_create_replication_instance_output::CreateReplicationInstanceOutputBuilder;

pub use crate::operation::create_replication_instance::_create_replication_instance_input::CreateReplicationInstanceInputBuilder;

impl crate::operation::create_replication_instance::builders::CreateReplicationInstanceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_replication_instance::CreateReplicationInstanceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_replication_instance::CreateReplicationInstanceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_replication_instance();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateReplicationInstance`.
///
/// <p>Creates the replication instance using the specified parameters.</p>
/// <p>DMS requires that your account have certain roles with appropriate permissions before you can create a replication instance. For information on the required roles, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Security.html#CHAP_Security.APIRole">Creating the IAM Roles to Use With the CLI and DMS API</a>. For information on the required permissions, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Security.html#CHAP_Security.IAMPermissions">IAM Permissions Needed to Use DMS</a>.</p><note>
/// <p>If you don't specify a version when creating a replication instance, DMS will create the instance using the default engine version. For information about the default engine version, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_ReleaseNotes.html">Release Notes</a>.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateReplicationInstanceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_replication_instance::builders::CreateReplicationInstanceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_replication_instance::CreateReplicationInstanceOutput,
        crate::operation::create_replication_instance::CreateReplicationInstanceError,
    > for CreateReplicationInstanceFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_replication_instance::CreateReplicationInstanceOutput,
            crate::operation::create_replication_instance::CreateReplicationInstanceError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateReplicationInstanceFluentBuilder {
    /// Creates a new `CreateReplicationInstanceFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateReplicationInstance as a reference.
    pub fn as_input(&self) -> &crate::operation::create_replication_instance::builders::CreateReplicationInstanceInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_replication_instance::CreateReplicationInstanceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_replication_instance::CreateReplicationInstanceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_replication_instance::CreateReplicationInstance::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_replication_instance::CreateReplicationInstance::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_replication_instance::CreateReplicationInstanceOutput,
        crate::operation::create_replication_instance::CreateReplicationInstanceError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
        self.set_config_override(::std::option::Option::Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The replication instance identifier. This parameter is stored as a lowercase string.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>Must contain 1-63 alphanumeric characters or hyphens.</p></li>
    /// <li>
    /// <p>First character must be a letter.</p></li>
    /// <li>
    /// <p>Can't end with a hyphen or contain two consecutive hyphens.</p></li>
    /// </ul>
    /// <p>Example: <code>myrepinstance</code></p>
    pub fn replication_instance_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.replication_instance_identifier(input.into());
        self
    }
    /// <p>The replication instance identifier. This parameter is stored as a lowercase string.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>Must contain 1-63 alphanumeric characters or hyphens.</p></li>
    /// <li>
    /// <p>First character must be a letter.</p></li>
    /// <li>
    /// <p>Can't end with a hyphen or contain two consecutive hyphens.</p></li>
    /// </ul>
    /// <p>Example: <code>myrepinstance</code></p>
    pub fn set_replication_instance_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_replication_instance_identifier(input);
        self
    }
    /// <p>The replication instance identifier. This parameter is stored as a lowercase string.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>Must contain 1-63 alphanumeric characters or hyphens.</p></li>
    /// <li>
    /// <p>First character must be a letter.</p></li>
    /// <li>
    /// <p>Can't end with a hyphen or contain two consecutive hyphens.</p></li>
    /// </ul>
    /// <p>Example: <code>myrepinstance</code></p>
    pub fn get_replication_instance_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_replication_instance_identifier()
    }
    /// <p>The amount of storage (in gigabytes) to be initially allocated for the replication instance.</p>
    pub fn allocated_storage(mut self, input: i32) -> Self {
        self.inner = self.inner.allocated_storage(input);
        self
    }
    /// <p>The amount of storage (in gigabytes) to be initially allocated for the replication instance.</p>
    pub fn set_allocated_storage(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_allocated_storage(input);
        self
    }
    /// <p>The amount of storage (in gigabytes) to be initially allocated for the replication instance.</p>
    pub fn get_allocated_storage(&self) -> &::std::option::Option<i32> {
        self.inner.get_allocated_storage()
    }
    /// <p>The compute and memory capacity of the replication instance as defined for the specified replication instance class. For example to specify the instance class dms.c4.large, set this parameter to <code>"dms.c4.large"</code>.</p>
    /// <p>For more information on the settings and capacities for the available replication instance classes, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_ReplicationInstance.Types.html "> Choosing the right DMS replication instance</a>; and, <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_BestPractices.SizingReplicationInstance.html">Selecting the best size for a replication instance</a>.</p>
    pub fn replication_instance_class(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.replication_instance_class(input.into());
        self
    }
    /// <p>The compute and memory capacity of the replication instance as defined for the specified replication instance class. For example to specify the instance class dms.c4.large, set this parameter to <code>"dms.c4.large"</code>.</p>
    /// <p>For more information on the settings and capacities for the available replication instance classes, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_ReplicationInstance.Types.html "> Choosing the right DMS replication instance</a>; and, <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_BestPractices.SizingReplicationInstance.html">Selecting the best size for a replication instance</a>.</p>
    pub fn set_replication_instance_class(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_replication_instance_class(input);
        self
    }
    /// <p>The compute and memory capacity of the replication instance as defined for the specified replication instance class. For example to specify the instance class dms.c4.large, set this parameter to <code>"dms.c4.large"</code>.</p>
    /// <p>For more information on the settings and capacities for the available replication instance classes, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_ReplicationInstance.Types.html "> Choosing the right DMS replication instance</a>; and, <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_BestPractices.SizingReplicationInstance.html">Selecting the best size for a replication instance</a>.</p>
    pub fn get_replication_instance_class(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_replication_instance_class()
    }
    ///
    /// Appends an item to `VpcSecurityGroupIds`.
    ///
    /// To override the contents of this collection use [`set_vpc_security_group_ids`](Self::set_vpc_security_group_ids).
    ///
    /// <p>Specifies the VPC security group to be used with the replication instance. The VPC security group must work with the VPC containing the replication instance.</p>
    pub fn vpc_security_group_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.vpc_security_group_ids(input.into());
        self
    }
    /// <p>Specifies the VPC security group to be used with the replication instance. The VPC security group must work with the VPC containing the replication instance.</p>
    pub fn set_vpc_security_group_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_vpc_security_group_ids(input);
        self
    }
    /// <p>Specifies the VPC security group to be used with the replication instance. The VPC security group must work with the VPC containing the replication instance.</p>
    pub fn get_vpc_security_group_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_vpc_security_group_ids()
    }
    /// <p>The Availability Zone where the replication instance will be created. The default value is a random, system-chosen Availability Zone in the endpoint's Amazon Web Services Region, for example: <code>us-east-1d</code>.</p>
    pub fn availability_zone(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.availability_zone(input.into());
        self
    }
    /// <p>The Availability Zone where the replication instance will be created. The default value is a random, system-chosen Availability Zone in the endpoint's Amazon Web Services Region, for example: <code>us-east-1d</code>.</p>
    pub fn set_availability_zone(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_availability_zone(input);
        self
    }
    /// <p>The Availability Zone where the replication instance will be created. The default value is a random, system-chosen Availability Zone in the endpoint's Amazon Web Services Region, for example: <code>us-east-1d</code>.</p>
    pub fn get_availability_zone(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_availability_zone()
    }
    /// <p>A subnet group to associate with the replication instance.</p>
    pub fn replication_subnet_group_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.replication_subnet_group_identifier(input.into());
        self
    }
    /// <p>A subnet group to associate with the replication instance.</p>
    pub fn set_replication_subnet_group_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_replication_subnet_group_identifier(input);
        self
    }
    /// <p>A subnet group to associate with the replication instance.</p>
    pub fn get_replication_subnet_group_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_replication_subnet_group_identifier()
    }
    /// <p>The weekly time range during which system maintenance can occur, in Universal Coordinated Time (UTC).</p>
    /// <p>Format: <code>ddd:hh24:mi-ddd:hh24:mi</code></p>
    /// <p>Default: A 30-minute window selected at random from an 8-hour block of time per Amazon Web Services Region, occurring on a random day of the week.</p>
    /// <p>Valid Days: Mon, Tue, Wed, Thu, Fri, Sat, Sun</p>
    /// <p>Constraints: Minimum 30-minute window.</p>
    pub fn preferred_maintenance_window(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.preferred_maintenance_window(input.into());
        self
    }
    /// <p>The weekly time range during which system maintenance can occur, in Universal Coordinated Time (UTC).</p>
    /// <p>Format: <code>ddd:hh24:mi-ddd:hh24:mi</code></p>
    /// <p>Default: A 30-minute window selected at random from an 8-hour block of time per Amazon Web Services Region, occurring on a random day of the week.</p>
    /// <p>Valid Days: Mon, Tue, Wed, Thu, Fri, Sat, Sun</p>
    /// <p>Constraints: Minimum 30-minute window.</p>
    pub fn set_preferred_maintenance_window(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_preferred_maintenance_window(input);
        self
    }
    /// <p>The weekly time range during which system maintenance can occur, in Universal Coordinated Time (UTC).</p>
    /// <p>Format: <code>ddd:hh24:mi-ddd:hh24:mi</code></p>
    /// <p>Default: A 30-minute window selected at random from an 8-hour block of time per Amazon Web Services Region, occurring on a random day of the week.</p>
    /// <p>Valid Days: Mon, Tue, Wed, Thu, Fri, Sat, Sun</p>
    /// <p>Constraints: Minimum 30-minute window.</p>
    pub fn get_preferred_maintenance_window(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_preferred_maintenance_window()
    }
    /// <p>Specifies whether the replication instance is a Multi-AZ deployment. You can't set the <code>AvailabilityZone</code> parameter if the Multi-AZ parameter is set to <code>true</code>.</p>
    pub fn multi_az(mut self, input: bool) -> Self {
        self.inner = self.inner.multi_az(input);
        self
    }
    /// <p>Specifies whether the replication instance is a Multi-AZ deployment. You can't set the <code>AvailabilityZone</code> parameter if the Multi-AZ parameter is set to <code>true</code>.</p>
    pub fn set_multi_az(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_multi_az(input);
        self
    }
    /// <p>Specifies whether the replication instance is a Multi-AZ deployment. You can't set the <code>AvailabilityZone</code> parameter if the Multi-AZ parameter is set to <code>true</code>.</p>
    pub fn get_multi_az(&self) -> &::std::option::Option<bool> {
        self.inner.get_multi_az()
    }
    /// <p>The engine version number of the replication instance.</p>
    /// <p>If an engine version number is not specified when a replication instance is created, the default is the latest engine version available.</p>
    pub fn engine_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.engine_version(input.into());
        self
    }
    /// <p>The engine version number of the replication instance.</p>
    /// <p>If an engine version number is not specified when a replication instance is created, the default is the latest engine version available.</p>
    pub fn set_engine_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_engine_version(input);
        self
    }
    /// <p>The engine version number of the replication instance.</p>
    /// <p>If an engine version number is not specified when a replication instance is created, the default is the latest engine version available.</p>
    pub fn get_engine_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_engine_version()
    }
    /// <p>A value that indicates whether minor engine upgrades are applied automatically to the replication instance during the maintenance window. This parameter defaults to <code>true</code>.</p>
    /// <p>Default: <code>true</code></p>
    pub fn auto_minor_version_upgrade(mut self, input: bool) -> Self {
        self.inner = self.inner.auto_minor_version_upgrade(input);
        self
    }
    /// <p>A value that indicates whether minor engine upgrades are applied automatically to the replication instance during the maintenance window. This parameter defaults to <code>true</code>.</p>
    /// <p>Default: <code>true</code></p>
    pub fn set_auto_minor_version_upgrade(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_auto_minor_version_upgrade(input);
        self
    }
    /// <p>A value that indicates whether minor engine upgrades are applied automatically to the replication instance during the maintenance window. This parameter defaults to <code>true</code>.</p>
    /// <p>Default: <code>true</code></p>
    pub fn get_auto_minor_version_upgrade(&self) -> &::std::option::Option<bool> {
        self.inner.get_auto_minor_version_upgrade()
    }
    ///
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>One or more tags to be assigned to the replication instance.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>One or more tags to be assigned to the replication instance.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>One or more tags to be assigned to the replication instance.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
    /// <p>An KMS key identifier that is used to encrypt the data on the replication instance.</p>
    /// <p>If you don't specify a value for the <code>KmsKeyId</code> parameter, then DMS uses your default encryption key.</p>
    /// <p>KMS creates the default encryption key for your Amazon Web Services account. Your Amazon Web Services account has a different default encryption key for each Amazon Web Services Region.</p>
    pub fn kms_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.kms_key_id(input.into());
        self
    }
    /// <p>An KMS key identifier that is used to encrypt the data on the replication instance.</p>
    /// <p>If you don't specify a value for the <code>KmsKeyId</code> parameter, then DMS uses your default encryption key.</p>
    /// <p>KMS creates the default encryption key for your Amazon Web Services account. Your Amazon Web Services account has a different default encryption key for each Amazon Web Services Region.</p>
    pub fn set_kms_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_kms_key_id(input);
        self
    }
    /// <p>An KMS key identifier that is used to encrypt the data on the replication instance.</p>
    /// <p>If you don't specify a value for the <code>KmsKeyId</code> parameter, then DMS uses your default encryption key.</p>
    /// <p>KMS creates the default encryption key for your Amazon Web Services account. Your Amazon Web Services account has a different default encryption key for each Amazon Web Services Region.</p>
    pub fn get_kms_key_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_kms_key_id()
    }
    /// <p>Specifies the accessibility options for the replication instance. A value of <code>true</code> represents an instance with a public IP address. A value of <code>false</code> represents an instance with a private IP address. The default value is <code>true</code>.</p>
    pub fn publicly_accessible(mut self, input: bool) -> Self {
        self.inner = self.inner.publicly_accessible(input);
        self
    }
    /// <p>Specifies the accessibility options for the replication instance. A value of <code>true</code> represents an instance with a public IP address. A value of <code>false</code> represents an instance with a private IP address. The default value is <code>true</code>.</p>
    pub fn set_publicly_accessible(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_publicly_accessible(input);
        self
    }
    /// <p>Specifies the accessibility options for the replication instance. A value of <code>true</code> represents an instance with a public IP address. A value of <code>false</code> represents an instance with a private IP address. The default value is <code>true</code>.</p>
    pub fn get_publicly_accessible(&self) -> &::std::option::Option<bool> {
        self.inner.get_publicly_accessible()
    }
    /// <p>A list of custom DNS name servers supported for the replication instance to access your on-premise source or target database. This list overrides the default name servers supported by the replication instance. You can specify a comma-separated list of internet addresses for up to four on-premise DNS name servers. For example: <code>"1.1.1.1,2.2.2.2,3.3.3.3,4.4.4.4"</code></p>
    pub fn dns_name_servers(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.dns_name_servers(input.into());
        self
    }
    /// <p>A list of custom DNS name servers supported for the replication instance to access your on-premise source or target database. This list overrides the default name servers supported by the replication instance. You can specify a comma-separated list of internet addresses for up to four on-premise DNS name servers. For example: <code>"1.1.1.1,2.2.2.2,3.3.3.3,4.4.4.4"</code></p>
    pub fn set_dns_name_servers(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_dns_name_servers(input);
        self
    }
    /// <p>A list of custom DNS name servers supported for the replication instance to access your on-premise source or target database. This list overrides the default name servers supported by the replication instance. You can specify a comma-separated list of internet addresses for up to four on-premise DNS name servers. For example: <code>"1.1.1.1,2.2.2.2,3.3.3.3,4.4.4.4"</code></p>
    pub fn get_dns_name_servers(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_dns_name_servers()
    }
    /// <p>A friendly name for the resource identifier at the end of the <code>EndpointArn</code> response parameter that is returned in the created <code>Endpoint</code> object. The value for this parameter can have up to 31 characters. It can contain only ASCII letters, digits, and hyphen ('-'). Also, it can't end with a hyphen or contain two consecutive hyphens, and can only begin with a letter, such as <code>Example-App-ARN1</code>. For example, this value might result in the <code>EndpointArn</code> value <code>arn:aws:dms:eu-west-1:012345678901:rep:Example-App-ARN1</code>. If you don't specify a <code>ResourceIdentifier</code> value, DMS generates a default identifier value for the end of <code>EndpointArn</code>.</p>
    pub fn resource_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_identifier(input.into());
        self
    }
    /// <p>A friendly name for the resource identifier at the end of the <code>EndpointArn</code> response parameter that is returned in the created <code>Endpoint</code> object. The value for this parameter can have up to 31 characters. It can contain only ASCII letters, digits, and hyphen ('-'). Also, it can't end with a hyphen or contain two consecutive hyphens, and can only begin with a letter, such as <code>Example-App-ARN1</code>. For example, this value might result in the <code>EndpointArn</code> value <code>arn:aws:dms:eu-west-1:012345678901:rep:Example-App-ARN1</code>. If you don't specify a <code>ResourceIdentifier</code> value, DMS generates a default identifier value for the end of <code>EndpointArn</code>.</p>
    pub fn set_resource_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_identifier(input);
        self
    }
    /// <p>A friendly name for the resource identifier at the end of the <code>EndpointArn</code> response parameter that is returned in the created <code>Endpoint</code> object. The value for this parameter can have up to 31 characters. It can contain only ASCII letters, digits, and hyphen ('-'). Also, it can't end with a hyphen or contain two consecutive hyphens, and can only begin with a letter, such as <code>Example-App-ARN1</code>. For example, this value might result in the <code>EndpointArn</code> value <code>arn:aws:dms:eu-west-1:012345678901:rep:Example-App-ARN1</code>. If you don't specify a <code>ResourceIdentifier</code> value, DMS generates a default identifier value for the end of <code>EndpointArn</code>.</p>
    pub fn get_resource_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resource_identifier()
    }
    /// <p>The type of IP address protocol used by a replication instance, such as IPv4 only or Dual-stack that supports both IPv4 and IPv6 addressing. IPv6 only is not yet supported.</p>
    pub fn network_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.network_type(input.into());
        self
    }
    /// <p>The type of IP address protocol used by a replication instance, such as IPv4 only or Dual-stack that supports both IPv4 and IPv6 addressing. IPv6 only is not yet supported.</p>
    pub fn set_network_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_network_type(input);
        self
    }
    /// <p>The type of IP address protocol used by a replication instance, such as IPv4 only or Dual-stack that supports both IPv4 and IPv6 addressing. IPv6 only is not yet supported.</p>
    pub fn get_network_type(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_network_type()
    }
    /// <p>Specifies the settings required for kerberos authentication when creating the replication instance.</p>
    pub fn kerberos_authentication_settings(mut self, input: crate::types::KerberosAuthenticationSettings) -> Self {
        self.inner = self.inner.kerberos_authentication_settings(input);
        self
    }
    /// <p>Specifies the settings required for kerberos authentication when creating the replication instance.</p>
    pub fn set_kerberos_authentication_settings(mut self, input: ::std::option::Option<crate::types::KerberosAuthenticationSettings>) -> Self {
        self.inner = self.inner.set_kerberos_authentication_settings(input);
        self
    }
    /// <p>Specifies the settings required for kerberos authentication when creating the replication instance.</p>
    pub fn get_kerberos_authentication_settings(&self) -> &::std::option::Option<crate::types::KerberosAuthenticationSettings> {
        self.inner.get_kerberos_authentication_settings()
    }
}
