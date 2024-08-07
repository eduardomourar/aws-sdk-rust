// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_flow_alias::_delete_flow_alias_output::DeleteFlowAliasOutputBuilder;

pub use crate::operation::delete_flow_alias::_delete_flow_alias_input::DeleteFlowAliasInputBuilder;

impl crate::operation::delete_flow_alias::builders::DeleteFlowAliasInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_flow_alias::DeleteFlowAliasOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_flow_alias::DeleteFlowAliasError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_flow_alias();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteFlowAlias`.
///
/// <p>Deletes an alias of a flow.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteFlowAliasFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_flow_alias::builders::DeleteFlowAliasInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_flow_alias::DeleteFlowAliasOutput,
        crate::operation::delete_flow_alias::DeleteFlowAliasError,
    > for DeleteFlowAliasFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_flow_alias::DeleteFlowAliasOutput,
            crate::operation::delete_flow_alias::DeleteFlowAliasError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteFlowAliasFluentBuilder {
    /// Creates a new `DeleteFlowAliasFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteFlowAlias as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_flow_alias::builders::DeleteFlowAliasInputBuilder {
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
        crate::operation::delete_flow_alias::DeleteFlowAliasOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_flow_alias::DeleteFlowAliasError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_flow_alias::DeleteFlowAlias::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_flow_alias::DeleteFlowAlias::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_flow_alias::DeleteFlowAliasOutput,
        crate::operation::delete_flow_alias::DeleteFlowAliasError,
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
    /// <p>The unique identifier of the flow that the alias belongs to.</p>
    pub fn flow_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.flow_identifier(input.into());
        self
    }
    /// <p>The unique identifier of the flow that the alias belongs to.</p>
    pub fn set_flow_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_flow_identifier(input);
        self
    }
    /// <p>The unique identifier of the flow that the alias belongs to.</p>
    pub fn get_flow_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_flow_identifier()
    }
    /// <p>The unique identifier of the alias to be deleted.</p>
    pub fn alias_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.alias_identifier(input.into());
        self
    }
    /// <p>The unique identifier of the alias to be deleted.</p>
    pub fn set_alias_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_alias_identifier(input);
        self
    }
    /// <p>The unique identifier of the alias to be deleted.</p>
    pub fn get_alias_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_alias_identifier()
    }
}
