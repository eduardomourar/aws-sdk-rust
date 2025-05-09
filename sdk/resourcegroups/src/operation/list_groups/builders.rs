// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_groups::_list_groups_output::ListGroupsOutputBuilder;

pub use crate::operation::list_groups::_list_groups_input::ListGroupsInputBuilder;

impl crate::operation::list_groups::builders::ListGroupsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_groups::ListGroupsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_groups::ListGroupsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_groups();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListGroups`.
///
/// <p>Returns a list of existing Resource Groups in your account.</p>
/// <p><b>Minimum permissions</b></p>
/// <p>To run this command, you must have the following permissions:</p>
/// <ul>
/// <li>
/// <p><code>resource-groups:ListGroups</code></p></li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListGroupsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_groups::builders::ListGroupsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_groups::ListGroupsOutput,
        crate::operation::list_groups::ListGroupsError,
    > for ListGroupsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_groups::ListGroupsOutput,
            crate::operation::list_groups::ListGroupsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListGroupsFluentBuilder {
    /// Creates a new `ListGroupsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListGroups as a reference.
    pub fn as_input(&self) -> &crate::operation::list_groups::builders::ListGroupsInputBuilder {
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
        crate::operation::list_groups::ListGroupsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_groups::ListGroupsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_groups::ListGroups::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_groups::ListGroups::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_groups::ListGroupsOutput,
        crate::operation::list_groups::ListGroupsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_groups::paginator::ListGroupsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_groups::paginator::ListGroupsPaginator {
        crate::operation::list_groups::paginator::ListGroupsPaginator::new(self.handle, self.inner)
    }
    ///
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>Filters, formatted as <code>GroupFilter</code> objects, that you want to apply to a <code>ListGroups</code> operation.</p>
    /// <ul>
    /// <li>
    /// <p><code>resource-type</code> - Filter the results to include only those resource groups that have the specified resource type in their <code>ResourceTypeFilter</code>. For example, <code>AWS::EC2::Instance</code> would return any resource group with a <code>ResourceTypeFilter</code> that includes <code>AWS::EC2::Instance</code>.</p></li>
    /// <li>
    /// <p><code>configuration-type</code> - Filter the results to include only those groups that have the specified configuration types attached. The current supported values are:</p>
    /// <ul>
    /// <li>
    /// <p><code>AWS::ResourceGroups::ApplicationGroup</code></p></li>
    /// <li>
    /// <p><code>AWS::AppRegistry::Application</code></p></li>
    /// <li>
    /// <p><code>AWS::AppRegistry::ApplicationResourceGroup</code></p></li>
    /// <li>
    /// <p><code>AWS::CloudFormation::Stack</code></p></li>
    /// <li>
    /// <p><code>AWS::EC2::CapacityReservationPool</code></p></li>
    /// <li>
    /// <p><code>AWS::EC2::HostManagement</code></p></li>
    /// <li>
    /// <p><code>AWS::NetworkFirewall::RuleGroup</code></p></li>
    /// </ul></li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::GroupFilter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>Filters, formatted as <code>GroupFilter</code> objects, that you want to apply to a <code>ListGroups</code> operation.</p>
    /// <ul>
    /// <li>
    /// <p><code>resource-type</code> - Filter the results to include only those resource groups that have the specified resource type in their <code>ResourceTypeFilter</code>. For example, <code>AWS::EC2::Instance</code> would return any resource group with a <code>ResourceTypeFilter</code> that includes <code>AWS::EC2::Instance</code>.</p></li>
    /// <li>
    /// <p><code>configuration-type</code> - Filter the results to include only those groups that have the specified configuration types attached. The current supported values are:</p>
    /// <ul>
    /// <li>
    /// <p><code>AWS::ResourceGroups::ApplicationGroup</code></p></li>
    /// <li>
    /// <p><code>AWS::AppRegistry::Application</code></p></li>
    /// <li>
    /// <p><code>AWS::AppRegistry::ApplicationResourceGroup</code></p></li>
    /// <li>
    /// <p><code>AWS::CloudFormation::Stack</code></p></li>
    /// <li>
    /// <p><code>AWS::EC2::CapacityReservationPool</code></p></li>
    /// <li>
    /// <p><code>AWS::EC2::HostManagement</code></p></li>
    /// <li>
    /// <p><code>AWS::NetworkFirewall::RuleGroup</code></p></li>
    /// </ul></li>
    /// </ul>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::GroupFilter>>) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>Filters, formatted as <code>GroupFilter</code> objects, that you want to apply to a <code>ListGroups</code> operation.</p>
    /// <ul>
    /// <li>
    /// <p><code>resource-type</code> - Filter the results to include only those resource groups that have the specified resource type in their <code>ResourceTypeFilter</code>. For example, <code>AWS::EC2::Instance</code> would return any resource group with a <code>ResourceTypeFilter</code> that includes <code>AWS::EC2::Instance</code>.</p></li>
    /// <li>
    /// <p><code>configuration-type</code> - Filter the results to include only those groups that have the specified configuration types attached. The current supported values are:</p>
    /// <ul>
    /// <li>
    /// <p><code>AWS::ResourceGroups::ApplicationGroup</code></p></li>
    /// <li>
    /// <p><code>AWS::AppRegistry::Application</code></p></li>
    /// <li>
    /// <p><code>AWS::AppRegistry::ApplicationResourceGroup</code></p></li>
    /// <li>
    /// <p><code>AWS::CloudFormation::Stack</code></p></li>
    /// <li>
    /// <p><code>AWS::EC2::CapacityReservationPool</code></p></li>
    /// <li>
    /// <p><code>AWS::EC2::HostManagement</code></p></li>
    /// <li>
    /// <p><code>AWS::NetworkFirewall::RuleGroup</code></p></li>
    /// </ul></li>
    /// </ul>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::GroupFilter>> {
        self.inner.get_filters()
    }
    /// <p>The total number of results that you want included on each page of the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that the service might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The total number of results that you want included on each page of the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that the service might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The total number of results that you want included on each page of the response. If you do not include this parameter, it defaults to a value that is specific to the operation. If additional items exist beyond the maximum you specify, the <code>NextToken</code> response element is present and has a value (is not null). Include that value as the <code>NextToken</code> request parameter in the next call to the operation to get the next part of the results. Note that the service might return fewer results than the maximum even when there are more results available. You should check <code>NextToken</code> after every operation to ensure that you receive all of the results.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>The parameter for receiving additional results if you receive a <code>NextToken</code> response in a previous request. A <code>NextToken</code> response indicates that more output is available. Set this parameter to the value provided by a previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The parameter for receiving additional results if you receive a <code>NextToken</code> response in a previous request. A <code>NextToken</code> response indicates that more output is available. Set this parameter to the value provided by a previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The parameter for receiving additional results if you receive a <code>NextToken</code> response in a previous request. A <code>NextToken</code> response indicates that more output is available. Set this parameter to the value provided by a previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
