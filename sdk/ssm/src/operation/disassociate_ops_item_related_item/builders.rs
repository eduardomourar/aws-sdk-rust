// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disassociate_ops_item_related_item::_disassociate_ops_item_related_item_output::DisassociateOpsItemRelatedItemOutputBuilder;

pub use crate::operation::disassociate_ops_item_related_item::_disassociate_ops_item_related_item_input::DisassociateOpsItemRelatedItemInputBuilder;

impl crate::operation::disassociate_ops_item_related_item::builders::DisassociateOpsItemRelatedItemInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::disassociate_ops_item_related_item::DisassociateOpsItemRelatedItemOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::disassociate_ops_item_related_item::DisassociateOpsItemRelatedItemError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.disassociate_ops_item_related_item();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DisassociateOpsItemRelatedItem`.
///
/// <p>Deletes the association between an OpsItem and a related item. For example, this API operation can delete an Incident Manager incident from an OpsItem. Incident Manager is a tool in Amazon Web Services Systems Manager.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DisassociateOpsItemRelatedItemFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::disassociate_ops_item_related_item::builders::DisassociateOpsItemRelatedItemInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::disassociate_ops_item_related_item::DisassociateOpsItemRelatedItemOutput,
        crate::operation::disassociate_ops_item_related_item::DisassociateOpsItemRelatedItemError,
    > for DisassociateOpsItemRelatedItemFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::disassociate_ops_item_related_item::DisassociateOpsItemRelatedItemOutput,
            crate::operation::disassociate_ops_item_related_item::DisassociateOpsItemRelatedItemError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DisassociateOpsItemRelatedItemFluentBuilder {
    /// Creates a new `DisassociateOpsItemRelatedItemFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DisassociateOpsItemRelatedItem as a reference.
    pub fn as_input(&self) -> &crate::operation::disassociate_ops_item_related_item::builders::DisassociateOpsItemRelatedItemInputBuilder {
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
        crate::operation::disassociate_ops_item_related_item::DisassociateOpsItemRelatedItemOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::disassociate_ops_item_related_item::DisassociateOpsItemRelatedItemError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::disassociate_ops_item_related_item::DisassociateOpsItemRelatedItem::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::disassociate_ops_item_related_item::DisassociateOpsItemRelatedItem::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::disassociate_ops_item_related_item::DisassociateOpsItemRelatedItemOutput,
        crate::operation::disassociate_ops_item_related_item::DisassociateOpsItemRelatedItemError,
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
    /// <p>The ID of the OpsItem for which you want to delete an association between the OpsItem and a related item.</p>
    pub fn ops_item_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.ops_item_id(input.into());
        self
    }
    /// <p>The ID of the OpsItem for which you want to delete an association between the OpsItem and a related item.</p>
    pub fn set_ops_item_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_ops_item_id(input);
        self
    }
    /// <p>The ID of the OpsItem for which you want to delete an association between the OpsItem and a related item.</p>
    pub fn get_ops_item_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_ops_item_id()
    }
    /// <p>The ID of the association for which you want to delete an association between the OpsItem and a related item.</p>
    pub fn association_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.association_id(input.into());
        self
    }
    /// <p>The ID of the association for which you want to delete an association between the OpsItem and a related item.</p>
    pub fn set_association_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_association_id(input);
        self
    }
    /// <p>The ID of the association for which you want to delete an association between the OpsItem and a related item.</p>
    pub fn get_association_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_association_id()
    }
}
