// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_event_rule::_create_event_rule_output::CreateEventRuleOutputBuilder;

pub use crate::operation::create_event_rule::_create_event_rule_input::CreateEventRuleInputBuilder;

impl crate::operation::create_event_rule::builders::CreateEventRuleInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_event_rule::CreateEventRuleOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_event_rule::CreateEventRuleError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_event_rule();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateEventRule`.
///
/// <p>Creates an <a href="https://docs.aws.amazon.com/notifications/latest/userguide/glossary.html"> <code>EventRule</code> </a> that is associated with a specified <code>NotificationConfiguration</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateEventRuleFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_event_rule::builders::CreateEventRuleInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_event_rule::CreateEventRuleOutput,
        crate::operation::create_event_rule::CreateEventRuleError,
    > for CreateEventRuleFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_event_rule::CreateEventRuleOutput,
            crate::operation::create_event_rule::CreateEventRuleError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateEventRuleFluentBuilder {
    /// Creates a new `CreateEventRuleFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateEventRule as a reference.
    pub fn as_input(&self) -> &crate::operation::create_event_rule::builders::CreateEventRuleInputBuilder {
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
        crate::operation::create_event_rule::CreateEventRuleOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_event_rule::CreateEventRuleError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_event_rule::CreateEventRule::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_event_rule::CreateEventRule::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_event_rule::CreateEventRuleOutput,
        crate::operation::create_event_rule::CreateEventRuleError,
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
    /// <p>The Amazon Resource Name (ARN) of the <code>NotificationConfiguration</code> associated with this <code>EventRule</code>.</p>
    pub fn notification_configuration_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.notification_configuration_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the <code>NotificationConfiguration</code> associated with this <code>EventRule</code>.</p>
    pub fn set_notification_configuration_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_notification_configuration_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the <code>NotificationConfiguration</code> associated with this <code>EventRule</code>.</p>
    pub fn get_notification_configuration_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_notification_configuration_arn()
    }
    /// <p>The matched event source.</p>
    /// <p>Must match one of the valid EventBridge sources. Only Amazon Web Services service sourced events are supported. For example, <code>aws.ec2</code> and <code>aws.cloudwatch</code>. For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-service-event.html#eb-service-event-delivery-level">Event delivery from Amazon Web Services services</a> in the <i>Amazon EventBridge User Guide</i>.</p>
    pub fn source(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.source(input.into());
        self
    }
    /// <p>The matched event source.</p>
    /// <p>Must match one of the valid EventBridge sources. Only Amazon Web Services service sourced events are supported. For example, <code>aws.ec2</code> and <code>aws.cloudwatch</code>. For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-service-event.html#eb-service-event-delivery-level">Event delivery from Amazon Web Services services</a> in the <i>Amazon EventBridge User Guide</i>.</p>
    pub fn set_source(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_source(input);
        self
    }
    /// <p>The matched event source.</p>
    /// <p>Must match one of the valid EventBridge sources. Only Amazon Web Services service sourced events are supported. For example, <code>aws.ec2</code> and <code>aws.cloudwatch</code>. For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-service-event.html#eb-service-event-delivery-level">Event delivery from Amazon Web Services services</a> in the <i>Amazon EventBridge User Guide</i>.</p>
    pub fn get_source(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_source()
    }
    /// <p>The event type to match.</p>
    /// <p>Must match one of the valid Amazon EventBridge event types. For example, EC2 Instance State-change Notification and Amazon CloudWatch Alarm State Change. For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-service-event.html#eb-service-event-delivery-level">Event delivery from Amazon Web Services services</a> in the <i>Amazon EventBridge User Guide</i>.</p>
    pub fn event_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.event_type(input.into());
        self
    }
    /// <p>The event type to match.</p>
    /// <p>Must match one of the valid Amazon EventBridge event types. For example, EC2 Instance State-change Notification and Amazon CloudWatch Alarm State Change. For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-service-event.html#eb-service-event-delivery-level">Event delivery from Amazon Web Services services</a> in the <i>Amazon EventBridge User Guide</i>.</p>
    pub fn set_event_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_event_type(input);
        self
    }
    /// <p>The event type to match.</p>
    /// <p>Must match one of the valid Amazon EventBridge event types. For example, EC2 Instance State-change Notification and Amazon CloudWatch Alarm State Change. For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-service-event.html#eb-service-event-delivery-level">Event delivery from Amazon Web Services services</a> in the <i>Amazon EventBridge User Guide</i>.</p>
    pub fn get_event_type(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_event_type()
    }
    /// <p>An additional event pattern used to further filter the events this <code>EventRule</code> receives.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-event-patterns.html">Amazon EventBridge event patterns</a> in the <i>Amazon EventBridge User Guide.</i></p>
    pub fn event_pattern(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.event_pattern(input.into());
        self
    }
    /// <p>An additional event pattern used to further filter the events this <code>EventRule</code> receives.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-event-patterns.html">Amazon EventBridge event patterns</a> in the <i>Amazon EventBridge User Guide.</i></p>
    pub fn set_event_pattern(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_event_pattern(input);
        self
    }
    /// <p>An additional event pattern used to further filter the events this <code>EventRule</code> receives.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/eventbridge/latest/userguide/eb-event-patterns.html">Amazon EventBridge event patterns</a> in the <i>Amazon EventBridge User Guide.</i></p>
    pub fn get_event_pattern(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_event_pattern()
    }
    ///
    /// Appends an item to `regions`.
    ///
    /// To override the contents of this collection use [`set_regions`](Self::set_regions).
    ///
    /// <p>A list of Amazon Web Services Regions that send events to this <code>EventRule</code>.</p>
    pub fn regions(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.regions(input.into());
        self
    }
    /// <p>A list of Amazon Web Services Regions that send events to this <code>EventRule</code>.</p>
    pub fn set_regions(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_regions(input);
        self
    }
    /// <p>A list of Amazon Web Services Regions that send events to this <code>EventRule</code>.</p>
    pub fn get_regions(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_regions()
    }
}
