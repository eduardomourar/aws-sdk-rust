// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::batch_delete_topic_reviewed_answer::_batch_delete_topic_reviewed_answer_output::BatchDeleteTopicReviewedAnswerOutputBuilder;

pub use crate::operation::batch_delete_topic_reviewed_answer::_batch_delete_topic_reviewed_answer_input::BatchDeleteTopicReviewedAnswerInputBuilder;

impl crate::operation::batch_delete_topic_reviewed_answer::builders::BatchDeleteTopicReviewedAnswerInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::batch_delete_topic_reviewed_answer::BatchDeleteTopicReviewedAnswerOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::batch_delete_topic_reviewed_answer::BatchDeleteTopicReviewedAnswerError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.batch_delete_topic_reviewed_answer();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `BatchDeleteTopicReviewedAnswer`.
///
/// <p>Deletes reviewed answers for Q Topic.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct BatchDeleteTopicReviewedAnswerFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::batch_delete_topic_reviewed_answer::builders::BatchDeleteTopicReviewedAnswerInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::batch_delete_topic_reviewed_answer::BatchDeleteTopicReviewedAnswerOutput,
        crate::operation::batch_delete_topic_reviewed_answer::BatchDeleteTopicReviewedAnswerError,
    > for BatchDeleteTopicReviewedAnswerFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::batch_delete_topic_reviewed_answer::BatchDeleteTopicReviewedAnswerOutput,
            crate::operation::batch_delete_topic_reviewed_answer::BatchDeleteTopicReviewedAnswerError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl BatchDeleteTopicReviewedAnswerFluentBuilder {
    /// Creates a new `BatchDeleteTopicReviewedAnswerFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the BatchDeleteTopicReviewedAnswer as a reference.
    pub fn as_input(&self) -> &crate::operation::batch_delete_topic_reviewed_answer::builders::BatchDeleteTopicReviewedAnswerInputBuilder {
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
        crate::operation::batch_delete_topic_reviewed_answer::BatchDeleteTopicReviewedAnswerOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::batch_delete_topic_reviewed_answer::BatchDeleteTopicReviewedAnswerError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::batch_delete_topic_reviewed_answer::BatchDeleteTopicReviewedAnswer::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::batch_delete_topic_reviewed_answer::BatchDeleteTopicReviewedAnswer::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::batch_delete_topic_reviewed_answer::BatchDeleteTopicReviewedAnswerOutput,
        crate::operation::batch_delete_topic_reviewed_answer::BatchDeleteTopicReviewedAnswerError,
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
    /// <p>The ID of the Amazon Web Services account that you want to delete a reviewed answers in.</p>
    pub fn aws_account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.aws_account_id(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services account that you want to delete a reviewed answers in.</p>
    pub fn set_aws_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_aws_account_id(input);
        self
    }
    /// <p>The ID of the Amazon Web Services account that you want to delete a reviewed answers in.</p>
    pub fn get_aws_account_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_aws_account_id()
    }
    /// <p>The ID for the topic reviewed answer that you want to delete. This ID is unique per Amazon Web Services Region for each Amazon Web Services account.</p>
    pub fn topic_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.topic_id(input.into());
        self
    }
    /// <p>The ID for the topic reviewed answer that you want to delete. This ID is unique per Amazon Web Services Region for each Amazon Web Services account.</p>
    pub fn set_topic_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_topic_id(input);
        self
    }
    /// <p>The ID for the topic reviewed answer that you want to delete. This ID is unique per Amazon Web Services Region for each Amazon Web Services account.</p>
    pub fn get_topic_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_topic_id()
    }
    ///
    /// Appends an item to `AnswerIds`.
    ///
    /// To override the contents of this collection use [`set_answer_ids`](Self::set_answer_ids).
    ///
    /// <p>The Answer IDs of the Answers to be deleted.</p>
    pub fn answer_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.answer_ids(input.into());
        self
    }
    /// <p>The Answer IDs of the Answers to be deleted.</p>
    pub fn set_answer_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_answer_ids(input);
        self
    }
    /// <p>The Answer IDs of the Answers to be deleted.</p>
    pub fn get_answer_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_answer_ids()
    }
}
