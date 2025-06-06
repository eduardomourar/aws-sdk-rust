// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_sink_policy::_put_sink_policy_output::PutSinkPolicyOutputBuilder;

pub use crate::operation::put_sink_policy::_put_sink_policy_input::PutSinkPolicyInputBuilder;

impl crate::operation::put_sink_policy::builders::PutSinkPolicyInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::put_sink_policy::PutSinkPolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_sink_policy::PutSinkPolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.put_sink_policy();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PutSinkPolicy`.
///
/// <p>Creates or updates the resource policy that grants permissions to source accounts to link to the monitoring account sink. When you create a sink policy, you can grant permissions to all accounts in an organization or to individual accounts.</p>
/// <p>You can also use a sink policy to limit the types of data that is shared. The six types of services with their respective resource types that you can allow or deny are:</p>
/// <ul>
/// <li>
/// <p><b>Metrics</b> - Specify with <code>AWS::CloudWatch::Metric</code></p></li>
/// <li>
/// <p><b>Log groups</b> - Specify with <code>AWS::Logs::LogGroup</code></p></li>
/// <li>
/// <p><b>Traces</b> - Specify with <code>AWS::XRay::Trace</code></p></li>
/// <li>
/// <p><b>Application Insights - Applications</b> - Specify with <code>AWS::ApplicationInsights::Application</code></p></li>
/// <li>
/// <p><b>Internet Monitor</b> - Specify with <code>AWS::InternetMonitor::Monitor</code></p></li>
/// <li>
/// <p><b>Application Signals</b> - Specify with <code>AWS::ApplicationSignals::Service</code> and <code>AWS::ApplicationSignals::ServiceLevelObjective</code></p></li>
/// </ul>
/// <p>See the examples in this section to see how to specify permitted source accounts and data types.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutSinkPolicyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_sink_policy::builders::PutSinkPolicyInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::put_sink_policy::PutSinkPolicyOutput,
        crate::operation::put_sink_policy::PutSinkPolicyError,
    > for PutSinkPolicyFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::put_sink_policy::PutSinkPolicyOutput,
            crate::operation::put_sink_policy::PutSinkPolicyError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl PutSinkPolicyFluentBuilder {
    /// Creates a new `PutSinkPolicyFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PutSinkPolicy as a reference.
    pub fn as_input(&self) -> &crate::operation::put_sink_policy::builders::PutSinkPolicyInputBuilder {
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
        crate::operation::put_sink_policy::PutSinkPolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_sink_policy::PutSinkPolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::put_sink_policy::PutSinkPolicy::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::put_sink_policy::PutSinkPolicy::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::put_sink_policy::PutSinkPolicyOutput,
        crate::operation::put_sink_policy::PutSinkPolicyError,
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
    /// <p>The ARN of the sink to attach this policy to.</p>
    pub fn sink_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.sink_identifier(input.into());
        self
    }
    /// <p>The ARN of the sink to attach this policy to.</p>
    pub fn set_sink_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_sink_identifier(input);
        self
    }
    /// <p>The ARN of the sink to attach this policy to.</p>
    pub fn get_sink_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_sink_identifier()
    }
    /// <p>The JSON policy to use. If you are updating an existing policy, the entire existing policy is replaced by what you specify here.</p>
    /// <p>The policy must be in JSON string format with quotation marks escaped and no newlines.</p>
    /// <p>For examples of different types of policies, see the <b>Examples</b> section on this page.</p>
    pub fn policy(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.policy(input.into());
        self
    }
    /// <p>The JSON policy to use. If you are updating an existing policy, the entire existing policy is replaced by what you specify here.</p>
    /// <p>The policy must be in JSON string format with quotation marks escaped and no newlines.</p>
    /// <p>For examples of different types of policies, see the <b>Examples</b> section on this page.</p>
    pub fn set_policy(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_policy(input);
        self
    }
    /// <p>The JSON policy to use. If you are updating an existing policy, the entire existing policy is replaced by what you specify here.</p>
    /// <p>The policy must be in JSON string format with quotation marks escaped and no newlines.</p>
    /// <p>For examples of different types of policies, see the <b>Examples</b> section on this page.</p>
    pub fn get_policy(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_policy()
    }
}
