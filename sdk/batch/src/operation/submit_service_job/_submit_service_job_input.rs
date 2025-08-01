// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SubmitServiceJobInput {
    /// <p>The name of the service job. It can be up to 128 characters long. It can contain uppercase and lowercase letters, numbers, hyphens (-), and underscores (_).</p>
    pub job_name: ::std::option::Option<::std::string::String>,
    /// <p>The job queue into which the service job is submitted. You can specify either the name or the ARN of the queue. The job queue must have the type <code>SAGEMAKER_TRAINING</code>.</p>
    pub job_queue: ::std::option::Option<::std::string::String>,
    /// <p>The retry strategy to use for failed service jobs that are submitted with this service job request.</p>
    pub retry_strategy: ::std::option::Option<crate::types::ServiceJobRetryStrategy>,
    /// <p>The scheduling priority of the service job. Valid values are integers between 0 and 9999.</p>
    pub scheduling_priority: ::std::option::Option<i32>,
    /// <p>The request, in JSON, for the service that the SubmitServiceJob operation is queueing.</p>
    pub service_request_payload: ::std::option::Option<::std::string::String>,
    /// <p>The type of service job. For SageMaker Training jobs, specify <code>SAGEMAKER_TRAINING</code>.</p>
    pub service_job_type: ::std::option::Option<crate::types::ServiceJobType>,
    /// <p>The share identifier for the service job. Don't specify this parameter if the job queue doesn't have a fair- share scheduling policy. If the job queue has a fair-share scheduling policy, then this parameter must be specified.</p>
    pub share_identifier: ::std::option::Option<::std::string::String>,
    /// <p>The timeout configuration for the service job. If none is specified, Batch defers to the default timeout of the underlying service handling the job.</p>
    pub timeout_config: ::std::option::Option<crate::types::ServiceJobTimeout>,
    /// <p>The tags that you apply to the service job request. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your Batch resources</a>.</p>
    pub tags: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    /// <p>A unique identifier for the request. This token is used to ensure idempotency of requests. If this parameter is specified and two submit requests with identical payloads and <code>clientToken</code>s are received, these requests are considered the same request and the second request is rejected.</p>
    pub client_token: ::std::option::Option<::std::string::String>,
}
impl SubmitServiceJobInput {
    /// <p>The name of the service job. It can be up to 128 characters long. It can contain uppercase and lowercase letters, numbers, hyphens (-), and underscores (_).</p>
    pub fn job_name(&self) -> ::std::option::Option<&str> {
        self.job_name.as_deref()
    }
    /// <p>The job queue into which the service job is submitted. You can specify either the name or the ARN of the queue. The job queue must have the type <code>SAGEMAKER_TRAINING</code>.</p>
    pub fn job_queue(&self) -> ::std::option::Option<&str> {
        self.job_queue.as_deref()
    }
    /// <p>The retry strategy to use for failed service jobs that are submitted with this service job request.</p>
    pub fn retry_strategy(&self) -> ::std::option::Option<&crate::types::ServiceJobRetryStrategy> {
        self.retry_strategy.as_ref()
    }
    /// <p>The scheduling priority of the service job. Valid values are integers between 0 and 9999.</p>
    pub fn scheduling_priority(&self) -> ::std::option::Option<i32> {
        self.scheduling_priority
    }
    /// <p>The request, in JSON, for the service that the SubmitServiceJob operation is queueing.</p>
    pub fn service_request_payload(&self) -> ::std::option::Option<&str> {
        self.service_request_payload.as_deref()
    }
    /// <p>The type of service job. For SageMaker Training jobs, specify <code>SAGEMAKER_TRAINING</code>.</p>
    pub fn service_job_type(&self) -> ::std::option::Option<&crate::types::ServiceJobType> {
        self.service_job_type.as_ref()
    }
    /// <p>The share identifier for the service job. Don't specify this parameter if the job queue doesn't have a fair- share scheduling policy. If the job queue has a fair-share scheduling policy, then this parameter must be specified.</p>
    pub fn share_identifier(&self) -> ::std::option::Option<&str> {
        self.share_identifier.as_deref()
    }
    /// <p>The timeout configuration for the service job. If none is specified, Batch defers to the default timeout of the underlying service handling the job.</p>
    pub fn timeout_config(&self) -> ::std::option::Option<&crate::types::ServiceJobTimeout> {
        self.timeout_config.as_ref()
    }
    /// <p>The tags that you apply to the service job request. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your Batch resources</a>.</p>
    pub fn tags(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.tags.as_ref()
    }
    /// <p>A unique identifier for the request. This token is used to ensure idempotency of requests. If this parameter is specified and two submit requests with identical payloads and <code>clientToken</code>s are received, these requests are considered the same request and the second request is rejected.</p>
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }
}
impl SubmitServiceJobInput {
    /// Creates a new builder-style object to manufacture [`SubmitServiceJobInput`](crate::operation::submit_service_job::SubmitServiceJobInput).
    pub fn builder() -> crate::operation::submit_service_job::builders::SubmitServiceJobInputBuilder {
        crate::operation::submit_service_job::builders::SubmitServiceJobInputBuilder::default()
    }
}

