// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_restore_testing_plan::_create_restore_testing_plan_output::CreateRestoreTestingPlanOutputBuilder;

pub use crate::operation::create_restore_testing_plan::_create_restore_testing_plan_input::CreateRestoreTestingPlanInputBuilder;

impl crate::operation::create_restore_testing_plan::builders::CreateRestoreTestingPlanInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_restore_testing_plan::CreateRestoreTestingPlanOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_restore_testing_plan::CreateRestoreTestingPlanError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_restore_testing_plan();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateRestoreTestingPlan`.
///
/// <p>Creates a restore testing plan.</p>
/// <p>The first of two steps to create a restore testing plan. After this request is successful, finish the procedure using CreateRestoreTestingSelection.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateRestoreTestingPlanFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_restore_testing_plan::builders::CreateRestoreTestingPlanInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_restore_testing_plan::CreateRestoreTestingPlanOutput,
        crate::operation::create_restore_testing_plan::CreateRestoreTestingPlanError,
    > for CreateRestoreTestingPlanFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_restore_testing_plan::CreateRestoreTestingPlanOutput,
            crate::operation::create_restore_testing_plan::CreateRestoreTestingPlanError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateRestoreTestingPlanFluentBuilder {
    /// Creates a new `CreateRestoreTestingPlanFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateRestoreTestingPlan as a reference.
    pub fn as_input(&self) -> &crate::operation::create_restore_testing_plan::builders::CreateRestoreTestingPlanInputBuilder {
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
        crate::operation::create_restore_testing_plan::CreateRestoreTestingPlanOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_restore_testing_plan::CreateRestoreTestingPlanError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_restore_testing_plan::CreateRestoreTestingPlan::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_restore_testing_plan::CreateRestoreTestingPlan::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_restore_testing_plan::CreateRestoreTestingPlanOutput,
        crate::operation::create_restore_testing_plan::CreateRestoreTestingPlanError,
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
    /// <p>This is a unique string that identifies the request and allows failed requests to be retriedwithout the risk of running the operation twice. This parameter is optional. If used, this parameter must contain 1 to 50 alphanumeric or '-_.' characters.</p>
    pub fn creator_request_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.creator_request_id(input.into());
        self
    }
    /// <p>This is a unique string that identifies the request and allows failed requests to be retriedwithout the risk of running the operation twice. This parameter is optional. If used, this parameter must contain 1 to 50 alphanumeric or '-_.' characters.</p>
    pub fn set_creator_request_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_creator_request_id(input);
        self
    }
    /// <p>This is a unique string that identifies the request and allows failed requests to be retriedwithout the risk of running the operation twice. This parameter is optional. If used, this parameter must contain 1 to 50 alphanumeric or '-_.' characters.</p>
    pub fn get_creator_request_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_creator_request_id()
    }
    /// <p>A restore testing plan must contain a unique <code>RestoreTestingPlanName</code> string you create and must contain a <code>ScheduleExpression</code> cron. You may optionally include a <code>StartWindowHours</code> integer and a <code>CreatorRequestId</code> string.</p>
    /// <p>The <code>RestoreTestingPlanName</code> is a unique string that is the name of the restore testing plan. This cannot be changed after creation, and it must consist of only alphanumeric characters and underscores.</p>
    pub fn restore_testing_plan(mut self, input: crate::types::RestoreTestingPlanForCreate) -> Self {
        self.inner = self.inner.restore_testing_plan(input);
        self
    }
    /// <p>A restore testing plan must contain a unique <code>RestoreTestingPlanName</code> string you create and must contain a <code>ScheduleExpression</code> cron. You may optionally include a <code>StartWindowHours</code> integer and a <code>CreatorRequestId</code> string.</p>
    /// <p>The <code>RestoreTestingPlanName</code> is a unique string that is the name of the restore testing plan. This cannot be changed after creation, and it must consist of only alphanumeric characters and underscores.</p>
    pub fn set_restore_testing_plan(mut self, input: ::std::option::Option<crate::types::RestoreTestingPlanForCreate>) -> Self {
        self.inner = self.inner.set_restore_testing_plan(input);
        self
    }
    /// <p>A restore testing plan must contain a unique <code>RestoreTestingPlanName</code> string you create and must contain a <code>ScheduleExpression</code> cron. You may optionally include a <code>StartWindowHours</code> integer and a <code>CreatorRequestId</code> string.</p>
    /// <p>The <code>RestoreTestingPlanName</code> is a unique string that is the name of the restore testing plan. This cannot be changed after creation, and it must consist of only alphanumeric characters and underscores.</p>
    pub fn get_restore_testing_plan(&self) -> &::std::option::Option<crate::types::RestoreTestingPlanForCreate> {
        self.inner.get_restore_testing_plan()
    }
    ///
    /// Adds a key-value pair to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags to assign to the restore testing plan.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>The tags to assign to the restore testing plan.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The tags to assign to the restore testing plan.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_tags()
    }
}
