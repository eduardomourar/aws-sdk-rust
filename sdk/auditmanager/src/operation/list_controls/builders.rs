// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_controls::_list_controls_output::ListControlsOutputBuilder;

pub use crate::operation::list_controls::_list_controls_input::ListControlsInputBuilder;

impl crate::operation::list_controls::builders::ListControlsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_controls::ListControlsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_controls::ListControlsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_controls();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListControls`.
///
/// <p>Returns a list of controls from Audit Manager.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListControlsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_controls::builders::ListControlsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_controls::ListControlsOutput,
        crate::operation::list_controls::ListControlsError,
    > for ListControlsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_controls::ListControlsOutput,
            crate::operation::list_controls::ListControlsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListControlsFluentBuilder {
    /// Creates a new `ListControlsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListControls as a reference.
    pub fn as_input(&self) -> &crate::operation::list_controls::builders::ListControlsInputBuilder {
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
        crate::operation::list_controls::ListControlsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_controls::ListControlsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_controls::ListControls::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_controls::ListControls::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_controls::ListControlsOutput,
        crate::operation::list_controls::ListControlsError,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_controls::paginator::ListControlsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_controls::paginator::ListControlsPaginator {
        crate::operation::list_controls::paginator::ListControlsPaginator::new(self.handle, self.inner)
    }
    /// <p>A filter that narrows the list of controls to a specific type.</p>
    pub fn control_type(mut self, input: crate::types::ControlType) -> Self {
        self.inner = self.inner.control_type(input);
        self
    }
    /// <p>A filter that narrows the list of controls to a specific type.</p>
    pub fn set_control_type(mut self, input: ::std::option::Option<crate::types::ControlType>) -> Self {
        self.inner = self.inner.set_control_type(input);
        self
    }
    /// <p>A filter that narrows the list of controls to a specific type.</p>
    pub fn get_control_type(&self) -> &::std::option::Option<crate::types::ControlType> {
        self.inner.get_control_type()
    }
    /// <p>The pagination token that's used to fetch the next set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The pagination token that's used to fetch the next set of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The pagination token that's used to fetch the next set of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of results on a page or for an API request call.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results on a page or for an API request call.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results on a page or for an API request call.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>A filter that narrows the list of controls to a specific resource from the Amazon Web Services Control Catalog.</p>
    /// <p>To use this parameter, specify the ARN of the Control Catalog resource. You can specify either a control domain, a control objective, or a common control. For information about how to find the ARNs for these resources, see <a href="https://docs.aws.amazon.com/controlcatalog/latest/APIReference/API_ListDomains.html"> <code>ListDomains</code> </a>, <a href="https://docs.aws.amazon.com/controlcatalog/latest/APIReference/API_ListObjectives.html"> <code>ListObjectives</code> </a>, and <a href="https://docs.aws.amazon.com/controlcatalog/latest/APIReference/API_ListCommonControls.html"> <code>ListCommonControls</code> </a>.</p><note>
    /// <p>You can only filter by one Control Catalog resource at a time. Specifying multiple resource ARNs isn’t currently supported. If you want to filter by more than one ARN, we recommend that you run the <code>ListControls</code> operation separately for each ARN.</p>
    /// </note>
    /// <p>Alternatively, specify <code>UNCATEGORIZED</code> to list controls that aren't mapped to a Control Catalog resource. For example, this operation might return a list of custom controls that don't belong to any control domain or control objective.</p>
    pub fn control_catalog_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.control_catalog_id(input.into());
        self
    }
    /// <p>A filter that narrows the list of controls to a specific resource from the Amazon Web Services Control Catalog.</p>
    /// <p>To use this parameter, specify the ARN of the Control Catalog resource. You can specify either a control domain, a control objective, or a common control. For information about how to find the ARNs for these resources, see <a href="https://docs.aws.amazon.com/controlcatalog/latest/APIReference/API_ListDomains.html"> <code>ListDomains</code> </a>, <a href="https://docs.aws.amazon.com/controlcatalog/latest/APIReference/API_ListObjectives.html"> <code>ListObjectives</code> </a>, and <a href="https://docs.aws.amazon.com/controlcatalog/latest/APIReference/API_ListCommonControls.html"> <code>ListCommonControls</code> </a>.</p><note>
    /// <p>You can only filter by one Control Catalog resource at a time. Specifying multiple resource ARNs isn’t currently supported. If you want to filter by more than one ARN, we recommend that you run the <code>ListControls</code> operation separately for each ARN.</p>
    /// </note>
    /// <p>Alternatively, specify <code>UNCATEGORIZED</code> to list controls that aren't mapped to a Control Catalog resource. For example, this operation might return a list of custom controls that don't belong to any control domain or control objective.</p>
    pub fn set_control_catalog_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_control_catalog_id(input);
        self
    }
    /// <p>A filter that narrows the list of controls to a specific resource from the Amazon Web Services Control Catalog.</p>
    /// <p>To use this parameter, specify the ARN of the Control Catalog resource. You can specify either a control domain, a control objective, or a common control. For information about how to find the ARNs for these resources, see <a href="https://docs.aws.amazon.com/controlcatalog/latest/APIReference/API_ListDomains.html"> <code>ListDomains</code> </a>, <a href="https://docs.aws.amazon.com/controlcatalog/latest/APIReference/API_ListObjectives.html"> <code>ListObjectives</code> </a>, and <a href="https://docs.aws.amazon.com/controlcatalog/latest/APIReference/API_ListCommonControls.html"> <code>ListCommonControls</code> </a>.</p><note>
    /// <p>You can only filter by one Control Catalog resource at a time. Specifying multiple resource ARNs isn’t currently supported. If you want to filter by more than one ARN, we recommend that you run the <code>ListControls</code> operation separately for each ARN.</p>
    /// </note>
    /// <p>Alternatively, specify <code>UNCATEGORIZED</code> to list controls that aren't mapped to a Control Catalog resource. For example, this operation might return a list of custom controls that don't belong to any control domain or control objective.</p>
    pub fn get_control_catalog_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_control_catalog_id()
    }
}
