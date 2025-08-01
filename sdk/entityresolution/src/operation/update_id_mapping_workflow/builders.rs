// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_id_mapping_workflow::_update_id_mapping_workflow_output::UpdateIdMappingWorkflowOutputBuilder;

pub use crate::operation::update_id_mapping_workflow::_update_id_mapping_workflow_input::UpdateIdMappingWorkflowInputBuilder;

impl crate::operation::update_id_mapping_workflow::builders::UpdateIdMappingWorkflowInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_id_mapping_workflow::UpdateIdMappingWorkflowOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_id_mapping_workflow::UpdateIdMappingWorkflowError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_id_mapping_workflow();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateIdMappingWorkflow`.
///
/// <p>Updates an existing <code>IdMappingWorkflow</code>. This method is identical to CreateIdMappingWorkflow, except it uses an HTTP <code>PUT</code> request instead of a <code>POST</code> request, and the <code>IdMappingWorkflow</code> must already exist for the method to succeed.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateIdMappingWorkflowFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_id_mapping_workflow::builders::UpdateIdMappingWorkflowInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_id_mapping_workflow::UpdateIdMappingWorkflowOutput,
        crate::operation::update_id_mapping_workflow::UpdateIdMappingWorkflowError,
    > for UpdateIdMappingWorkflowFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_id_mapping_workflow::UpdateIdMappingWorkflowOutput,
            crate::operation::update_id_mapping_workflow::UpdateIdMappingWorkflowError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateIdMappingWorkflowFluentBuilder {
    /// Creates a new `UpdateIdMappingWorkflowFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateIdMappingWorkflow as a reference.
    pub fn as_input(&self) -> &crate::operation::update_id_mapping_workflow::builders::UpdateIdMappingWorkflowInputBuilder {
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
        crate::operation::update_id_mapping_workflow::UpdateIdMappingWorkflowOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_id_mapping_workflow::UpdateIdMappingWorkflowError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_id_mapping_workflow::UpdateIdMappingWorkflow::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_id_mapping_workflow::UpdateIdMappingWorkflow::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_id_mapping_workflow::UpdateIdMappingWorkflowOutput,
        crate::operation::update_id_mapping_workflow::UpdateIdMappingWorkflowError,
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
    /// <p>The name of the workflow.</p>
    pub fn workflow_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.workflow_name(input.into());
        self
    }
    /// <p>The name of the workflow.</p>
    pub fn set_workflow_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_workflow_name(input);
        self
    }
    /// <p>The name of the workflow.</p>
    pub fn get_workflow_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_workflow_name()
    }
    /// <p>A description of the workflow.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description of the workflow.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>A description of the workflow.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    ///
    /// Appends an item to `inputSourceConfig`.
    ///
    /// To override the contents of this collection use [`set_input_source_config`](Self::set_input_source_config).
    ///
    /// <p>A list of <code>InputSource</code> objects, which have the fields <code>InputSourceARN</code> and <code>SchemaName</code>.</p>
    pub fn input_source_config(mut self, input: crate::types::IdMappingWorkflowInputSource) -> Self {
        self.inner = self.inner.input_source_config(input);
        self
    }
    /// <p>A list of <code>InputSource</code> objects, which have the fields <code>InputSourceARN</code> and <code>SchemaName</code>.</p>
    pub fn set_input_source_config(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::IdMappingWorkflowInputSource>>) -> Self {
        self.inner = self.inner.set_input_source_config(input);
        self
    }
    /// <p>A list of <code>InputSource</code> objects, which have the fields <code>InputSourceARN</code> and <code>SchemaName</code>.</p>
    pub fn get_input_source_config(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::IdMappingWorkflowInputSource>> {
        self.inner.get_input_source_config()
    }
    ///
    /// Appends an item to `outputSourceConfig`.
    ///
    /// To override the contents of this collection use [`set_output_source_config`](Self::set_output_source_config).
    ///
    /// <p>A list of <code>OutputSource</code> objects, each of which contains fields <code>outputS3Path</code> and <code>KMSArn</code>.</p>
    pub fn output_source_config(mut self, input: crate::types::IdMappingWorkflowOutputSource) -> Self {
        self.inner = self.inner.output_source_config(input);
        self
    }
    /// <p>A list of <code>OutputSource</code> objects, each of which contains fields <code>outputS3Path</code> and <code>KMSArn</code>.</p>
    pub fn set_output_source_config(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::IdMappingWorkflowOutputSource>>) -> Self {
        self.inner = self.inner.set_output_source_config(input);
        self
    }
    /// <p>A list of <code>OutputSource</code> objects, each of which contains fields <code>outputS3Path</code> and <code>KMSArn</code>.</p>
    pub fn get_output_source_config(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::IdMappingWorkflowOutputSource>> {
        self.inner.get_output_source_config()
    }
    /// <p>An object which defines the ID mapping technique and any additional configurations.</p>
    pub fn id_mapping_techniques(mut self, input: crate::types::IdMappingTechniques) -> Self {
        self.inner = self.inner.id_mapping_techniques(input);
        self
    }
    /// <p>An object which defines the ID mapping technique and any additional configurations.</p>
    pub fn set_id_mapping_techniques(mut self, input: ::std::option::Option<crate::types::IdMappingTechniques>) -> Self {
        self.inner = self.inner.set_id_mapping_techniques(input);
        self
    }
    /// <p>An object which defines the ID mapping technique and any additional configurations.</p>
    pub fn get_id_mapping_techniques(&self) -> &::std::option::Option<crate::types::IdMappingTechniques> {
        self.inner.get_id_mapping_techniques()
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role. Entity Resolution assumes this role to access Amazon Web Services resources on your behalf.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.role_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role. Entity Resolution assumes this role to access Amazon Web Services resources on your behalf.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_role_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role. Entity Resolution assumes this role to access Amazon Web Services resources on your behalf.</p>
    pub fn get_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_role_arn()
    }
}