/// A builder for [`SubmitServiceJobInput`](crate::operation::submit_service_job::SubmitServiceJobInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct SubmitServiceJobInputBuilder {
    pub(crate) job_name: ::std::option::Option<::std::string::String>,
    pub(crate) job_queue: ::std::option::Option<::std::string::String>,
    pub(crate) retry_strategy: ::std::option::Option<crate::types::ServiceJobRetryStrategy>,
    pub(crate) scheduling_priority: ::std::option::Option<i32>,
    pub(crate) service_request_payload: ::std::option::Option<::std::string::String>,
    pub(crate) service_job_type: ::std::option::Option<crate::types::ServiceJobType>,
    pub(crate) share_identifier: ::std::option::Option<::std::string::String>,
    pub(crate) timeout_config: ::std::option::Option<crate::types::ServiceJobTimeout>,
    pub(crate) tags: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
}
impl SubmitServiceJobInputBuilder {
    /// <p>The name of the service job. It can be up to 128 characters long. It can contain uppercase and lowercase letters, numbers, hyphens (-), and underscores (_).</p>
    /// This field is required.
    pub fn job_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.job_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the service job. It can be up to 128 characters long. It can contain uppercase and lowercase letters, numbers, hyphens (-), and underscores (_).</p>
    pub fn set_job_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.job_name = input;
        self
    }
    /// <p>The name of the service job. It can be up to 128 characters long. It can contain uppercase and lowercase letters, numbers, hyphens (-), and underscores (_).</p>
    pub fn get_job_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.job_name
    }
    /// <p>The job queue into which the service job is submitted. You can specify either the name or the ARN of the queue. The job queue must have the type <code>SAGEMAKER_TRAINING</code>.</p>
    /// This field is required.
    pub fn job_queue(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.job_queue = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The job queue into which the service job is submitted. You can specify either the name or the ARN of the queue. The job queue must have the type <code>SAGEMAKER_TRAINING</code>.</p>
    pub fn set_job_queue(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.job_queue = input;
        self
    }
    /// <p>The job queue into which the service job is submitted. You can specify either the name or the ARN of the queue. The job queue must have the type <code>SAGEMAKER_TRAINING</code>.</p>
    pub fn get_job_queue(&self) -> &::std::option::Option<::std::string::String> {
        &self.job_queue
    }
    /// <p>The retry strategy to use for failed service jobs that are submitted with this service job request.</p>
    pub fn retry_strategy(mut self, input: crate::types::ServiceJobRetryStrategy) -> Self {
        self.retry_strategy = ::std::option::Option::Some(input);
        self
    }
    /// <p>The retry strategy to use for failed service jobs that are submitted with this service job request.</p>
    pub fn set_retry_strategy(mut self, input: ::std::option::Option<crate::types::ServiceJobRetryStrategy>) -> Self {
        self.retry_strategy = input;
        self
    }
    /// <p>The retry strategy to use for failed service jobs that are submitted with this service job request.</p>
    pub fn get_retry_strategy(&self) -> &::std::option::Option<crate::types::ServiceJobRetryStrategy> {
        &self.retry_strategy
    }
    /// <p>The scheduling priority of the service job. Valid values are integers between 0 and 9999.</p>
    pub fn scheduling_priority(mut self, input: i32) -> Self {
        self.scheduling_priority = ::std::option::Option::Some(input);
        self
    }
    /// <p>The scheduling priority of the service job. Valid values are integers between 0 and 9999.</p>
    pub fn set_scheduling_priority(mut self, input: ::std::option::Option<i32>) -> Self {
        self.scheduling_priority = input;
        self
    }
    /// <p>The scheduling priority of the service job. Valid values are integers between 0 and 9999.</p>
    pub fn get_scheduling_priority(&self) -> &::std::option::Option<i32> {
        &self.scheduling_priority
    }
    /// <p>The request, in JSON, for the service that the SubmitServiceJob operation is queueing.</p>
    /// This field is required.
    pub fn service_request_payload(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.service_request_payload = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The request, in JSON, for the service that the SubmitServiceJob operation is queueing.</p>
    pub fn set_service_request_payload(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.service_request_payload = input;
        self
    }
    /// <p>The request, in JSON, for the service that the SubmitServiceJob operation is queueing.</p>
    pub fn get_service_request_payload(&self) -> &::std::option::Option<::std::string::String> {
        &self.service_request_payload
    }
    /// <p>The type of service job. For SageMaker Training jobs, specify <code>SAGEMAKER_TRAINING</code>.</p>
    /// This field is required.
    pub fn service_job_type(mut self, input: crate::types::ServiceJobType) -> Self {
        self.service_job_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of service job. For SageMaker Training jobs, specify <code>SAGEMAKER_TRAINING</code>.</p>
    pub fn set_service_job_type(mut self, input: ::std::option::Option<crate::types::ServiceJobType>) -> Self {
        self.service_job_type = input;
        self
    }
    /// <p>The type of service job. For SageMaker Training jobs, specify <code>SAGEMAKER_TRAINING</code>.</p>
    pub fn get_service_job_type(&self) -> &::std::option::Option<crate::types::ServiceJobType> {
        &self.service_job_type
    }
    /// <p>The share identifier for the service job. Don't specify this parameter if the job queue doesn't have a fair- share scheduling policy. If the job queue has a fair-share scheduling policy, then this parameter must be specified.</p>
    pub fn share_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.share_identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The share identifier for the service job. Don't specify this parameter if the job queue doesn't have a fair- share scheduling policy. If the job queue has a fair-share scheduling policy, then this parameter must be specified.</p>
    pub fn set_share_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.share_identifier = input;
        self
    }
    /// <p>The share identifier for the service job. Don't specify this parameter if the job queue doesn't have a fair- share scheduling policy. If the job queue has a fair-share scheduling policy, then this parameter must be specified.</p>
    pub fn get_share_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.share_identifier
    }
    /// <p>The timeout configuration for the service job. If none is specified, Batch defers to the default timeout of the underlying service handling the job.</p>
    pub fn timeout_config(mut self, input: crate::types::ServiceJobTimeout) -> Self {
        self.timeout_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>The timeout configuration for the service job. If none is specified, Batch defers to the default timeout of the underlying service handling the job.</p>
    pub fn set_timeout_config(mut self, input: ::std::option::Option<crate::types::ServiceJobTimeout>) -> Self {
        self.timeout_config = input;
        self
    }
    /// <p>The timeout configuration for the service job. If none is specified, Batch defers to the default timeout of the underlying service handling the job.</p>
    pub fn get_timeout_config(&self) -> &::std::option::Option<crate::types::ServiceJobTimeout> {
        &self.timeout_config
    }
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags that you apply to the service job request. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your Batch resources</a>.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut hash_map = self.tags.unwrap_or_default();
        hash_map.insert(k.into(), v.into());
        self.tags = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>The tags that you apply to the service job request. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your Batch resources</a>.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.tags = input;
        self
    }
    /// <p>The tags that you apply to the service job request. Each tag consists of a key and an optional value. For more information, see <a href="https://docs.aws.amazon.com/batch/latest/userguide/using-tags.html">Tagging your Batch resources</a>.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        &self.tags
    }
    /// <p>A unique identifier for the request. This token is used to ensure idempotency of requests. If this parameter is specified and two submit requests with identical payloads and <code>clientToken</code>s are received, these requests are considered the same request and the second request is rejected.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique identifier for the request. This token is used to ensure idempotency of requests. If this parameter is specified and two submit requests with identical payloads and <code>clientToken</code>s are received, these requests are considered the same request and the second request is rejected.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// <p>A unique identifier for the request. This token is used to ensure idempotency of requests. If this parameter is specified and two submit requests with identical payloads and <code>clientToken</code>s are received, these requests are considered the same request and the second request is rejected.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.client_token
    }
    /// Consumes the builder and constructs a [`SubmitServiceJobInput`](crate::operation::submit_service_job::SubmitServiceJobInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::submit_service_job::SubmitServiceJobInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::submit_service_job::SubmitServiceJobInput {
            job_name: self.job_name,
            job_queue: self.job_queue,
            retry_strategy: self.retry_strategy,
            scheduling_priority: self.scheduling_priority,
            service_request_payload: self.service_request_payload,
            service_job_type: self.service_job_type,
            share_identifier: self.share_identifier,
            timeout_config: self.timeout_config,
            tags: self.tags,
            client_token: self.client_token,
        })
    }
}
