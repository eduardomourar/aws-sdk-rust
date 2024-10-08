// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_asset_model::_delete_asset_model_output::DeleteAssetModelOutputBuilder;

pub use crate::operation::delete_asset_model::_delete_asset_model_input::DeleteAssetModelInputBuilder;

impl crate::operation::delete_asset_model::builders::DeleteAssetModelInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_asset_model::DeleteAssetModelOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_asset_model::DeleteAssetModelError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_asset_model();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteAssetModel`.
///
/// <p>Deletes an asset model. This action can't be undone. You must delete all assets created from an asset model before you can delete the model. Also, you can't delete an asset model if a parent asset model exists that contains a property formula expression that depends on the asset model that you want to delete. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/delete-assets-and-models.html">Deleting assets and models</a> in the <i>IoT SiteWise User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteAssetModelFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_asset_model::builders::DeleteAssetModelInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_asset_model::DeleteAssetModelOutput,
        crate::operation::delete_asset_model::DeleteAssetModelError,
    > for DeleteAssetModelFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_asset_model::DeleteAssetModelOutput,
            crate::operation::delete_asset_model::DeleteAssetModelError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteAssetModelFluentBuilder {
    /// Creates a new `DeleteAssetModelFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteAssetModel as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_asset_model::builders::DeleteAssetModelInputBuilder {
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
        crate::operation::delete_asset_model::DeleteAssetModelOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_asset_model::DeleteAssetModelError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_asset_model::DeleteAssetModel::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_asset_model::DeleteAssetModel::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_asset_model::DeleteAssetModelOutput,
        crate::operation::delete_asset_model::DeleteAssetModelError,
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
    /// <p>The ID of the asset model to delete. This can be either the actual ID in UUID format, or else <code>externalId:</code> followed by the external ID, if it has one. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/object-ids.html#external-id-references">Referencing objects with external IDs</a> in the <i>IoT SiteWise User Guide</i>.</p>
    pub fn asset_model_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.asset_model_id(input.into());
        self
    }
    /// <p>The ID of the asset model to delete. This can be either the actual ID in UUID format, or else <code>externalId:</code> followed by the external ID, if it has one. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/object-ids.html#external-id-references">Referencing objects with external IDs</a> in the <i>IoT SiteWise User Guide</i>.</p>
    pub fn set_asset_model_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_asset_model_id(input);
        self
    }
    /// <p>The ID of the asset model to delete. This can be either the actual ID in UUID format, or else <code>externalId:</code> followed by the external ID, if it has one. For more information, see <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/object-ids.html#external-id-references">Referencing objects with external IDs</a> in the <i>IoT SiteWise User Guide</i>.</p>
    pub fn get_asset_model_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_asset_model_id()
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
    /// <p>The expected current entity tag (ETag) for the asset model’s latest or active version (specified using <code>matchForVersionType</code>). The delete request is rejected if the tag does not match the latest or active version's current entity tag. See <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/opt-locking-for-model.html">Optimistic locking for asset model writes</a> in the <i>IoT SiteWise User Guide</i>.</p>
    pub fn if_match(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.if_match(input.into());
        self
    }
    /// <p>The expected current entity tag (ETag) for the asset model’s latest or active version (specified using <code>matchForVersionType</code>). The delete request is rejected if the tag does not match the latest or active version's current entity tag. See <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/opt-locking-for-model.html">Optimistic locking for asset model writes</a> in the <i>IoT SiteWise User Guide</i>.</p>
    pub fn set_if_match(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_if_match(input);
        self
    }
    /// <p>The expected current entity tag (ETag) for the asset model’s latest or active version (specified using <code>matchForVersionType</code>). The delete request is rejected if the tag does not match the latest or active version's current entity tag. See <a href="https://docs.aws.amazon.com/iot-sitewise/latest/userguide/opt-locking-for-model.html">Optimistic locking for asset model writes</a> in the <i>IoT SiteWise User Guide</i>.</p>
    pub fn get_if_match(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_if_match()
    }
    /// <p>Accepts <b>*</b> to reject the delete request if an active version (specified using <code>matchForVersionType</code> as <code>ACTIVE</code>) already exists for the asset model.</p>
    pub fn if_none_match(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.if_none_match(input.into());
        self
    }
    /// <p>Accepts <b>*</b> to reject the delete request if an active version (specified using <code>matchForVersionType</code> as <code>ACTIVE</code>) already exists for the asset model.</p>
    pub fn set_if_none_match(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_if_none_match(input);
        self
    }
    /// <p>Accepts <b>*</b> to reject the delete request if an active version (specified using <code>matchForVersionType</code> as <code>ACTIVE</code>) already exists for the asset model.</p>
    pub fn get_if_none_match(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_if_none_match()
    }
    /// <p>Specifies the asset model version type (<code>LATEST</code> or <code>ACTIVE</code>) used in conjunction with <code>If-Match</code> or <code>If-None-Match</code> headers to determine the target ETag for the delete operation.</p>
    pub fn match_for_version_type(mut self, input: crate::types::AssetModelVersionType) -> Self {
        self.inner = self.inner.match_for_version_type(input);
        self
    }
    /// <p>Specifies the asset model version type (<code>LATEST</code> or <code>ACTIVE</code>) used in conjunction with <code>If-Match</code> or <code>If-None-Match</code> headers to determine the target ETag for the delete operation.</p>
    pub fn set_match_for_version_type(mut self, input: ::std::option::Option<crate::types::AssetModelVersionType>) -> Self {
        self.inner = self.inner.set_match_for_version_type(input);
        self
    }
    /// <p>Specifies the asset model version type (<code>LATEST</code> or <code>ACTIVE</code>) used in conjunction with <code>If-Match</code> or <code>If-None-Match</code> headers to determine the target ETag for the delete operation.</p>
    pub fn get_match_for_version_type(&self) -> &::std::option::Option<crate::types::AssetModelVersionType> {
        self.inner.get_match_for_version_type()
    }
}
