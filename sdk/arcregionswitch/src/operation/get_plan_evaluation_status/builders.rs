// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_plan_evaluation_status::_get_plan_evaluation_status_output::GetPlanEvaluationStatusOutputBuilder;

pub use crate::operation::get_plan_evaluation_status::_get_plan_evaluation_status_input::GetPlanEvaluationStatusInputBuilder;

impl crate::operation::get_plan_evaluation_status::builders::GetPlanEvaluationStatusInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_plan_evaluation_status::GetPlanEvaluationStatusOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_plan_evaluation_status::GetPlanEvaluationStatusError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_plan_evaluation_status();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetPlanEvaluationStatus`.
///
/// <p>Retrieves the evaluation status of a Region switch plan. The evaluation status provides information about the last time the plan was evaluated and any warnings or issues detected.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetPlanEvaluationStatusFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_plan_evaluation_status::builders::GetPlanEvaluationStatusInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_plan_evaluation_status::GetPlanEvaluationStatusOutput,
        crate::operation::get_plan_evaluation_status::GetPlanEvaluationStatusError,
    > for GetPlanEvaluationStatusFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_plan_evaluation_status::GetPlanEvaluationStatusOutput,
            crate::operation::get_plan_evaluation_status::GetPlanEvaluationStatusError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetPlanEvaluationStatusFluentBuilder {
    /// Creates a new `GetPlanEvaluationStatusFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetPlanEvaluationStatus as a reference.
    pub fn as_input(&self) -> &crate::operation::get_plan_evaluation_status::builders::GetPlanEvaluationStatusInputBuilder {
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
        crate::operation::get_plan_evaluation_status::GetPlanEvaluationStatusOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_plan_evaluation_status::GetPlanEvaluationStatusError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_plan_evaluation_status::GetPlanEvaluationStatus::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_plan_evaluation_status::GetPlanEvaluationStatus::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_plan_evaluation_status::GetPlanEvaluationStatusOutput,
        crate::operation::get_plan_evaluation_status::GetPlanEvaluationStatusError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::get_plan_evaluation_status::paginator::GetPlanEvaluationStatusPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::get_plan_evaluation_status::paginator::GetPlanEvaluationStatusPaginator {
        crate::operation::get_plan_evaluation_status::paginator::GetPlanEvaluationStatusPaginator::new(self.handle, self.inner)
    }
    /// <p>The Amazon Resource Name (ARN) of the Region switch plan to retrieve evaluation status for.</p>
    pub fn plan_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.plan_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Region switch plan to retrieve evaluation status for.</p>
    pub fn set_plan_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_plan_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Region switch plan to retrieve evaluation status for.</p>
    pub fn get_plan_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_plan_arn()
    }
    /// <p>The number of objects that you want to return with this call.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The number of objects that you want to return with this call.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The number of objects that you want to return with this call.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>Specifies that you want to receive the next page of results. Valid only if you received a <code>nextToken</code> response in the previous request. If you did, it indicates that more output is available. Set this parameter to the value provided by the previous call's <code>nextToken</code> response to request the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>Specifies that you want to receive the next page of results. Valid only if you received a <code>nextToken</code> response in the previous request. If you did, it indicates that more output is available. Set this parameter to the value provided by the previous call's <code>nextToken</code> response to request the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>Specifies that you want to receive the next page of results. Valid only if you received a <code>nextToken</code> response in the previous request. If you did, it indicates that more output is available. Set this parameter to the value provided by the previous call's <code>nextToken</code> response to request the next page of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
