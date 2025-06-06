// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_person_tracking::_start_person_tracking_output::StartPersonTrackingOutputBuilder;

pub use crate::operation::start_person_tracking::_start_person_tracking_input::StartPersonTrackingInputBuilder;

impl crate::operation::start_person_tracking::builders::StartPersonTrackingInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::start_person_tracking::StartPersonTrackingOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_person_tracking::StartPersonTrackingError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.start_person_tracking();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StartPersonTracking`.
///
/// <note>
/// <p><i>End of support notice:</i> On October 31, 2025, AWS will discontinue support for Amazon Rekognition People Pathing. After October 31, 2025, you will no longer be able to use the Rekognition People Pathing capability. For more information, visit this <a href="https://aws.amazon.com/blogs/machine-learning/transitioning-from-amazon-rekognition-people-pathing-exploring-other-alternatives/">blog post</a>.</p>
/// </note>
/// <p>Starts the asynchronous tracking of a person's path in a stored video.</p>
/// <p>Amazon Rekognition Video can track the path of people in a video stored in an Amazon S3 bucket. Use <code>Video</code> to specify the bucket name and the filename of the video. <code>StartPersonTracking</code> returns a job identifier (<code>JobId</code>) which you use to get the results of the operation. When label detection is finished, Amazon Rekognition publishes a completion status to the Amazon Simple Notification Service topic that you specify in <code>NotificationChannel</code>.</p>
/// <p>To get the results of the person detection operation, first check that the status value published to the Amazon SNS topic is <code>SUCCEEDED</code>. If so, call <code>GetPersonTracking</code> and pass the job identifier (<code>JobId</code>) from the initial call to <code>StartPersonTracking</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartPersonTrackingFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_person_tracking::builders::StartPersonTrackingInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::start_person_tracking::StartPersonTrackingOutput,
        crate::operation::start_person_tracking::StartPersonTrackingError,
    > for StartPersonTrackingFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::start_person_tracking::StartPersonTrackingOutput,
            crate::operation::start_person_tracking::StartPersonTrackingError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl StartPersonTrackingFluentBuilder {
    /// Creates a new `StartPersonTrackingFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StartPersonTracking as a reference.
    pub fn as_input(&self) -> &crate::operation::start_person_tracking::builders::StartPersonTrackingInputBuilder {
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
        crate::operation::start_person_tracking::StartPersonTrackingOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_person_tracking::StartPersonTrackingError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::start_person_tracking::StartPersonTracking::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::start_person_tracking::StartPersonTracking::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::start_person_tracking::StartPersonTrackingOutput,
        crate::operation::start_person_tracking::StartPersonTrackingError,
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
    /// <p>The video in which you want to detect people. The video must be stored in an Amazon S3 bucket.</p>
    pub fn video(mut self, input: crate::types::Video) -> Self {
        self.inner = self.inner.video(input);
        self
    }
    /// <p>The video in which you want to detect people. The video must be stored in an Amazon S3 bucket.</p>
    pub fn set_video(mut self, input: ::std::option::Option<crate::types::Video>) -> Self {
        self.inner = self.inner.set_video(input);
        self
    }
    /// <p>The video in which you want to detect people. The video must be stored in an Amazon S3 bucket.</p>
    pub fn get_video(&self) -> &::std::option::Option<crate::types::Video> {
        self.inner.get_video()
    }
    /// <p>Idempotent token used to identify the start request. If you use the same token with multiple <code>StartPersonTracking</code> requests, the same <code>JobId</code> is returned. Use <code>ClientRequestToken</code> to prevent the same job from being accidently started more than once.</p>
    pub fn client_request_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_request_token(input.into());
        self
    }
    /// <p>Idempotent token used to identify the start request. If you use the same token with multiple <code>StartPersonTracking</code> requests, the same <code>JobId</code> is returned. Use <code>ClientRequestToken</code> to prevent the same job from being accidently started more than once.</p>
    pub fn set_client_request_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_request_token(input);
        self
    }
    /// <p>Idempotent token used to identify the start request. If you use the same token with multiple <code>StartPersonTracking</code> requests, the same <code>JobId</code> is returned. Use <code>ClientRequestToken</code> to prevent the same job from being accidently started more than once.</p>
    pub fn get_client_request_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_request_token()
    }
    /// <p>The Amazon SNS topic ARN you want Amazon Rekognition Video to publish the completion status of the people detection operation to. The Amazon SNS topic must have a topic name that begins with <i>AmazonRekognition</i> if you are using the AmazonRekognitionServiceRole permissions policy.</p>
    pub fn notification_channel(mut self, input: crate::types::NotificationChannel) -> Self {
        self.inner = self.inner.notification_channel(input);
        self
    }
    /// <p>The Amazon SNS topic ARN you want Amazon Rekognition Video to publish the completion status of the people detection operation to. The Amazon SNS topic must have a topic name that begins with <i>AmazonRekognition</i> if you are using the AmazonRekognitionServiceRole permissions policy.</p>
    pub fn set_notification_channel(mut self, input: ::std::option::Option<crate::types::NotificationChannel>) -> Self {
        self.inner = self.inner.set_notification_channel(input);
        self
    }
    /// <p>The Amazon SNS topic ARN you want Amazon Rekognition Video to publish the completion status of the people detection operation to. The Amazon SNS topic must have a topic name that begins with <i>AmazonRekognition</i> if you are using the AmazonRekognitionServiceRole permissions policy.</p>
    pub fn get_notification_channel(&self) -> &::std::option::Option<crate::types::NotificationChannel> {
        self.inner.get_notification_channel()
    }
    /// <p>An identifier you specify that's returned in the completion notification that's published to your Amazon Simple Notification Service topic. For example, you can use <code>JobTag</code> to group related jobs and identify them in the completion notification.</p>
    pub fn job_tag(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.job_tag(input.into());
        self
    }
    /// <p>An identifier you specify that's returned in the completion notification that's published to your Amazon Simple Notification Service topic. For example, you can use <code>JobTag</code> to group related jobs and identify them in the completion notification.</p>
    pub fn set_job_tag(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_job_tag(input);
        self
    }
    /// <p>An identifier you specify that's returned in the completion notification that's published to your Amazon Simple Notification Service topic. For example, you can use <code>JobTag</code> to group related jobs and identify them in the completion notification.</p>
    pub fn get_job_tag(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_job_tag()
    }
}
