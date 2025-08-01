// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_computation_model::_update_computation_model_output::UpdateComputationModelOutputBuilder;

pub use crate::operation::update_computation_model::_update_computation_model_input::UpdateComputationModelInputBuilder;

impl crate::operation::update_computation_model::builders::UpdateComputationModelInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_computation_model::UpdateComputationModelOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_computation_model::UpdateComputationModelError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_computation_model();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateComputationModel`.
///
/// <p>Updates the computation model.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateComputationModelFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_computation_model::builders::UpdateComputationModelInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_computation_model::UpdateComputationModelOutput,
        crate::operation::update_computation_model::UpdateComputationModelError,
    > for UpdateComputationModelFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_computation_model::UpdateComputationModelOutput,
            crate::operation::update_computation_model::UpdateComputationModelError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateComputationModelFluentBuilder {
    /// Creates a new `UpdateComputationModelFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateComputationModel as a reference.
    pub fn as_input(&self) -> &crate::operation::update_computation_model::builders::UpdateComputationModelInputBuilder {
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
        crate::operation::update_computation_model::UpdateComputationModelOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_computation_model::UpdateComputationModelError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_computation_model::UpdateComputationModel::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_computation_model::UpdateComputationModel::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_computation_model::UpdateComputationModelOutput,
        crate::operation::update_computation_model::UpdateComputationModelError,
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
    /// <p>The ID of the computation model.</p>
    pub fn computation_model_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.computation_model_id(input.into());
        self
    }
    /// <p>The ID of the computation model.</p>
    pub fn set_computation_model_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_computation_model_id(input);
        self
    }
    /// <p>The ID of the computation model.</p>
    pub fn get_computation_model_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_computation_model_id()
    }
    /// <p>The name of the computation model.</p>
    pub fn computation_model_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.computation_model_name(input.into());
        self
    }
    /// <p>The name of the computation model.</p>
    pub fn set_computation_model_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_computation_model_name(input);
        self
    }
    /// <p>The name of the computation model.</p>
    pub fn get_computation_model_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_computation_model_name()
    }
    /// <p>The description of the computation model.</p>
    pub fn computation_model_description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.computation_model_description(input.into());
        self
    }
    /// <p>The description of the computation model.</p>
    pub fn set_computation_model_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_computation_model_description(input);
        self
    }
    /// <p>The description of the computation model.</p>
    pub fn get_computation_model_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_computation_model_description()
    }
    /// <p>The configuration for the computation model.</p>
    pub fn computation_model_configuration(mut self, input: crate::types::ComputationModelConfiguration) -> Self {
        self.inner = self.inner.computation_model_configuration(input);
        self
    }
    /// <p>The configuration for the computation model.</p>
    pub fn set_computation_model_configuration(mut self, input: ::std::option::Option<crate::types::ComputationModelConfiguration>) -> Self {
        self.inner = self.inner.set_computation_model_configuration(input);
        self
    }
    /// <p>The configuration for the computation model.</p>
    pub fn get_computation_model_configuration(&self) -> &::std::option::Option<crate::types::ComputationModelConfiguration> {
        self.inner.get_computation_model_configuration()
    }
    ///
    /// Adds a key-value pair to `computationModelDataBinding`.
    ///
    /// To override the contents of this collection use [`set_computation_model_data_binding`](Self::set_computation_model_data_binding).
    ///
    /// <p>The data binding for the computation model. Key is a variable name defined in configuration. Value is a <code>ComputationModelDataBindingValue</code> referenced by the variable.</p>
    pub fn computation_model_data_binding(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: crate::types::ComputationModelDataBindingValue,
    ) -> Self {
        self.inner = self.inner.computation_model_data_binding(k.into(), v);
        self
    }
    /// <p>The data binding for the computation model. Key is a variable name defined in configuration. Value is a <code>ComputationModelDataBindingValue</code> referenced by the variable.</p>
    pub fn set_computation_model_data_binding(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::ComputationModelDataBindingValue>>,
    ) -> Self {
        self.inner = self.inner.set_computation_model_data_binding(input);
        self
    }
    /// <p>The data binding for the computation model. Key is a variable name defined in configuration. Value is a <code>ComputationModelDataBindingValue</code> referenced by the variable.</p>
    pub fn get_computation_model_data_binding(
        &self,
    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::ComputationModelDataBindingValue>> {
        self.inner.get_computation_model_data_binding()
    }
    /// <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. Don't reuse this client token if a new idempotent request is required.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. Don't reuse this client token if a new idempotent request is required.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>A unique case-sensitive identifier that you can provide to ensure the idempotency of the request. Don't reuse this client token if a new idempotent request is required.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
}
