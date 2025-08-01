// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_telemetry_rule::_delete_telemetry_rule_output::DeleteTelemetryRuleOutputBuilder;

pub use crate::operation::delete_telemetry_rule::_delete_telemetry_rule_input::DeleteTelemetryRuleInputBuilder;

impl crate::operation::delete_telemetry_rule::builders::DeleteTelemetryRuleInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_telemetry_rule::DeleteTelemetryRuleOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_telemetry_rule::DeleteTelemetryRuleError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_telemetry_rule();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteTelemetryRule`.
///
/// <p>Deletes a telemetry rule from your account. Any telemetry configurations previously created by the rule will remain but no new resources will be configured by this rule.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteTelemetryRuleFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_telemetry_rule::builders::DeleteTelemetryRuleInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_telemetry_rule::DeleteTelemetryRuleOutput,
        crate::operation::delete_telemetry_rule::DeleteTelemetryRuleError,
    > for DeleteTelemetryRuleFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_telemetry_rule::DeleteTelemetryRuleOutput,
            crate::operation::delete_telemetry_rule::DeleteTelemetryRuleError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteTelemetryRuleFluentBuilder {
    /// Creates a new `DeleteTelemetryRuleFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteTelemetryRule as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_telemetry_rule::builders::DeleteTelemetryRuleInputBuilder {
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
        crate::operation::delete_telemetry_rule::DeleteTelemetryRuleOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_telemetry_rule::DeleteTelemetryRuleError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_telemetry_rule::DeleteTelemetryRule::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_telemetry_rule::DeleteTelemetryRule::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_telemetry_rule::DeleteTelemetryRuleOutput,
        crate::operation::delete_telemetry_rule::DeleteTelemetryRuleError,
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
    /// <p>The identifier (name or ARN) of the telemetry rule to delete.</p>
    pub fn rule_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.rule_identifier(input.into());
        self
    }
    /// <p>The identifier (name or ARN) of the telemetry rule to delete.</p>
    pub fn set_rule_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_rule_identifier(input);
        self
    }
    /// <p>The identifier (name or ARN) of the telemetry rule to delete.</p>
    pub fn get_rule_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_rule_identifier()
    }
}
