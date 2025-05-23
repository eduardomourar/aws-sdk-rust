// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_replication::_start_replication_output::StartReplicationOutputBuilder;

pub use crate::operation::start_replication::_start_replication_input::StartReplicationInputBuilder;

impl crate::operation::start_replication::builders::StartReplicationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::start_replication::StartReplicationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_replication::StartReplicationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.start_replication();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StartReplication`.
///
/// <p>For a given DMS Serverless replication configuration, DMS connects to the source endpoint and collects the metadata to analyze the replication workload. Using this metadata, DMS then computes and provisions the required capacity and starts replicating to the target endpoint using the server resources that DMS has provisioned for the DMS Serverless replication.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartReplicationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_replication::builders::StartReplicationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::start_replication::StartReplicationOutput,
        crate::operation::start_replication::StartReplicationError,
    > for StartReplicationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::start_replication::StartReplicationOutput,
            crate::operation::start_replication::StartReplicationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl StartReplicationFluentBuilder {
    /// Creates a new `StartReplicationFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StartReplication as a reference.
    pub fn as_input(&self) -> &crate::operation::start_replication::builders::StartReplicationInputBuilder {
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
        crate::operation::start_replication::StartReplicationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_replication::StartReplicationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::start_replication::StartReplication::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::start_replication::StartReplication::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::start_replication::StartReplicationOutput,
        crate::operation::start_replication::StartReplicationError,
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
    /// <p>The Amazon Resource Name of the replication for which to start replication.</p>
    pub fn replication_config_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.replication_config_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name of the replication for which to start replication.</p>
    pub fn set_replication_config_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_replication_config_arn(input);
        self
    }
    /// <p>The Amazon Resource Name of the replication for which to start replication.</p>
    pub fn get_replication_config_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_replication_config_arn()
    }
    /// <p>The replication type.</p>
    /// <p>When the replication type is <code>full-load</code> or <code>full-load-and-cdc</code>, the only valid value for the first run of the replication is <code>start-replication</code>. This option will start the replication.</p>
    /// <p>You can also use <code>ReloadTables</code> to reload specific tables that failed during replication instead of restarting the replication.</p>
    /// <p>The <code>resume-processing</code> option isn't applicable for a full-load replication, because you can't resume partially loaded tables during the full load phase.</p>
    /// <p>For a <code>full-load-and-cdc</code> replication, DMS migrates table data, and then applies data changes that occur on the source. To load all the tables again, and start capturing source changes, use <code>reload-target</code>. Otherwise use <code>resume-processing</code>, to replicate the changes from the last stop position.</p>
    pub fn start_replication_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.start_replication_type(input.into());
        self
    }
    /// <p>The replication type.</p>
    /// <p>When the replication type is <code>full-load</code> or <code>full-load-and-cdc</code>, the only valid value for the first run of the replication is <code>start-replication</code>. This option will start the replication.</p>
    /// <p>You can also use <code>ReloadTables</code> to reload specific tables that failed during replication instead of restarting the replication.</p>
    /// <p>The <code>resume-processing</code> option isn't applicable for a full-load replication, because you can't resume partially loaded tables during the full load phase.</p>
    /// <p>For a <code>full-load-and-cdc</code> replication, DMS migrates table data, and then applies data changes that occur on the source. To load all the tables again, and start capturing source changes, use <code>reload-target</code>. Otherwise use <code>resume-processing</code>, to replicate the changes from the last stop position.</p>
    pub fn set_start_replication_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_start_replication_type(input);
        self
    }
    /// <p>The replication type.</p>
    /// <p>When the replication type is <code>full-load</code> or <code>full-load-and-cdc</code>, the only valid value for the first run of the replication is <code>start-replication</code>. This option will start the replication.</p>
    /// <p>You can also use <code>ReloadTables</code> to reload specific tables that failed during replication instead of restarting the replication.</p>
    /// <p>The <code>resume-processing</code> option isn't applicable for a full-load replication, because you can't resume partially loaded tables during the full load phase.</p>
    /// <p>For a <code>full-load-and-cdc</code> replication, DMS migrates table data, and then applies data changes that occur on the source. To load all the tables again, and start capturing source changes, use <code>reload-target</code>. Otherwise use <code>resume-processing</code>, to replicate the changes from the last stop position.</p>
    pub fn get_start_replication_type(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_start_replication_type()
    }
    /// <p>User-defined settings for the premigration assessment. The possible values are:</p>
    /// <ul>
    /// <li>
    /// <p><code>ResultLocationFolder</code>: The folder within an Amazon S3 bucket where you want DMS to store the results of this assessment run.</p></li>
    /// <li>
    /// <p><code>ResultEncryptionMode</code>: The supported values are <code>SSE_KMS</code> and <code>SSE_S3</code>. If these values are not provided, then the files are not encrypted at rest. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.S3.html#CHAP_Target.S3.KMSKeys">Creating Amazon Web Services KMS keys to encrypt Amazon S3 target objects</a>.</p></li>
    /// <li>
    /// <p><code>ResultKmsKeyArn</code>: The ARN of a customer KMS encryption key that you specify when you set <code>ResultEncryptionMode</code> to <code>SSE_KMS</code>.</p></li>
    /// <li>
    /// <p><code>IncludeOnly</code>: A space-separated list of names for specific individual assessments that you want to include. These names come from the default list of individual assessments that Database Migration Service supports for the associated migration.</p></li>
    /// <li>
    /// <p><code>Exclude</code>: A space-separated list of names for specific individual assessments that you want to exclude. These names come from the default list of individual assessments that Database Migration Service supports for the associated migration.</p></li>
    /// <li>
    /// <p><code>FailOnAssessmentFailure</code>: A configurable setting you can set to <code>true</code> (the default setting) or <code>false</code>. Use this setting to to stop the replication from starting automatically if the assessment fails. This can help you evaluate the issue that is preventing the replication from running successfully.</p></li>
    /// </ul>
    pub fn premigration_assessment_settings(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.premigration_assessment_settings(input.into());
        self
    }
    /// <p>User-defined settings for the premigration assessment. The possible values are:</p>
    /// <ul>
    /// <li>
    /// <p><code>ResultLocationFolder</code>: The folder within an Amazon S3 bucket where you want DMS to store the results of this assessment run.</p></li>
    /// <li>
    /// <p><code>ResultEncryptionMode</code>: The supported values are <code>SSE_KMS</code> and <code>SSE_S3</code>. If these values are not provided, then the files are not encrypted at rest. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.S3.html#CHAP_Target.S3.KMSKeys">Creating Amazon Web Services KMS keys to encrypt Amazon S3 target objects</a>.</p></li>
    /// <li>
    /// <p><code>ResultKmsKeyArn</code>: The ARN of a customer KMS encryption key that you specify when you set <code>ResultEncryptionMode</code> to <code>SSE_KMS</code>.</p></li>
    /// <li>
    /// <p><code>IncludeOnly</code>: A space-separated list of names for specific individual assessments that you want to include. These names come from the default list of individual assessments that Database Migration Service supports for the associated migration.</p></li>
    /// <li>
    /// <p><code>Exclude</code>: A space-separated list of names for specific individual assessments that you want to exclude. These names come from the default list of individual assessments that Database Migration Service supports for the associated migration.</p></li>
    /// <li>
    /// <p><code>FailOnAssessmentFailure</code>: A configurable setting you can set to <code>true</code> (the default setting) or <code>false</code>. Use this setting to to stop the replication from starting automatically if the assessment fails. This can help you evaluate the issue that is preventing the replication from running successfully.</p></li>
    /// </ul>
    pub fn set_premigration_assessment_settings(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_premigration_assessment_settings(input);
        self
    }
    /// <p>User-defined settings for the premigration assessment. The possible values are:</p>
    /// <ul>
    /// <li>
    /// <p><code>ResultLocationFolder</code>: The folder within an Amazon S3 bucket where you want DMS to store the results of this assessment run.</p></li>
    /// <li>
    /// <p><code>ResultEncryptionMode</code>: The supported values are <code>SSE_KMS</code> and <code>SSE_S3</code>. If these values are not provided, then the files are not encrypted at rest. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.S3.html#CHAP_Target.S3.KMSKeys">Creating Amazon Web Services KMS keys to encrypt Amazon S3 target objects</a>.</p></li>
    /// <li>
    /// <p><code>ResultKmsKeyArn</code>: The ARN of a customer KMS encryption key that you specify when you set <code>ResultEncryptionMode</code> to <code>SSE_KMS</code>.</p></li>
    /// <li>
    /// <p><code>IncludeOnly</code>: A space-separated list of names for specific individual assessments that you want to include. These names come from the default list of individual assessments that Database Migration Service supports for the associated migration.</p></li>
    /// <li>
    /// <p><code>Exclude</code>: A space-separated list of names for specific individual assessments that you want to exclude. These names come from the default list of individual assessments that Database Migration Service supports for the associated migration.</p></li>
    /// <li>
    /// <p><code>FailOnAssessmentFailure</code>: A configurable setting you can set to <code>true</code> (the default setting) or <code>false</code>. Use this setting to to stop the replication from starting automatically if the assessment fails. This can help you evaluate the issue that is preventing the replication from running successfully.</p></li>
    /// </ul>
    pub fn get_premigration_assessment_settings(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_premigration_assessment_settings()
    }
    /// <p>Indicates the start time for a change data capture (CDC) operation. Use either <code>CdcStartTime</code> or <code>CdcStartPosition</code> to specify when you want a CDC operation to start. Specifying both values results in an error.</p>
    pub fn cdc_start_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.cdc_start_time(input);
        self
    }
    /// <p>Indicates the start time for a change data capture (CDC) operation. Use either <code>CdcStartTime</code> or <code>CdcStartPosition</code> to specify when you want a CDC operation to start. Specifying both values results in an error.</p>
    pub fn set_cdc_start_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_cdc_start_time(input);
        self
    }
    /// <p>Indicates the start time for a change data capture (CDC) operation. Use either <code>CdcStartTime</code> or <code>CdcStartPosition</code> to specify when you want a CDC operation to start. Specifying both values results in an error.</p>
    pub fn get_cdc_start_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_cdc_start_time()
    }
    /// <p>Indicates when you want a change data capture (CDC) operation to start. Use either <code>CdcStartPosition</code> or <code>CdcStartTime</code> to specify when you want a CDC operation to start. Specifying both values results in an error.</p>
    /// <p>The value can be in date, checkpoint, or LSN/SCN format.</p>
    pub fn cdc_start_position(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.cdc_start_position(input.into());
        self
    }
    /// <p>Indicates when you want a change data capture (CDC) operation to start. Use either <code>CdcStartPosition</code> or <code>CdcStartTime</code> to specify when you want a CDC operation to start. Specifying both values results in an error.</p>
    /// <p>The value can be in date, checkpoint, or LSN/SCN format.</p>
    pub fn set_cdc_start_position(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_cdc_start_position(input);
        self
    }
    /// <p>Indicates when you want a change data capture (CDC) operation to start. Use either <code>CdcStartPosition</code> or <code>CdcStartTime</code> to specify when you want a CDC operation to start. Specifying both values results in an error.</p>
    /// <p>The value can be in date, checkpoint, or LSN/SCN format.</p>
    pub fn get_cdc_start_position(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_cdc_start_position()
    }
    /// <p>Indicates when you want a change data capture (CDC) operation to stop. The value can be either server time or commit time.</p>
    pub fn cdc_stop_position(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.cdc_stop_position(input.into());
        self
    }
    /// <p>Indicates when you want a change data capture (CDC) operation to stop. The value can be either server time or commit time.</p>
    pub fn set_cdc_stop_position(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_cdc_stop_position(input);
        self
    }
    /// <p>Indicates when you want a change data capture (CDC) operation to stop. The value can be either server time or commit time.</p>
    pub fn get_cdc_stop_position(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_cdc_stop_position()
    }
}
