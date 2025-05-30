// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_preferences::_update_preferences_output::UpdatePreferencesOutputBuilder;

pub use crate::operation::update_preferences::_update_preferences_input::UpdatePreferencesInputBuilder;

impl crate::operation::update_preferences::builders::UpdatePreferencesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_preferences::UpdatePreferencesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_preferences::UpdatePreferencesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_preferences();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdatePreferences`.
///
/// <p>Updates a set of preferences for an account in order to add account-specific preferences into the service. These preferences impact how the savings associated with recommendations are presented.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdatePreferencesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_preferences::builders::UpdatePreferencesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_preferences::UpdatePreferencesOutput,
        crate::operation::update_preferences::UpdatePreferencesError,
    > for UpdatePreferencesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_preferences::UpdatePreferencesOutput,
            crate::operation::update_preferences::UpdatePreferencesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdatePreferencesFluentBuilder {
    /// Creates a new `UpdatePreferencesFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdatePreferences as a reference.
    pub fn as_input(&self) -> &crate::operation::update_preferences::builders::UpdatePreferencesInputBuilder {
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
        crate::operation::update_preferences::UpdatePreferencesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_preferences::UpdatePreferencesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_preferences::UpdatePreferences::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_preferences::UpdatePreferences::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_preferences::UpdatePreferencesOutput,
        crate::operation::update_preferences::UpdatePreferencesError,
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
    /// <p>Sets the "savings estimation mode" preference.</p>
    pub fn savings_estimation_mode(mut self, input: crate::types::SavingsEstimationMode) -> Self {
        self.inner = self.inner.savings_estimation_mode(input);
        self
    }
    /// <p>Sets the "savings estimation mode" preference.</p>
    pub fn set_savings_estimation_mode(mut self, input: ::std::option::Option<crate::types::SavingsEstimationMode>) -> Self {
        self.inner = self.inner.set_savings_estimation_mode(input);
        self
    }
    /// <p>Sets the "savings estimation mode" preference.</p>
    pub fn get_savings_estimation_mode(&self) -> &::std::option::Option<crate::types::SavingsEstimationMode> {
        self.inner.get_savings_estimation_mode()
    }
    /// <p>Sets the "member account discount visibility" preference.</p>
    pub fn member_account_discount_visibility(mut self, input: crate::types::MemberAccountDiscountVisibility) -> Self {
        self.inner = self.inner.member_account_discount_visibility(input);
        self
    }
    /// <p>Sets the "member account discount visibility" preference.</p>
    pub fn set_member_account_discount_visibility(mut self, input: ::std::option::Option<crate::types::MemberAccountDiscountVisibility>) -> Self {
        self.inner = self.inner.set_member_account_discount_visibility(input);
        self
    }
    /// <p>Sets the "member account discount visibility" preference.</p>
    pub fn get_member_account_discount_visibility(&self) -> &::std::option::Option<crate::types::MemberAccountDiscountVisibility> {
        self.inner.get_member_account_discount_visibility()
    }
    /// <p>Sets the preferences for how Reserved Instances and Savings Plans cost-saving opportunities are prioritized in terms of payment option and term length.</p>
    pub fn preferred_commitment(mut self, input: crate::types::PreferredCommitment) -> Self {
        self.inner = self.inner.preferred_commitment(input);
        self
    }
    /// <p>Sets the preferences for how Reserved Instances and Savings Plans cost-saving opportunities are prioritized in terms of payment option and term length.</p>
    pub fn set_preferred_commitment(mut self, input: ::std::option::Option<crate::types::PreferredCommitment>) -> Self {
        self.inner = self.inner.set_preferred_commitment(input);
        self
    }
    /// <p>Sets the preferences for how Reserved Instances and Savings Plans cost-saving opportunities are prioritized in terms of payment option and term length.</p>
    pub fn get_preferred_commitment(&self) -> &::std::option::Option<crate::types::PreferredCommitment> {
        self.inner.get_preferred_commitment()
    }
}
