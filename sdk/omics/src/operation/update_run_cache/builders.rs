// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_run_cache::_update_run_cache_output::UpdateRunCacheOutputBuilder;

pub use crate::operation::update_run_cache::_update_run_cache_input::UpdateRunCacheInputBuilder;

impl crate::operation::update_run_cache::builders::UpdateRunCacheInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_run_cache::UpdateRunCacheOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_run_cache::UpdateRunCacheError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_run_cache();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateRunCache`.
///
/// <p>Updates a run cache using its ID and returns a response with no body if the operation is successful. You can update the run cache description, name, or the default run cache behavior with <code>CACHE_ON_FAILURE</code> or <code>CACHE_ALWAYS</code>. To confirm that your run cache settings have been properly updated, use the <code>GetRunCache</code> API operation.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/omics/latest/dev/how-run-cache.html">How call caching works</a> in the <i>Amazon Web Services HealthOmics User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateRunCacheFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_run_cache::builders::UpdateRunCacheInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_run_cache::UpdateRunCacheOutput,
        crate::operation::update_run_cache::UpdateRunCacheError,
    > for UpdateRunCacheFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_run_cache::UpdateRunCacheOutput,
            crate::operation::update_run_cache::UpdateRunCacheError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateRunCacheFluentBuilder {
    /// Creates a new `UpdateRunCacheFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateRunCache as a reference.
    pub fn as_input(&self) -> &crate::operation::update_run_cache::builders::UpdateRunCacheInputBuilder {
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
        crate::operation::update_run_cache::UpdateRunCacheOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_run_cache::UpdateRunCacheError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_run_cache::UpdateRunCache::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_run_cache::UpdateRunCache::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_run_cache::UpdateRunCacheOutput,
        crate::operation::update_run_cache::UpdateRunCacheError,
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
    /// <p>Update the default run cache behavior.</p>
    pub fn cache_behavior(mut self, input: crate::types::CacheBehavior) -> Self {
        self.inner = self.inner.cache_behavior(input);
        self
    }
    /// <p>Update the default run cache behavior.</p>
    pub fn set_cache_behavior(mut self, input: ::std::option::Option<crate::types::CacheBehavior>) -> Self {
        self.inner = self.inner.set_cache_behavior(input);
        self
    }
    /// <p>Update the default run cache behavior.</p>
    pub fn get_cache_behavior(&self) -> &::std::option::Option<crate::types::CacheBehavior> {
        self.inner.get_cache_behavior()
    }
    /// <p>Update the run cache description.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>Update the run cache description.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>Update the run cache description.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>The identifier of the run cache you want to update.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The identifier of the run cache you want to update.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>The identifier of the run cache you want to update.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_id()
    }
    /// <p>Update the name of the run cache.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>Update the name of the run cache.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>Update the name of the run cache.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
}
