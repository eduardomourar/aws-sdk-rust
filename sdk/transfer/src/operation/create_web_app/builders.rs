// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_web_app::_create_web_app_output::CreateWebAppOutputBuilder;

pub use crate::operation::create_web_app::_create_web_app_input::CreateWebAppInputBuilder;

impl crate::operation::create_web_app::builders::CreateWebAppInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_web_app::CreateWebAppOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_web_app::CreateWebAppError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_web_app();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateWebApp`.
///
/// <p>Creates a web app based on specified parameters, and returns the ID for the new web app.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateWebAppFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_web_app::builders::CreateWebAppInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_web_app::CreateWebAppOutput,
        crate::operation::create_web_app::CreateWebAppError,
    > for CreateWebAppFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_web_app::CreateWebAppOutput,
            crate::operation::create_web_app::CreateWebAppError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateWebAppFluentBuilder {
    /// Creates a new `CreateWebAppFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateWebApp as a reference.
    pub fn as_input(&self) -> &crate::operation::create_web_app::builders::CreateWebAppInputBuilder {
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
        crate::operation::create_web_app::CreateWebAppOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_web_app::CreateWebAppError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_web_app::CreateWebApp::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_web_app::CreateWebApp::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_web_app::CreateWebAppOutput,
        crate::operation::create_web_app::CreateWebAppError,
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
    /// <p>You can provide a structure that contains the details for the identity provider to use with your web app.</p>
    /// <p>For more details about this parameter, see <a href="https://docs.aws.amazon.com/transfer/latest/userguide/webapp-identity-center.html">Configure your identity provider for Transfer Family web apps</a>.</p>
    pub fn identity_provider_details(mut self, input: crate::types::WebAppIdentityProviderDetails) -> Self {
        self.inner = self.inner.identity_provider_details(input);
        self
    }
    /// <p>You can provide a structure that contains the details for the identity provider to use with your web app.</p>
    /// <p>For more details about this parameter, see <a href="https://docs.aws.amazon.com/transfer/latest/userguide/webapp-identity-center.html">Configure your identity provider for Transfer Family web apps</a>.</p>
    pub fn set_identity_provider_details(mut self, input: ::std::option::Option<crate::types::WebAppIdentityProviderDetails>) -> Self {
        self.inner = self.inner.set_identity_provider_details(input);
        self
    }
    /// <p>You can provide a structure that contains the details for the identity provider to use with your web app.</p>
    /// <p>For more details about this parameter, see <a href="https://docs.aws.amazon.com/transfer/latest/userguide/webapp-identity-center.html">Configure your identity provider for Transfer Family web apps</a>.</p>
    pub fn get_identity_provider_details(&self) -> &::std::option::Option<crate::types::WebAppIdentityProviderDetails> {
        self.inner.get_identity_provider_details()
    }
    /// <p>The <code>AccessEndpoint</code> is the URL that you provide to your users for them to interact with the Transfer Family web app. You can specify a custom URL or use the default value.</p>
    /// <p>Before you enter a custom URL for this parameter, follow the steps described in <a href="https://docs.aws.amazon.com/transfer/latest/userguide/webapp-customize.html">Update your access endpoint with a custom URL</a>.</p>
    pub fn access_endpoint(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.access_endpoint(input.into());
        self
    }
    /// <p>The <code>AccessEndpoint</code> is the URL that you provide to your users for them to interact with the Transfer Family web app. You can specify a custom URL or use the default value.</p>
    /// <p>Before you enter a custom URL for this parameter, follow the steps described in <a href="https://docs.aws.amazon.com/transfer/latest/userguide/webapp-customize.html">Update your access endpoint with a custom URL</a>.</p>
    pub fn set_access_endpoint(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_access_endpoint(input);
        self
    }
    /// <p>The <code>AccessEndpoint</code> is the URL that you provide to your users for them to interact with the Transfer Family web app. You can specify a custom URL or use the default value.</p>
    /// <p>Before you enter a custom URL for this parameter, follow the steps described in <a href="https://docs.aws.amazon.com/transfer/latest/userguide/webapp-customize.html">Update your access endpoint with a custom URL</a>.</p>
    pub fn get_access_endpoint(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_access_endpoint()
    }
    /// <p>A union that contains the value for number of concurrent connections or the user sessions on your web app.</p>
    pub fn web_app_units(mut self, input: crate::types::WebAppUnits) -> Self {
        self.inner = self.inner.web_app_units(input);
        self
    }
    /// <p>A union that contains the value for number of concurrent connections or the user sessions on your web app.</p>
    pub fn set_web_app_units(mut self, input: ::std::option::Option<crate::types::WebAppUnits>) -> Self {
        self.inner = self.inner.set_web_app_units(input);
        self
    }
    /// <p>A union that contains the value for number of concurrent connections or the user sessions on your web app.</p>
    pub fn get_web_app_units(&self) -> &::std::option::Option<crate::types::WebAppUnits> {
        self.inner.get_web_app_units()
    }
    ///
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Key-value pairs that can be used to group and search for web apps.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>Key-value pairs that can be used to group and search for web apps.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>Key-value pairs that can be used to group and search for web apps.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
    /// <p>Setting for the type of endpoint policy for the web app. The default value is <code>STANDARD</code>.</p>
    /// <p>If you are creating the web app in an Amazon Web Services GovCloud (US) Region, you can set this parameter to <code>FIPS</code>.</p>
    pub fn web_app_endpoint_policy(mut self, input: crate::types::WebAppEndpointPolicy) -> Self {
        self.inner = self.inner.web_app_endpoint_policy(input);
        self
    }
    /// <p>Setting for the type of endpoint policy for the web app. The default value is <code>STANDARD</code>.</p>
    /// <p>If you are creating the web app in an Amazon Web Services GovCloud (US) Region, you can set this parameter to <code>FIPS</code>.</p>
    pub fn set_web_app_endpoint_policy(mut self, input: ::std::option::Option<crate::types::WebAppEndpointPolicy>) -> Self {
        self.inner = self.inner.set_web_app_endpoint_policy(input);
        self
    }
    /// <p>Setting for the type of endpoint policy for the web app. The default value is <code>STANDARD</code>.</p>
    /// <p>If you are creating the web app in an Amazon Web Services GovCloud (US) Region, you can set this parameter to <code>FIPS</code>.</p>
    pub fn get_web_app_endpoint_policy(&self) -> &::std::option::Option<crate::types::WebAppEndpointPolicy> {
        self.inner.get_web_app_endpoint_policy()
    }
}
