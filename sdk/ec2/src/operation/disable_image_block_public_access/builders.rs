// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disable_image_block_public_access::_disable_image_block_public_access_output::DisableImageBlockPublicAccessOutputBuilder;

pub use crate::operation::disable_image_block_public_access::_disable_image_block_public_access_input::DisableImageBlockPublicAccessInputBuilder;

impl crate::operation::disable_image_block_public_access::builders::DisableImageBlockPublicAccessInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::disable_image_block_public_access::DisableImageBlockPublicAccessOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::disable_image_block_public_access::DisableImageBlockPublicAccessError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.disable_image_block_public_access();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DisableImageBlockPublicAccess`.
///
/// <p>Disables <i>block public access for AMIs</i> at the account level in the specified Amazon Web Services Region. This removes the <i>block public access</i> restriction from your account. With the restriction removed, you can publicly share your AMIs in the specified Amazon Web Services Region.</p>
/// <p>The API can take up to 10 minutes to configure this setting. During this time, if you run <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_GetImageBlockPublicAccessState.html">GetImageBlockPublicAccessState</a>, the response will be <code>block-new-sharing</code>. When the API has completed the configuration, the response will be <code>unblocked</code>.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/block-public-access-to-amis.html">Block public access to your AMIs</a> in the <i>Amazon EC2 User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DisableImageBlockPublicAccessFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::disable_image_block_public_access::builders::DisableImageBlockPublicAccessInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::disable_image_block_public_access::DisableImageBlockPublicAccessOutput,
        crate::operation::disable_image_block_public_access::DisableImageBlockPublicAccessError,
    > for DisableImageBlockPublicAccessFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::disable_image_block_public_access::DisableImageBlockPublicAccessOutput,
            crate::operation::disable_image_block_public_access::DisableImageBlockPublicAccessError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DisableImageBlockPublicAccessFluentBuilder {
    /// Creates a new `DisableImageBlockPublicAccessFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DisableImageBlockPublicAccess as a reference.
    pub fn as_input(&self) -> &crate::operation::disable_image_block_public_access::builders::DisableImageBlockPublicAccessInputBuilder {
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
        crate::operation::disable_image_block_public_access::DisableImageBlockPublicAccessOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::disable_image_block_public_access::DisableImageBlockPublicAccessError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::disable_image_block_public_access::DisableImageBlockPublicAccess::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::disable_image_block_public_access::DisableImageBlockPublicAccess::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::disable_image_block_public_access::DisableImageBlockPublicAccessOutput,
        crate::operation::disable_image_block_public_access::DisableImageBlockPublicAccessError,
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
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
}
