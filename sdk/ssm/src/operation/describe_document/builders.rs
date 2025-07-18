// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_document::_describe_document_output::DescribeDocumentOutputBuilder;

pub use crate::operation::describe_document::_describe_document_input::DescribeDocumentInputBuilder;

impl crate::operation::describe_document::builders::DescribeDocumentInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_document::DescribeDocumentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_document::DescribeDocumentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_document();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeDocument`.
///
/// <p>Describes the specified Amazon Web Services Systems Manager document (SSM document).</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeDocumentFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_document::builders::DescribeDocumentInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_document::DescribeDocumentOutput,
        crate::operation::describe_document::DescribeDocumentError,
    > for DescribeDocumentFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_document::DescribeDocumentOutput,
            crate::operation::describe_document::DescribeDocumentError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeDocumentFluentBuilder {
    /// Creates a new `DescribeDocumentFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeDocument as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_document::builders::DescribeDocumentInputBuilder {
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
        crate::operation::describe_document::DescribeDocumentOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_document::DescribeDocumentError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_document::DescribeDocument::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_document::DescribeDocument::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_document::DescribeDocumentOutput,
        crate::operation::describe_document::DescribeDocumentError,
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
    /// <p>The name of the SSM document.</p><note>
    /// <p>If you're calling a shared SSM document from a different Amazon Web Services account, <code>Name</code> is the full Amazon Resource Name (ARN) of the document.</p>
    /// </note>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the SSM document.</p><note>
    /// <p>If you're calling a shared SSM document from a different Amazon Web Services account, <code>Name</code> is the full Amazon Resource Name (ARN) of the document.</p>
    /// </note>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the SSM document.</p><note>
    /// <p>If you're calling a shared SSM document from a different Amazon Web Services account, <code>Name</code> is the full Amazon Resource Name (ARN) of the document.</p>
    /// </note>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>The document version for which you want information. Can be a specific version or the default version.</p>
    pub fn document_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.document_version(input.into());
        self
    }
    /// <p>The document version for which you want information. Can be a specific version or the default version.</p>
    pub fn set_document_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_document_version(input);
        self
    }
    /// <p>The document version for which you want information. Can be a specific version or the default version.</p>
    pub fn get_document_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_document_version()
    }
    /// <p>An optional field specifying the version of the artifact associated with the document. For example, 12.6. This value is unique across all versions of a document, and can't be changed.</p>
    pub fn version_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.version_name(input.into());
        self
    }
    /// <p>An optional field specifying the version of the artifact associated with the document. For example, 12.6. This value is unique across all versions of a document, and can't be changed.</p>
    pub fn set_version_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_version_name(input);
        self
    }
    /// <p>An optional field specifying the version of the artifact associated with the document. For example, 12.6. This value is unique across all versions of a document, and can't be changed.</p>
    pub fn get_version_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_version_name()
    }
}
