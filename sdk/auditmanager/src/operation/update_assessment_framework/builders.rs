// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_assessment_framework::_update_assessment_framework_output::UpdateAssessmentFrameworkOutputBuilder;

pub use crate::operation::update_assessment_framework::_update_assessment_framework_input::UpdateAssessmentFrameworkInputBuilder;

impl crate::operation::update_assessment_framework::builders::UpdateAssessmentFrameworkInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_assessment_framework::UpdateAssessmentFrameworkOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_assessment_framework::UpdateAssessmentFrameworkError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_assessment_framework();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateAssessmentFramework`.
///
/// <p>Updates a custom framework in Audit Manager.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateAssessmentFrameworkFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_assessment_framework::builders::UpdateAssessmentFrameworkInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_assessment_framework::UpdateAssessmentFrameworkOutput,
        crate::operation::update_assessment_framework::UpdateAssessmentFrameworkError,
    > for UpdateAssessmentFrameworkFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_assessment_framework::UpdateAssessmentFrameworkOutput,
            crate::operation::update_assessment_framework::UpdateAssessmentFrameworkError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateAssessmentFrameworkFluentBuilder {
    /// Creates a new `UpdateAssessmentFrameworkFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateAssessmentFramework as a reference.
    pub fn as_input(&self) -> &crate::operation::update_assessment_framework::builders::UpdateAssessmentFrameworkInputBuilder {
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
        crate::operation::update_assessment_framework::UpdateAssessmentFrameworkOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_assessment_framework::UpdateAssessmentFrameworkError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_assessment_framework::UpdateAssessmentFramework::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_assessment_framework::UpdateAssessmentFramework::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_assessment_framework::UpdateAssessmentFrameworkOutput,
        crate::operation::update_assessment_framework::UpdateAssessmentFrameworkError,
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
    /// <p>The unique identifier for the framework.</p>
    pub fn framework_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.framework_id(input.into());
        self
    }
    /// <p>The unique identifier for the framework.</p>
    pub fn set_framework_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_framework_id(input);
        self
    }
    /// <p>The unique identifier for the framework.</p>
    pub fn get_framework_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_framework_id()
    }
    /// <p>The name of the framework to be updated.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the framework to be updated.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the framework to be updated.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>The description of the updated framework.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The description of the updated framework.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The description of the updated framework.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>The compliance type that the new custom framework supports, such as CIS or HIPAA.</p>
    pub fn compliance_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.compliance_type(input.into());
        self
    }
    /// <p>The compliance type that the new custom framework supports, such as CIS or HIPAA.</p>
    pub fn set_compliance_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_compliance_type(input);
        self
    }
    /// <p>The compliance type that the new custom framework supports, such as CIS or HIPAA.</p>
    pub fn get_compliance_type(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_compliance_type()
    }
    ///
    /// Appends an item to `controlSets`.
    ///
    /// To override the contents of this collection use [`set_control_sets`](Self::set_control_sets).
    ///
    /// <p>The control sets that are associated with the framework.</p><note>
    /// <p>The <code>Controls</code> object returns a partial response when called through Framework APIs. For a complete <code>Controls</code> object, use <code>GetControl</code>.</p>
    /// </note>
    pub fn control_sets(mut self, input: crate::types::UpdateAssessmentFrameworkControlSet) -> Self {
        self.inner = self.inner.control_sets(input);
        self
    }
    /// <p>The control sets that are associated with the framework.</p><note>
    /// <p>The <code>Controls</code> object returns a partial response when called through Framework APIs. For a complete <code>Controls</code> object, use <code>GetControl</code>.</p>
    /// </note>
    pub fn set_control_sets(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::UpdateAssessmentFrameworkControlSet>>) -> Self {
        self.inner = self.inner.set_control_sets(input);
        self
    }
    /// <p>The control sets that are associated with the framework.</p><note>
    /// <p>The <code>Controls</code> object returns a partial response when called through Framework APIs. For a complete <code>Controls</code> object, use <code>GetControl</code>.</p>
    /// </note>
    pub fn get_control_sets(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::UpdateAssessmentFrameworkControlSet>> {
        self.inner.get_control_sets()
    }
}
