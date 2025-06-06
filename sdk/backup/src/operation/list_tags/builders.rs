// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_tags::_list_tags_output::ListTagsOutputBuilder;

pub use crate::operation::list_tags::_list_tags_input::ListTagsInputBuilder;

impl crate::operation::list_tags::builders::ListTagsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_tags::ListTagsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_tags::ListTagsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_tags();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListTags`.
///
/// <p>Returns the tags assigned to the resource, such as a target recovery point, backup plan, or backup vault.</p>
/// <p>This operation returns results depending on the resource type used in the value for <code>resourceArn</code>. For example, recovery points of Amazon DynamoDB with Advanced Settings have an ARN (Amazon Resource Name) that begins with <code>arn:aws:backup</code>. Recovery points (backups) of DynamoDB without Advanced Settings enabled have an ARN that begins with <code>arn:aws:dynamodb</code>.</p>
/// <p>When this operation is called and when you include values of <code>resourceArn</code> that have an ARN other than <code>arn:aws:backup</code>, it may return one of the exceptions listed below. To prevent this exception, include only values representing resource types that are fully managed by Backup. These have an ARN that begins <code>arn:aws:backup</code> and they are noted in the <a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/backup-feature-availability.html#features-by-resource">Feature availability by resource</a> table.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListTagsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_tags::builders::ListTagsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl crate::client::customize::internal::CustomizableSend<crate::operation::list_tags::ListTagsOutput, crate::operation::list_tags::ListTagsError>
    for ListTagsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<crate::operation::list_tags::ListTagsOutput, crate::operation::list_tags::ListTagsError>,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListTagsFluentBuilder {
    /// Creates a new `ListTagsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListTags as a reference.
    pub fn as_input(&self) -> &crate::operation::list_tags::builders::ListTagsInputBuilder {
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
        crate::operation::list_tags::ListTagsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_tags::ListTagsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_tags::ListTags::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_tags::ListTags::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<crate::operation::list_tags::ListTagsOutput, crate::operation::list_tags::ListTagsError, Self>
    {
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_tags::paginator::ListTagsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_tags::paginator::ListTagsPaginator {
        crate::operation::list_tags::paginator::ListTagsPaginator::new(self.handle, self.inner)
    }
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a resource. The format of the ARN depends on the type of resource. Valid targets for <code>ListTags</code> are recovery points, backup plans, and backup vaults.</p>
    pub fn resource_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_arn(input.into());
        self
    }
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a resource. The format of the ARN depends on the type of resource. Valid targets for <code>ListTags</code> are recovery points, backup plans, and backup vaults.</p>
    pub fn set_resource_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_arn(input);
        self
    }
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies a resource. The format of the ARN depends on the type of resource. Valid targets for <code>ListTags</code> are recovery points, backup plans, and backup vaults.</p>
    pub fn get_resource_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resource_arn()
    }
    /// <p>The next item following a partial list of returned items. For example, if a request is made to return <code>MaxResults</code> number of items, <code>NextToken</code> allows you to return more items in your list starting at the location pointed to by the next token.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The next item following a partial list of returned items. For example, if a request is made to return <code>MaxResults</code> number of items, <code>NextToken</code> allows you to return more items in your list starting at the location pointed to by the next token.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The next item following a partial list of returned items. For example, if a request is made to return <code>MaxResults</code> number of items, <code>NextToken</code> allows you to return more items in your list starting at the location pointed to by the next token.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of items to be returned.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of items to be returned.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of items to be returned.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
}
