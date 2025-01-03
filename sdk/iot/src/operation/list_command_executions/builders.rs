// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_command_executions::_list_command_executions_output::ListCommandExecutionsOutputBuilder;

pub use crate::operation::list_command_executions::_list_command_executions_input::ListCommandExecutionsInputBuilder;

impl crate::operation::list_command_executions::builders::ListCommandExecutionsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_command_executions::ListCommandExecutionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_command_executions::ListCommandExecutionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_command_executions();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListCommandExecutions`.
///
/// <p>List all command executions.</p><important>
/// <ul>
/// <li>
/// <p>You must provide only the <code>startedTimeFilter</code> or the <code>completedTimeFilter</code> information. If you provide both time filters, the API will generate an error. You can use this information to retrieve a list of command executions within a specific timeframe.</p></li>
/// <li>
/// <p>You must provide only the <code>commandArn</code> or the <code>thingArn</code> information depending on whether you want to list executions for a specific command or an IoT thing. If you provide both fields, the API will generate an error.</p></li>
/// </ul>
/// <p>For more information about considerations for using this API, see <a href="https://docs.aws.amazon.com/iot/latest/developerguide/iot-remote-command-execution-start-monitor.html#iot-remote-command-execution-list-cli">List command executions in your account (CLI)</a>.</p>
/// </important>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListCommandExecutionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_command_executions::builders::ListCommandExecutionsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_command_executions::ListCommandExecutionsOutput,
        crate::operation::list_command_executions::ListCommandExecutionsError,
    > for ListCommandExecutionsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_command_executions::ListCommandExecutionsOutput,
            crate::operation::list_command_executions::ListCommandExecutionsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListCommandExecutionsFluentBuilder {
    /// Creates a new `ListCommandExecutionsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListCommandExecutions as a reference.
    pub fn as_input(&self) -> &crate::operation::list_command_executions::builders::ListCommandExecutionsInputBuilder {
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
        crate::operation::list_command_executions::ListCommandExecutionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_command_executions::ListCommandExecutionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_command_executions::ListCommandExecutions::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_command_executions::ListCommandExecutions::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_command_executions::ListCommandExecutionsOutput,
        crate::operation::list_command_executions::ListCommandExecutionsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_command_executions::paginator::ListCommandExecutionsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_command_executions::paginator::ListCommandExecutionsPaginator {
        crate::operation::list_command_executions::paginator::ListCommandExecutionsPaginator::new(self.handle, self.inner)
    }
    /// <p>The maximum number of results to return in this operation.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return in this operation.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to return in this operation.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>To retrieve the next set of results, the <code>nextToken</code> value from a previous response; otherwise <code>null</code> to receive the first set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>To retrieve the next set of results, the <code>nextToken</code> value from a previous response; otherwise <code>null</code> to receive the first set of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>To retrieve the next set of results, the <code>nextToken</code> value from a previous response; otherwise <code>null</code> to receive the first set of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The namespace of the command.</p>
    pub fn namespace(mut self, input: crate::types::CommandNamespace) -> Self {
        self.inner = self.inner.namespace(input);
        self
    }
    /// <p>The namespace of the command.</p>
    pub fn set_namespace(mut self, input: ::std::option::Option<crate::types::CommandNamespace>) -> Self {
        self.inner = self.inner.set_namespace(input);
        self
    }
    /// <p>The namespace of the command.</p>
    pub fn get_namespace(&self) -> &::std::option::Option<crate::types::CommandNamespace> {
        self.inner.get_namespace()
    }
    /// <p>List all command executions for the device that have a particular status. For example, you can filter the list to display only command executions that have failed or timed out.</p>
    pub fn status(mut self, input: crate::types::CommandExecutionStatus) -> Self {
        self.inner = self.inner.status(input);
        self
    }
    /// <p>List all command executions for the device that have a particular status. For example, you can filter the list to display only command executions that have failed or timed out.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::CommandExecutionStatus>) -> Self {
        self.inner = self.inner.set_status(input);
        self
    }
    /// <p>List all command executions for the device that have a particular status. For example, you can filter the list to display only command executions that have failed or timed out.</p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::CommandExecutionStatus> {
        self.inner.get_status()
    }
    /// <p>Specify whether to list the command executions that were created in the ascending or descending order. By default, the API returns all commands in the descending order based on the start time or completion time of the executions, that are determined by the <code>startTimeFilter</code> and <code>completeTimeFilter</code> parameters.</p>
    pub fn sort_order(mut self, input: crate::types::SortOrder) -> Self {
        self.inner = self.inner.sort_order(input);
        self
    }
    /// <p>Specify whether to list the command executions that were created in the ascending or descending order. By default, the API returns all commands in the descending order based on the start time or completion time of the executions, that are determined by the <code>startTimeFilter</code> and <code>completeTimeFilter</code> parameters.</p>
    pub fn set_sort_order(mut self, input: ::std::option::Option<crate::types::SortOrder>) -> Self {
        self.inner = self.inner.set_sort_order(input);
        self
    }
    /// <p>Specify whether to list the command executions that were created in the ascending or descending order. By default, the API returns all commands in the descending order based on the start time or completion time of the executions, that are determined by the <code>startTimeFilter</code> and <code>completeTimeFilter</code> parameters.</p>
    pub fn get_sort_order(&self) -> &::std::option::Option<crate::types::SortOrder> {
        self.inner.get_sort_order()
    }
    /// <p>List all command executions that started any time before or after the date and time that you specify. The date and time uses the format <code>yyyy-MM-dd'T'HH:mm</code>.</p>
    pub fn started_time_filter(mut self, input: crate::types::TimeFilter) -> Self {
        self.inner = self.inner.started_time_filter(input);
        self
    }
    /// <p>List all command executions that started any time before or after the date and time that you specify. The date and time uses the format <code>yyyy-MM-dd'T'HH:mm</code>.</p>
    pub fn set_started_time_filter(mut self, input: ::std::option::Option<crate::types::TimeFilter>) -> Self {
        self.inner = self.inner.set_started_time_filter(input);
        self
    }
    /// <p>List all command executions that started any time before or after the date and time that you specify. The date and time uses the format <code>yyyy-MM-dd'T'HH:mm</code>.</p>
    pub fn get_started_time_filter(&self) -> &::std::option::Option<crate::types::TimeFilter> {
        self.inner.get_started_time_filter()
    }
    /// <p>List all command executions that completed any time before or after the date and time that you specify. The date and time uses the format <code>yyyy-MM-dd'T'HH:mm</code>.</p>
    pub fn completed_time_filter(mut self, input: crate::types::TimeFilter) -> Self {
        self.inner = self.inner.completed_time_filter(input);
        self
    }
    /// <p>List all command executions that completed any time before or after the date and time that you specify. The date and time uses the format <code>yyyy-MM-dd'T'HH:mm</code>.</p>
    pub fn set_completed_time_filter(mut self, input: ::std::option::Option<crate::types::TimeFilter>) -> Self {
        self.inner = self.inner.set_completed_time_filter(input);
        self
    }
    /// <p>List all command executions that completed any time before or after the date and time that you specify. The date and time uses the format <code>yyyy-MM-dd'T'HH:mm</code>.</p>
    pub fn get_completed_time_filter(&self) -> &::std::option::Option<crate::types::TimeFilter> {
        self.inner.get_completed_time_filter()
    }
    /// <p>The Amazon Resource Number (ARN) of the target device. You can use this information to list all command executions for a particular device.</p>
    pub fn target_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.target_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Number (ARN) of the target device. You can use this information to list all command executions for a particular device.</p>
    pub fn set_target_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_target_arn(input);
        self
    }
    /// <p>The Amazon Resource Number (ARN) of the target device. You can use this information to list all command executions for a particular device.</p>
    pub fn get_target_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_target_arn()
    }
    /// <p>The Amazon Resource Number (ARN) of the command. You can use this information to list all command executions for a particular command.</p>
    pub fn command_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.command_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Number (ARN) of the command. You can use this information to list all command executions for a particular command.</p>
    pub fn set_command_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_command_arn(input);
        self
    }
    /// <p>The Amazon Resource Number (ARN) of the command. You can use this information to list all command executions for a particular command.</p>
    pub fn get_command_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_command_arn()
    }
}
