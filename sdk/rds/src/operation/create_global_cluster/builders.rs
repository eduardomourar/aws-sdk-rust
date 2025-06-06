// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_global_cluster::_create_global_cluster_output::CreateGlobalClusterOutputBuilder;

pub use crate::operation::create_global_cluster::_create_global_cluster_input::CreateGlobalClusterInputBuilder;

impl crate::operation::create_global_cluster::builders::CreateGlobalClusterInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_global_cluster::CreateGlobalClusterOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_global_cluster::CreateGlobalClusterError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_global_cluster();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateGlobalCluster`.
///
/// <p>Creates an Aurora global database spread across multiple Amazon Web Services Regions. The global database contains a single primary cluster with read-write capability, and a read-only secondary cluster that receives data from the primary cluster through high-speed replication performed by the Aurora storage subsystem.</p>
/// <p>You can create a global database that is initially empty, and then create the primary and secondary DB clusters in the global database. Or you can specify an existing Aurora cluster during the create operation, and this cluster becomes the primary cluster of the global database.</p><note>
/// <p>This operation applies only to Aurora DB clusters.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateGlobalClusterFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_global_cluster::builders::CreateGlobalClusterInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_global_cluster::CreateGlobalClusterOutput,
        crate::operation::create_global_cluster::CreateGlobalClusterError,
    > for CreateGlobalClusterFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_global_cluster::CreateGlobalClusterOutput,
            crate::operation::create_global_cluster::CreateGlobalClusterError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateGlobalClusterFluentBuilder {
    /// Creates a new `CreateGlobalClusterFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateGlobalCluster as a reference.
    pub fn as_input(&self) -> &crate::operation::create_global_cluster::builders::CreateGlobalClusterInputBuilder {
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
        crate::operation::create_global_cluster::CreateGlobalClusterOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_global_cluster::CreateGlobalClusterError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_global_cluster::CreateGlobalCluster::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_global_cluster::CreateGlobalCluster::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_global_cluster::CreateGlobalClusterOutput,
        crate::operation::create_global_cluster::CreateGlobalClusterError,
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
    /// <p>The cluster identifier for this global database cluster. This parameter is stored as a lowercase string.</p>
    pub fn global_cluster_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.global_cluster_identifier(input.into());
        self
    }
    /// <p>The cluster identifier for this global database cluster. This parameter is stored as a lowercase string.</p>
    pub fn set_global_cluster_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_global_cluster_identifier(input);
        self
    }
    /// <p>The cluster identifier for this global database cluster. This parameter is stored as a lowercase string.</p>
    pub fn get_global_cluster_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_global_cluster_identifier()
    }
    /// <p>The Amazon Resource Name (ARN) to use as the primary cluster of the global database.</p>
    /// <p>If you provide a value for this parameter, don't specify values for the following settings because Amazon Aurora uses the values from the specified source DB cluster:</p>
    /// <ul>
    /// <li>
    /// <p><code>DatabaseName</code></p></li>
    /// <li>
    /// <p><code>Engine</code></p></li>
    /// <li>
    /// <p><code>EngineVersion</code></p></li>
    /// <li>
    /// <p><code>StorageEncrypted</code></p></li>
    /// </ul>
    pub fn source_db_cluster_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.source_db_cluster_identifier(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) to use as the primary cluster of the global database.</p>
    /// <p>If you provide a value for this parameter, don't specify values for the following settings because Amazon Aurora uses the values from the specified source DB cluster:</p>
    /// <ul>
    /// <li>
    /// <p><code>DatabaseName</code></p></li>
    /// <li>
    /// <p><code>Engine</code></p></li>
    /// <li>
    /// <p><code>EngineVersion</code></p></li>
    /// <li>
    /// <p><code>StorageEncrypted</code></p></li>
    /// </ul>
    pub fn set_source_db_cluster_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_source_db_cluster_identifier(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) to use as the primary cluster of the global database.</p>
    /// <p>If you provide a value for this parameter, don't specify values for the following settings because Amazon Aurora uses the values from the specified source DB cluster:</p>
    /// <ul>
    /// <li>
    /// <p><code>DatabaseName</code></p></li>
    /// <li>
    /// <p><code>Engine</code></p></li>
    /// <li>
    /// <p><code>EngineVersion</code></p></li>
    /// <li>
    /// <p><code>StorageEncrypted</code></p></li>
    /// </ul>
    pub fn get_source_db_cluster_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_source_db_cluster_identifier()
    }
    /// <p>The database engine to use for this global database cluster.</p>
    /// <p>Valid Values: <code>aurora-mysql | aurora-postgresql</code></p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>Can't be specified if <code>SourceDBClusterIdentifier</code> is specified. In this case, Amazon Aurora uses the engine of the source DB cluster.</p></li>
    /// </ul>
    pub fn engine(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.engine(input.into());
        self
    }
    /// <p>The database engine to use for this global database cluster.</p>
    /// <p>Valid Values: <code>aurora-mysql | aurora-postgresql</code></p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>Can't be specified if <code>SourceDBClusterIdentifier</code> is specified. In this case, Amazon Aurora uses the engine of the source DB cluster.</p></li>
    /// </ul>
    pub fn set_engine(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_engine(input);
        self
    }
    /// <p>The database engine to use for this global database cluster.</p>
    /// <p>Valid Values: <code>aurora-mysql | aurora-postgresql</code></p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>Can't be specified if <code>SourceDBClusterIdentifier</code> is specified. In this case, Amazon Aurora uses the engine of the source DB cluster.</p></li>
    /// </ul>
    pub fn get_engine(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_engine()
    }
    /// <p>The engine version to use for this global database cluster.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>Can't be specified if <code>SourceDBClusterIdentifier</code> is specified. In this case, Amazon Aurora uses the engine version of the source DB cluster.</p></li>
    /// </ul>
    pub fn engine_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.engine_version(input.into());
        self
    }
    /// <p>The engine version to use for this global database cluster.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>Can't be specified if <code>SourceDBClusterIdentifier</code> is specified. In this case, Amazon Aurora uses the engine version of the source DB cluster.</p></li>
    /// </ul>
    pub fn set_engine_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_engine_version(input);
        self
    }
    /// <p>The engine version to use for this global database cluster.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>Can't be specified if <code>SourceDBClusterIdentifier</code> is specified. In this case, Amazon Aurora uses the engine version of the source DB cluster.</p></li>
    /// </ul>
    pub fn get_engine_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_engine_version()
    }
    /// <p>The life cycle type for this global database cluster.</p><note>
    /// <p>By default, this value is set to <code>open-source-rds-extended-support</code>, which enrolls your global cluster into Amazon RDS Extended Support. At the end of standard support, you can avoid charges for Extended Support by setting the value to <code>open-source-rds-extended-support-disabled</code>. In this case, creating the global cluster will fail if the DB major version is past its end of standard support date.</p>
    /// </note>
    /// <p>This setting only applies to Aurora PostgreSQL-based global databases.</p>
    /// <p>You can use this setting to enroll your global cluster into Amazon RDS Extended Support. With RDS Extended Support, you can run the selected major engine version on your global cluster past the end of standard support for that engine version. For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/extended-support.html">Amazon RDS Extended Support with Amazon Aurora</a> in the <i>Amazon Aurora User Guide</i>.</p>
    /// <p>Valid Values: <code>open-source-rds-extended-support | open-source-rds-extended-support-disabled</code></p>
    /// <p>Default: <code>open-source-rds-extended-support</code></p>
    pub fn engine_lifecycle_support(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.engine_lifecycle_support(input.into());
        self
    }
    /// <p>The life cycle type for this global database cluster.</p><note>
    /// <p>By default, this value is set to <code>open-source-rds-extended-support</code>, which enrolls your global cluster into Amazon RDS Extended Support. At the end of standard support, you can avoid charges for Extended Support by setting the value to <code>open-source-rds-extended-support-disabled</code>. In this case, creating the global cluster will fail if the DB major version is past its end of standard support date.</p>
    /// </note>
    /// <p>This setting only applies to Aurora PostgreSQL-based global databases.</p>
    /// <p>You can use this setting to enroll your global cluster into Amazon RDS Extended Support. With RDS Extended Support, you can run the selected major engine version on your global cluster past the end of standard support for that engine version. For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/extended-support.html">Amazon RDS Extended Support with Amazon Aurora</a> in the <i>Amazon Aurora User Guide</i>.</p>
    /// <p>Valid Values: <code>open-source-rds-extended-support | open-source-rds-extended-support-disabled</code></p>
    /// <p>Default: <code>open-source-rds-extended-support</code></p>
    pub fn set_engine_lifecycle_support(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_engine_lifecycle_support(input);
        self
    }
    /// <p>The life cycle type for this global database cluster.</p><note>
    /// <p>By default, this value is set to <code>open-source-rds-extended-support</code>, which enrolls your global cluster into Amazon RDS Extended Support. At the end of standard support, you can avoid charges for Extended Support by setting the value to <code>open-source-rds-extended-support-disabled</code>. In this case, creating the global cluster will fail if the DB major version is past its end of standard support date.</p>
    /// </note>
    /// <p>This setting only applies to Aurora PostgreSQL-based global databases.</p>
    /// <p>You can use this setting to enroll your global cluster into Amazon RDS Extended Support. With RDS Extended Support, you can run the selected major engine version on your global cluster past the end of standard support for that engine version. For more information, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/extended-support.html">Amazon RDS Extended Support with Amazon Aurora</a> in the <i>Amazon Aurora User Guide</i>.</p>
    /// <p>Valid Values: <code>open-source-rds-extended-support | open-source-rds-extended-support-disabled</code></p>
    /// <p>Default: <code>open-source-rds-extended-support</code></p>
    pub fn get_engine_lifecycle_support(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_engine_lifecycle_support()
    }
    /// <p>Specifies whether to enable deletion protection for the new global database cluster. The global database can't be deleted when deletion protection is enabled.</p>
    pub fn deletion_protection(mut self, input: bool) -> Self {
        self.inner = self.inner.deletion_protection(input);
        self
    }
    /// <p>Specifies whether to enable deletion protection for the new global database cluster. The global database can't be deleted when deletion protection is enabled.</p>
    pub fn set_deletion_protection(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_deletion_protection(input);
        self
    }
    /// <p>Specifies whether to enable deletion protection for the new global database cluster. The global database can't be deleted when deletion protection is enabled.</p>
    pub fn get_deletion_protection(&self) -> &::std::option::Option<bool> {
        self.inner.get_deletion_protection()
    }
    /// <p>The name for your database of up to 64 alphanumeric characters. If you don't specify a name, Amazon Aurora doesn't create a database in the global database cluster.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>Can't be specified if <code>SourceDBClusterIdentifier</code> is specified. In this case, Amazon Aurora uses the database name from the source DB cluster.</p></li>
    /// </ul>
    pub fn database_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.database_name(input.into());
        self
    }
    /// <p>The name for your database of up to 64 alphanumeric characters. If you don't specify a name, Amazon Aurora doesn't create a database in the global database cluster.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>Can't be specified if <code>SourceDBClusterIdentifier</code> is specified. In this case, Amazon Aurora uses the database name from the source DB cluster.</p></li>
    /// </ul>
    pub fn set_database_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_database_name(input);
        self
    }
    /// <p>The name for your database of up to 64 alphanumeric characters. If you don't specify a name, Amazon Aurora doesn't create a database in the global database cluster.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>Can't be specified if <code>SourceDBClusterIdentifier</code> is specified. In this case, Amazon Aurora uses the database name from the source DB cluster.</p></li>
    /// </ul>
    pub fn get_database_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_database_name()
    }
    /// <p>Specifies whether to enable storage encryption for the new global database cluster.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>Can't be specified if <code>SourceDBClusterIdentifier</code> is specified. In this case, Amazon Aurora uses the setting from the source DB cluster.</p></li>
    /// </ul>
    pub fn storage_encrypted(mut self, input: bool) -> Self {
        self.inner = self.inner.storage_encrypted(input);
        self
    }
    /// <p>Specifies whether to enable storage encryption for the new global database cluster.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>Can't be specified if <code>SourceDBClusterIdentifier</code> is specified. In this case, Amazon Aurora uses the setting from the source DB cluster.</p></li>
    /// </ul>
    pub fn set_storage_encrypted(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_storage_encrypted(input);
        self
    }
    /// <p>Specifies whether to enable storage encryption for the new global database cluster.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>Can't be specified if <code>SourceDBClusterIdentifier</code> is specified. In this case, Amazon Aurora uses the setting from the source DB cluster.</p></li>
    /// </ul>
    pub fn get_storage_encrypted(&self) -> &::std::option::Option<bool> {
        self.inner.get_storage_encrypted()
    }
    ///
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Tags to assign to the global cluster.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>Tags to assign to the global cluster.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>Tags to assign to the global cluster.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
}
