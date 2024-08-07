// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::add_policy_statement::_add_policy_statement_output::AddPolicyStatementOutputBuilder;

pub use crate::operation::add_policy_statement::_add_policy_statement_input::AddPolicyStatementInputBuilder;

impl crate::operation::add_policy_statement::builders::AddPolicyStatementInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::add_policy_statement::AddPolicyStatementOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::add_policy_statement::AddPolicyStatementError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.add_policy_statement();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AddPolicyStatement`.
///
/// <p>Adds a policy statement object. To retrieve a list of existing policy statements, use the <code>GetPolicy</code> API.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AddPolicyStatementFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::add_policy_statement::builders::AddPolicyStatementInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::add_policy_statement::AddPolicyStatementOutput,
        crate::operation::add_policy_statement::AddPolicyStatementError,
    > for AddPolicyStatementFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::add_policy_statement::AddPolicyStatementOutput,
            crate::operation::add_policy_statement::AddPolicyStatementError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AddPolicyStatementFluentBuilder {
    /// Creates a new `AddPolicyStatementFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AddPolicyStatement as a reference.
    pub fn as_input(&self) -> &crate::operation::add_policy_statement::builders::AddPolicyStatementInputBuilder {
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
        crate::operation::add_policy_statement::AddPolicyStatementOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::add_policy_statement::AddPolicyStatementError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::add_policy_statement::AddPolicyStatement::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::add_policy_statement::AddPolicyStatement::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::add_policy_statement::AddPolicyStatementOutput,
        crate::operation::add_policy_statement::AddPolicyStatementError,
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
    /// <p>The Amazon Resource Name (ARN) of the resource that will be accessed by the principal.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the resource that will be accessed by the principal.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the resource that will be accessed by the principal.</p>
    pub fn get_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_arn()
    }
    /// <p>A statement identifier that differentiates the statement from others in the same policy.</p>
    pub fn statement_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.statement_id(input.into());
        self
    }
    /// <p>A statement identifier that differentiates the statement from others in the same policy.</p>
    pub fn set_statement_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_statement_id(input);
        self
    }
    /// <p>A statement identifier that differentiates the statement from others in the same policy.</p>
    pub fn get_statement_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_statement_id()
    }
    /// <p>Determines whether the permissions specified in the policy are to be allowed (<code>Allow</code>) or denied (<code>Deny</code>).</p><important>
    /// <p>If you set the value of the <code>effect</code> parameter to <code>Deny</code> for the <code>AddPolicyStatement</code> operation, you must also set the value of the <code>effect</code> parameter in the <code>policy</code> to <code>Deny</code> for the <code>PutPolicy</code> operation.</p>
    /// </important>
    pub fn effect(mut self, input: crate::types::StatementEffect) -> Self {
        self.inner = self.inner.effect(input);
        self
    }
    /// <p>Determines whether the permissions specified in the policy are to be allowed (<code>Allow</code>) or denied (<code>Deny</code>).</p><important>
    /// <p>If you set the value of the <code>effect</code> parameter to <code>Deny</code> for the <code>AddPolicyStatement</code> operation, you must also set the value of the <code>effect</code> parameter in the <code>policy</code> to <code>Deny</code> for the <code>PutPolicy</code> operation.</p>
    /// </important>
    pub fn set_effect(mut self, input: ::std::option::Option<crate::types::StatementEffect>) -> Self {
        self.inner = self.inner.set_effect(input);
        self
    }
    /// <p>Determines whether the permissions specified in the policy are to be allowed (<code>Allow</code>) or denied (<code>Deny</code>).</p><important>
    /// <p>If you set the value of the <code>effect</code> parameter to <code>Deny</code> for the <code>AddPolicyStatement</code> operation, you must also set the value of the <code>effect</code> parameter in the <code>policy</code> to <code>Deny</code> for the <code>PutPolicy</code> operation.</p>
    /// </important>
    pub fn get_effect(&self) -> &::std::option::Option<crate::types::StatementEffect> {
        self.inner.get_effect()
    }
    ///
    /// Appends an item to `action`.
    ///
    /// To override the contents of this collection use [`set_action`](Self::set_action).
    ///
    /// <p>The action that the principal can use on the resource.</p>
    /// <p>For example, <code>entityresolution:GetIdMappingJob</code>, <code>entityresolution:GetMatchingJob</code>.</p>
    pub fn action(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.action(input.into());
        self
    }
    /// <p>The action that the principal can use on the resource.</p>
    /// <p>For example, <code>entityresolution:GetIdMappingJob</code>, <code>entityresolution:GetMatchingJob</code>.</p>
    pub fn set_action(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_action(input);
        self
    }
    /// <p>The action that the principal can use on the resource.</p>
    /// <p>For example, <code>entityresolution:GetIdMappingJob</code>, <code>entityresolution:GetMatchingJob</code>.</p>
    pub fn get_action(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_action()
    }
    ///
    /// Appends an item to `principal`.
    ///
    /// To override the contents of this collection use [`set_principal`](Self::set_principal).
    ///
    /// <p>The Amazon Web Services service or Amazon Web Services account that can access the resource defined as ARN.</p>
    pub fn principal(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.principal(input.into());
        self
    }
    /// <p>The Amazon Web Services service or Amazon Web Services account that can access the resource defined as ARN.</p>
    pub fn set_principal(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_principal(input);
        self
    }
    /// <p>The Amazon Web Services service or Amazon Web Services account that can access the resource defined as ARN.</p>
    pub fn get_principal(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_principal()
    }
    /// <p>A set of condition keys that you can use in key policies.</p>
    pub fn condition(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.condition(input.into());
        self
    }
    /// <p>A set of condition keys that you can use in key policies.</p>
    pub fn set_condition(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_condition(input);
        self
    }
    /// <p>A set of condition keys that you can use in key policies.</p>
    pub fn get_condition(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_condition()
    }
}
