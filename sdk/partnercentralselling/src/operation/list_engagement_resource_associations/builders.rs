// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_engagement_resource_associations::_list_engagement_resource_associations_output::ListEngagementResourceAssociationsOutputBuilder;

pub use crate::operation::list_engagement_resource_associations::_list_engagement_resource_associations_input::ListEngagementResourceAssociationsInputBuilder;

impl crate::operation::list_engagement_resource_associations::builders::ListEngagementResourceAssociationsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_engagement_resource_associations::ListEngagementResourceAssociationsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_engagement_resource_associations::ListEngagementResourceAssociationsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_engagement_resource_associations();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListEngagementResourceAssociations`.
///
/// <p>Lists the associations between resources and engagements where the caller is a member and has at least one snapshot in the engagement.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListEngagementResourceAssociationsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_engagement_resource_associations::builders::ListEngagementResourceAssociationsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_engagement_resource_associations::ListEngagementResourceAssociationsOutput,
        crate::operation::list_engagement_resource_associations::ListEngagementResourceAssociationsError,
    > for ListEngagementResourceAssociationsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_engagement_resource_associations::ListEngagementResourceAssociationsOutput,
            crate::operation::list_engagement_resource_associations::ListEngagementResourceAssociationsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListEngagementResourceAssociationsFluentBuilder {
    /// Creates a new `ListEngagementResourceAssociationsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListEngagementResourceAssociations as a reference.
    pub fn as_input(&self) -> &crate::operation::list_engagement_resource_associations::builders::ListEngagementResourceAssociationsInputBuilder {
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
        crate::operation::list_engagement_resource_associations::ListEngagementResourceAssociationsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_engagement_resource_associations::ListEngagementResourceAssociationsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_engagement_resource_associations::ListEngagementResourceAssociations::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_engagement_resource_associations::ListEngagementResourceAssociations::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_engagement_resource_associations::ListEngagementResourceAssociationsOutput,
        crate::operation::list_engagement_resource_associations::ListEngagementResourceAssociationsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_engagement_resource_associations::paginator::ListEngagementResourceAssociationsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_engagement_resource_associations::paginator::ListEngagementResourceAssociationsPaginator {
        crate::operation::list_engagement_resource_associations::paginator::ListEngagementResourceAssociationsPaginator::new(self.handle, self.inner)
    }
    /// <p>Specifies the catalog in which to search for engagement-resource associations. Valid Values: "AWS" or "Sandbox"</p>
    /// <ul>
    /// <li>
    /// <p><code>AWS</code> for production environments.</p></li>
    /// <li>
    /// <p><code>Sandbox</code> for testing and development purposes.</p></li>
    /// </ul>
    pub fn catalog(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.catalog(input.into());
        self
    }
    /// <p>Specifies the catalog in which to search for engagement-resource associations. Valid Values: "AWS" or "Sandbox"</p>
    /// <ul>
    /// <li>
    /// <p><code>AWS</code> for production environments.</p></li>
    /// <li>
    /// <p><code>Sandbox</code> for testing and development purposes.</p></li>
    /// </ul>
    pub fn set_catalog(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_catalog(input);
        self
    }
    /// <p>Specifies the catalog in which to search for engagement-resource associations. Valid Values: "AWS" or "Sandbox"</p>
    /// <ul>
    /// <li>
    /// <p><code>AWS</code> for production environments.</p></li>
    /// <li>
    /// <p><code>Sandbox</code> for testing and development purposes.</p></li>
    /// </ul>
    pub fn get_catalog(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_catalog()
    }
    /// <p>Limits the number of results returned in a single call. Use this to control the number of results returned, especially useful for pagination.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>Limits the number of results returned in a single call. Use this to control the number of results returned, especially useful for pagination.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>Limits the number of results returned in a single call. Use this to control the number of results returned, especially useful for pagination.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>A token used for pagination of results. Include this token in subsequent requests to retrieve the next set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>A token used for pagination of results. Include this token in subsequent requests to retrieve the next set of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>A token used for pagination of results. Include this token in subsequent requests to retrieve the next set of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>Filters the results to include only associations related to the specified engagement. Use this when you want to find all resources associated with a specific engagement.</p>
    pub fn engagement_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.engagement_identifier(input.into());
        self
    }
    /// <p>Filters the results to include only associations related to the specified engagement. Use this when you want to find all resources associated with a specific engagement.</p>
    pub fn set_engagement_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_engagement_identifier(input);
        self
    }
    /// <p>Filters the results to include only associations related to the specified engagement. Use this when you want to find all resources associated with a specific engagement.</p>
    pub fn get_engagement_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_engagement_identifier()
    }
    /// <p>Filters the results to include only associations with resources of the specified type.</p>
    pub fn resource_type(mut self, input: crate::types::ResourceType) -> Self {
        self.inner = self.inner.resource_type(input);
        self
    }
    /// <p>Filters the results to include only associations with resources of the specified type.</p>
    pub fn set_resource_type(mut self, input: ::std::option::Option<crate::types::ResourceType>) -> Self {
        self.inner = self.inner.set_resource_type(input);
        self
    }
    /// <p>Filters the results to include only associations with resources of the specified type.</p>
    pub fn get_resource_type(&self) -> &::std::option::Option<crate::types::ResourceType> {
        self.inner.get_resource_type()
    }
    /// <p>Filters the results to include only associations with the specified resource. Varies depending on the resource type. Use this when you want to find all engagements associated with a specific resource.</p>
    pub fn resource_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_identifier(input.into());
        self
    }
    /// <p>Filters the results to include only associations with the specified resource. Varies depending on the resource type. Use this when you want to find all engagements associated with a specific resource.</p>
    pub fn set_resource_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_identifier(input);
        self
    }
    /// <p>Filters the results to include only associations with the specified resource. Varies depending on the resource type. Use this when you want to find all engagements associated with a specific resource.</p>
    pub fn get_resource_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resource_identifier()
    }
    /// <p>Filters the response to include only snapshots of resources owned by the specified AWS account ID. Use this when you want to find associations related to resources owned by a particular account.</p>
    pub fn created_by(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.created_by(input.into());
        self
    }
    /// <p>Filters the response to include only snapshots of resources owned by the specified AWS account ID. Use this when you want to find associations related to resources owned by a particular account.</p>
    pub fn set_created_by(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_created_by(input);
        self
    }
    /// <p>Filters the response to include only snapshots of resources owned by the specified AWS account ID. Use this when you want to find associations related to resources owned by a particular account.</p>
    pub fn get_created_by(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_created_by()
    }
}
