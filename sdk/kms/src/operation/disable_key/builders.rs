// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disable_key::_disable_key_output::DisableKeyOutputBuilder;

pub use crate::operation::disable_key::_disable_key_input::DisableKeyInputBuilder;

impl crate::operation::disable_key::builders::DisableKeyInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::disable_key::DisableKeyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::disable_key::DisableKeyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.disable_key();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DisableKey`.
///
/// <p>Sets the state of a KMS key to disabled. This change temporarily prevents use of the KMS key for <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-cryptography.html#cryptographic-operations">cryptographic operations</a>.</p>
/// <p>The KMS key that you use for this operation must be in a compatible key state. For more information about how key state affects the use of a KMS key, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/key-state.html">Key states of KMS keys</a> in the <i> <i>Key Management Service Developer Guide</i> </i>.</p>
/// <p><b>Cross-account use</b>: No. You cannot perform this operation on a KMS key in a different Amazon Web Services account.</p>
/// <p><b>Required permissions</b>: <a href="https://docs.aws.amazon.com/kms/latest/developerguide/kms-api-permissions-reference.html">kms:DisableKey</a> (key policy)</p>
/// <p><b>Related operations</b>: <code>EnableKey</code></p>
/// <p><b>Eventual consistency</b>: The KMS API follows an eventual consistency model. For more information, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/accessing-kms.html#programming-eventual-consistency">KMS eventual consistency</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DisableKeyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::disable_key::builders::DisableKeyInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::disable_key::DisableKeyOutput,
        crate::operation::disable_key::DisableKeyError,
    > for DisableKeyFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::disable_key::DisableKeyOutput,
            crate::operation::disable_key::DisableKeyError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DisableKeyFluentBuilder {
    /// Creates a new `DisableKeyFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DisableKey as a reference.
    pub fn as_input(&self) -> &crate::operation::disable_key::builders::DisableKeyInputBuilder {
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
        crate::operation::disable_key::DisableKeyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::disable_key::DisableKeyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::disable_key::DisableKey::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::disable_key::DisableKey::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::disable_key::DisableKeyOutput,
        crate::operation::disable_key::DisableKeyError,
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
    /// <p>Identifies the KMS key to disable.</p>
    /// <p>Specify the key ID or key ARN of the KMS key.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li>
    /// <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>
    /// <li>
    /// <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p>
    pub fn key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.key_id(input.into());
        self
    }
    /// <p>Identifies the KMS key to disable.</p>
    /// <p>Specify the key ID or key ARN of the KMS key.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li>
    /// <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>
    /// <li>
    /// <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p>
    pub fn set_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_key_id(input);
        self
    }
    /// <p>Identifies the KMS key to disable.</p>
    /// <p>Specify the key ID or key ARN of the KMS key.</p>
    /// <p>For example:</p>
    /// <ul>
    /// <li>
    /// <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>
    /// <li>
    /// <p>Key ARN: <code>arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code></p></li>
    /// </ul>
    /// <p>To get the key ID and key ARN for a KMS key, use <code>ListKeys</code> or <code>DescribeKey</code>.</p>
    pub fn get_key_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_key_id()
    }
}
