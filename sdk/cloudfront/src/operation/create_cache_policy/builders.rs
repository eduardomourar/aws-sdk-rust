// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_cache_policy::_create_cache_policy_output::CreateCachePolicyOutputBuilder;

pub use crate::operation::create_cache_policy::_create_cache_policy_input::CreateCachePolicyInputBuilder;

impl crate::operation::create_cache_policy::builders::CreateCachePolicyInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_cache_policy::CreateCachePolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_cache_policy::CreateCachePolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_cache_policy();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateCachePolicy`.
///
/// <p>Creates a cache policy.</p>
/// <p>After you create a cache policy, you can attach it to one or more cache behaviors. When it's attached to a cache behavior, the cache policy determines the following:</p>
/// <ul>
/// <li>
/// <p>The values that CloudFront includes in the <i>cache key</i>. These values can include HTTP headers, cookies, and URL query strings. CloudFront uses the cache key to find an object in its cache that it can return to the viewer.</p></li>
/// <li>
/// <p>The default, minimum, and maximum time to live (TTL) values that you want objects to stay in the CloudFront cache.</p><important>
/// <p>If your minimum TTL is greater than 0, CloudFront will cache content for at least the duration specified in the cache policy's minimum TTL, even if the <code>Cache-Control: no-cache</code>, <code>no-store</code>, or <code>private</code> directives are present in the origin headers.</p>
/// </important></li>
/// </ul>
/// <p>The headers, cookies, and query strings that are included in the cache key are also included in requests that CloudFront sends to the origin. CloudFront sends a request when it can't find an object in its cache that matches the request's cache key. If you want to send values to the origin but <i>not</i> include them in the cache key, use <code>OriginRequestPolicy</code>.</p>
/// <p>For more information about cache policies, see <a href="https://docs.aws.amazon.com/AmazonCloudFront/latest/DeveloperGuide/controlling-the-cache-key.html">Controlling the cache key</a> in the <i>Amazon CloudFront Developer Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateCachePolicyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_cache_policy::builders::CreateCachePolicyInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_cache_policy::CreateCachePolicyOutput,
        crate::operation::create_cache_policy::CreateCachePolicyError,
    > for CreateCachePolicyFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_cache_policy::CreateCachePolicyOutput,
            crate::operation::create_cache_policy::CreateCachePolicyError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateCachePolicyFluentBuilder {
    /// Creates a new `CreateCachePolicyFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateCachePolicy as a reference.
    pub fn as_input(&self) -> &crate::operation::create_cache_policy::builders::CreateCachePolicyInputBuilder {
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
        crate::operation::create_cache_policy::CreateCachePolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_cache_policy::CreateCachePolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_cache_policy::CreateCachePolicy::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_cache_policy::CreateCachePolicy::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_cache_policy::CreateCachePolicyOutput,
        crate::operation::create_cache_policy::CreateCachePolicyError,
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
    /// <p>A cache policy configuration.</p>
    pub fn cache_policy_config(mut self, input: crate::types::CachePolicyConfig) -> Self {
        self.inner = self.inner.cache_policy_config(input);
        self
    }
    /// <p>A cache policy configuration.</p>
    pub fn set_cache_policy_config(mut self, input: ::std::option::Option<crate::types::CachePolicyConfig>) -> Self {
        self.inner = self.inner.set_cache_policy_config(input);
        self
    }
    /// <p>A cache policy configuration.</p>
    pub fn get_cache_policy_config(&self) -> &::std::option::Option<crate::types::CachePolicyConfig> {
        self.inner.get_cache_policy_config()
    }
}
