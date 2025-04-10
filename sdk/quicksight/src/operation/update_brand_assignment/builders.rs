// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_brand_assignment::_update_brand_assignment_output::UpdateBrandAssignmentOutputBuilder;

pub use crate::operation::update_brand_assignment::_update_brand_assignment_input::UpdateBrandAssignmentInputBuilder;

impl crate::operation::update_brand_assignment::builders::UpdateBrandAssignmentInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_brand_assignment::UpdateBrandAssignmentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_brand_assignment::UpdateBrandAssignmentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_brand_assignment();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateBrandAssignment`.
///
/// <p>Updates a brand assignment.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateBrandAssignmentFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_brand_assignment::builders::UpdateBrandAssignmentInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_brand_assignment::UpdateBrandAssignmentOutput,
        crate::operation::update_brand_assignment::UpdateBrandAssignmentError,
    > for UpdateBrandAssignmentFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_brand_assignment::UpdateBrandAssignmentOutput,
            crate::operation::update_brand_assignment::UpdateBrandAssignmentError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateBrandAssignmentFluentBuilder {
    /// Creates a new `UpdateBrandAssignmentFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateBrandAssignment as a reference.
    pub fn as_input(&self) -> &crate::operation::update_brand_assignment::builders::UpdateBrandAssignmentInputBuilder {
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
        crate::operation::update_brand_assignment::UpdateBrandAssignmentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_brand_assignment::UpdateBrandAssignmentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_brand_assignment::UpdateBrandAssignment::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_brand_assignment::UpdateBrandAssignment::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_brand_assignment::UpdateBrandAssignmentOutput,
        crate::operation::update_brand_assignment::UpdateBrandAssignmentError,
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
    /// <p>The ID of the Amazon Web Services account that owns the brand assignment.</p>
    pub fn aws_account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.aws_account_id(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services account that owns the brand assignment.</p>
    pub fn set_aws_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_aws_account_id(input);
        self
    }
    /// <p>The ID of the Amazon Web Services account that owns the brand assignment.</p>
    pub fn get_aws_account_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_aws_account_id()
    }
    /// <p>The Amazon Resource Name (ARN) of the brand.</p>
    pub fn brand_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.brand_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the brand.</p>
    pub fn set_brand_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_brand_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the brand.</p>
    pub fn get_brand_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_brand_arn()
    }
}
