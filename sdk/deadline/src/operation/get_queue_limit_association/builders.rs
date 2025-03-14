// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_queue_limit_association::_get_queue_limit_association_output::GetQueueLimitAssociationOutputBuilder;

pub use crate::operation::get_queue_limit_association::_get_queue_limit_association_input::GetQueueLimitAssociationInputBuilder;

impl crate::operation::get_queue_limit_association::builders::GetQueueLimitAssociationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_queue_limit_association::GetQueueLimitAssociationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_queue_limit_association::GetQueueLimitAssociationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_queue_limit_association();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetQueueLimitAssociation`.
///
/// <p>Gets information about a specific association between a queue and a limit.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetQueueLimitAssociationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_queue_limit_association::builders::GetQueueLimitAssociationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_queue_limit_association::GetQueueLimitAssociationOutput,
        crate::operation::get_queue_limit_association::GetQueueLimitAssociationError,
    > for GetQueueLimitAssociationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_queue_limit_association::GetQueueLimitAssociationOutput,
            crate::operation::get_queue_limit_association::GetQueueLimitAssociationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetQueueLimitAssociationFluentBuilder {
    /// Creates a new `GetQueueLimitAssociationFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetQueueLimitAssociation as a reference.
    pub fn as_input(&self) -> &crate::operation::get_queue_limit_association::builders::GetQueueLimitAssociationInputBuilder {
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
        crate::operation::get_queue_limit_association::GetQueueLimitAssociationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_queue_limit_association::GetQueueLimitAssociationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_queue_limit_association::GetQueueLimitAssociation::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_queue_limit_association::GetQueueLimitAssociation::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_queue_limit_association::GetQueueLimitAssociationOutput,
        crate::operation::get_queue_limit_association::GetQueueLimitAssociationError,
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
    /// <p>The unique identifier of the farm that contains the associated queue and limit.</p>
    pub fn farm_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.farm_id(input.into());
        self
    }
    /// <p>The unique identifier of the farm that contains the associated queue and limit.</p>
    pub fn set_farm_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_farm_id(input);
        self
    }
    /// <p>The unique identifier of the farm that contains the associated queue and limit.</p>
    pub fn get_farm_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_farm_id()
    }
    /// <p>The unique identifier of the queue associated with the limit.</p>
    pub fn queue_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.queue_id(input.into());
        self
    }
    /// <p>The unique identifier of the queue associated with the limit.</p>
    pub fn set_queue_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_queue_id(input);
        self
    }
    /// <p>The unique identifier of the queue associated with the limit.</p>
    pub fn get_queue_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_queue_id()
    }
    /// <p>The unique identifier of the limit associated with the queue.</p>
    pub fn limit_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.limit_id(input.into());
        self
    }
    /// <p>The unique identifier of the limit associated with the queue.</p>
    pub fn set_limit_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_limit_id(input);
        self
    }
    /// <p>The unique identifier of the limit associated with the queue.</p>
    pub fn get_limit_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_limit_id()
    }
}
