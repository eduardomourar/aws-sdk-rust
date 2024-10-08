// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_buyer_dashboard::_get_buyer_dashboard_output::GetBuyerDashboardOutputBuilder;

pub use crate::operation::get_buyer_dashboard::_get_buyer_dashboard_input::GetBuyerDashboardInputBuilder;

impl crate::operation::get_buyer_dashboard::builders::GetBuyerDashboardInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_buyer_dashboard::GetBuyerDashboardOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_buyer_dashboard::GetBuyerDashboardError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_buyer_dashboard();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetBuyerDashboard`.
///
/// <p>Generates an embedding URL for an Amazon QuickSight dashboard for an anonymous user.</p><note>
/// <p>This API is available only to Amazon Web Services Organization management accounts or delegated administrators registered for the procurement insights (<code>procurement-insights.marketplace.amazonaws.com</code>) feature.</p>
/// </note>
/// <p>The following rules apply to a generated URL:</p>
/// <ul>
/// <li>
/// <p>It contains a temporary bearer token, valid for 5 minutes after it is generated. Once redeemed within that period, it cannot be re-used again.</p></li>
/// <li>
/// <p>It has a session lifetime of one hour. The 5-minute validity period runs separately from the session lifetime.</p></li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetBuyerDashboardFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_buyer_dashboard::builders::GetBuyerDashboardInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_buyer_dashboard::GetBuyerDashboardOutput,
        crate::operation::get_buyer_dashboard::GetBuyerDashboardError,
    > for GetBuyerDashboardFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_buyer_dashboard::GetBuyerDashboardOutput,
            crate::operation::get_buyer_dashboard::GetBuyerDashboardError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetBuyerDashboardFluentBuilder {
    /// Creates a new `GetBuyerDashboardFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetBuyerDashboard as a reference.
    pub fn as_input(&self) -> &crate::operation::get_buyer_dashboard::builders::GetBuyerDashboardInputBuilder {
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
        crate::operation::get_buyer_dashboard::GetBuyerDashboardOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_buyer_dashboard::GetBuyerDashboardError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_buyer_dashboard::GetBuyerDashboard::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_buyer_dashboard::GetBuyerDashboard::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_buyer_dashboard::GetBuyerDashboardOutput,
        crate::operation::get_buyer_dashboard::GetBuyerDashboardError,
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
    /// <p>The ARN of the requested dashboard.</p>
    pub fn dashboard_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.dashboard_identifier(input.into());
        self
    }
    /// <p>The ARN of the requested dashboard.</p>
    pub fn set_dashboard_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_dashboard_identifier(input);
        self
    }
    /// <p>The ARN of the requested dashboard.</p>
    pub fn get_dashboard_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_dashboard_identifier()
    }
    ///
    /// Appends an item to `embeddingDomains`.
    ///
    /// To override the contents of this collection use [`set_embedding_domains`](Self::set_embedding_domains).
    ///
    /// <p>Fully qualified domains that you add to the allow list for access to the generated URL that is then embedded. You can list up to two domains or subdomains in each API call. To include all subdomains under a specific domain, use <code>*</code>. For example, <code>https://*.amazon.com</code> includes all subdomains under <code>https://aws.amazon.com</code>.</p>
    pub fn embedding_domains(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.embedding_domains(input.into());
        self
    }
    /// <p>Fully qualified domains that you add to the allow list for access to the generated URL that is then embedded. You can list up to two domains or subdomains in each API call. To include all subdomains under a specific domain, use <code>*</code>. For example, <code>https://*.amazon.com</code> includes all subdomains under <code>https://aws.amazon.com</code>.</p>
    pub fn set_embedding_domains(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_embedding_domains(input);
        self
    }
    /// <p>Fully qualified domains that you add to the allow list for access to the generated URL that is then embedded. You can list up to two domains or subdomains in each API call. To include all subdomains under a specific domain, use <code>*</code>. For example, <code>https://*.amazon.com</code> includes all subdomains under <code>https://aws.amazon.com</code>.</p>
    pub fn get_embedding_domains(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_embedding_domains()
    }
}
