// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_whats_app_message_template::_update_whats_app_message_template_output::UpdateWhatsAppMessageTemplateOutputBuilder;

pub use crate::operation::update_whats_app_message_template::_update_whats_app_message_template_input::UpdateWhatsAppMessageTemplateInputBuilder;

impl crate::operation::update_whats_app_message_template::builders::UpdateWhatsAppMessageTemplateInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_whats_app_message_template::UpdateWhatsAppMessageTemplateOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_whats_app_message_template::UpdateWhatsAppMessageTemplateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_whats_app_message_template();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateWhatsAppMessageTemplate`.
///
/// <p>Updates an existing WhatsApp message template.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateWhatsAppMessageTemplateFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_whats_app_message_template::builders::UpdateWhatsAppMessageTemplateInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_whats_app_message_template::UpdateWhatsAppMessageTemplateOutput,
        crate::operation::update_whats_app_message_template::UpdateWhatsAppMessageTemplateError,
    > for UpdateWhatsAppMessageTemplateFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_whats_app_message_template::UpdateWhatsAppMessageTemplateOutput,
            crate::operation::update_whats_app_message_template::UpdateWhatsAppMessageTemplateError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateWhatsAppMessageTemplateFluentBuilder {
    /// Creates a new `UpdateWhatsAppMessageTemplateFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateWhatsAppMessageTemplate as a reference.
    pub fn as_input(&self) -> &crate::operation::update_whats_app_message_template::builders::UpdateWhatsAppMessageTemplateInputBuilder {
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
        crate::operation::update_whats_app_message_template::UpdateWhatsAppMessageTemplateOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_whats_app_message_template::UpdateWhatsAppMessageTemplateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_whats_app_message_template::UpdateWhatsAppMessageTemplate::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_whats_app_message_template::UpdateWhatsAppMessageTemplate::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_whats_app_message_template::UpdateWhatsAppMessageTemplateOutput,
        crate::operation::update_whats_app_message_template::UpdateWhatsAppMessageTemplateError,
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
    /// <p>The ID of the WhatsApp Business Account associated with this template.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The ID of the WhatsApp Business Account associated with this template.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>The ID of the WhatsApp Business Account associated with this template.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_id()
    }
    /// <p>The numeric ID of the template assigned by Meta.</p>
    pub fn meta_template_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.meta_template_id(input.into());
        self
    }
    /// <p>The numeric ID of the template assigned by Meta.</p>
    pub fn set_meta_template_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_meta_template_id(input);
        self
    }
    /// <p>The numeric ID of the template assigned by Meta.</p>
    pub fn get_meta_template_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_meta_template_id()
    }
    /// <p>The new category for the template (for example, UTILITY or MARKETING).</p>
    pub fn template_category(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.template_category(input.into());
        self
    }
    /// <p>The new category for the template (for example, UTILITY or MARKETING).</p>
    pub fn set_template_category(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_template_category(input);
        self
    }
    /// <p>The new category for the template (for example, UTILITY or MARKETING).</p>
    pub fn get_template_category(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_template_category()
    }
    /// <p>The updated components of the template as a JSON blob (maximum 3000 characters).</p>
    pub fn template_components(mut self, input: ::aws_smithy_types::Blob) -> Self {
        self.inner = self.inner.template_components(input);
        self
    }
    /// <p>The updated components of the template as a JSON blob (maximum 3000 characters).</p>
    pub fn set_template_components(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.inner = self.inner.set_template_components(input);
        self
    }
    /// <p>The updated components of the template as a JSON blob (maximum 3000 characters).</p>
    pub fn get_template_components(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        self.inner.get_template_components()
    }
}
