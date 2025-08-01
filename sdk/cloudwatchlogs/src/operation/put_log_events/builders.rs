// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::put_log_events::_put_log_events_output::PutLogEventsOutputBuilder;

pub use crate::operation::put_log_events::_put_log_events_input::PutLogEventsInputBuilder;

impl crate::operation::put_log_events::builders::PutLogEventsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::put_log_events::PutLogEventsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_log_events::PutLogEventsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.put_log_events();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `PutLogEvents`.
///
/// <p>Uploads a batch of log events to the specified log stream.</p><important>
/// <p>The sequence token is now ignored in <code>PutLogEvents</code> actions. <code>PutLogEvents</code> actions are always accepted and never return <code>InvalidSequenceTokenException</code> or <code>DataAlreadyAcceptedException</code> even if the sequence token is not valid. You can use parallel <code>PutLogEvents</code> actions on the same log stream.</p>
/// </important>
/// <p>The batch of events must satisfy the following constraints:</p>
/// <ul>
/// <li>
/// <p>The maximum batch size is 1,048,576 bytes. This size is calculated as the sum of all event messages in UTF-8, plus 26 bytes for each log event.</p></li>
/// <li>
/// <p>Events more than 2 hours in the future are rejected while processing remaining valid events.</p></li>
/// <li>
/// <p>Events older than 14 days or preceding the log group's retention period are rejected while processing remaining valid events.</p></li>
/// <li>
/// <p>The log events in the batch must be in chronological order by their timestamp. The timestamp is the time that the event occurred, expressed as the number of milliseconds after <code>Jan 1, 1970 00:00:00 UTC</code>. (In Amazon Web Services Tools for PowerShell and the Amazon Web Services SDK for .NET, the timestamp is specified in .NET format: <code>yyyy-mm-ddThh:mm:ss</code>. For example, <code>2017-09-15T13:45:30</code>.)</p></li>
/// <li>
/// <p>A batch of log events in a single request must be in a chronological order. Otherwise, the operation fails.</p></li>
/// <li>
/// <p>Each log event can be no larger than 1 MB.</p></li>
/// <li>
/// <p>The maximum number of log events in a batch is 10,000.</p></li>
/// <li>
/// <p>For valid events (within 14 days in the past to 2 hours in future), the time span in a single batch cannot exceed 24 hours. Otherwise, the operation fails.</p></li>
/// </ul><important>
/// <p>The quota of five requests per second per log stream has been removed. Instead, <code>PutLogEvents</code> actions are throttled based on a per-second per-account quota. You can request an increase to the per-second throttling quota by using the Service Quotas service.</p>
/// </important>
/// <p>If a call to <code>PutLogEvents</code> returns "UnrecognizedClientException" the most likely cause is a non-valid Amazon Web Services access key ID or secret key.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct PutLogEventsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::put_log_events::builders::PutLogEventsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::put_log_events::PutLogEventsOutput,
        crate::operation::put_log_events::PutLogEventsError,
    > for PutLogEventsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::put_log_events::PutLogEventsOutput,
            crate::operation::put_log_events::PutLogEventsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl PutLogEventsFluentBuilder {
    /// Creates a new `PutLogEventsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the PutLogEvents as a reference.
    pub fn as_input(&self) -> &crate::operation::put_log_events::builders::PutLogEventsInputBuilder {
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
        crate::operation::put_log_events::PutLogEventsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::put_log_events::PutLogEventsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::put_log_events::PutLogEvents::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::put_log_events::PutLogEvents::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::put_log_events::PutLogEventsOutput,
        crate::operation::put_log_events::PutLogEventsError,
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
    /// <p>The name of the log group.</p>
    pub fn log_group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.log_group_name(input.into());
        self
    }
    /// <p>The name of the log group.</p>
    pub fn set_log_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_log_group_name(input);
        self
    }
    /// <p>The name of the log group.</p>
    pub fn get_log_group_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_log_group_name()
    }
    /// <p>The name of the log stream.</p>
    pub fn log_stream_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.log_stream_name(input.into());
        self
    }
    /// <p>The name of the log stream.</p>
    pub fn set_log_stream_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_log_stream_name(input);
        self
    }
    /// <p>The name of the log stream.</p>
    pub fn get_log_stream_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_log_stream_name()
    }
    ///
    /// Appends an item to `logEvents`.
    ///
    /// To override the contents of this collection use [`set_log_events`](Self::set_log_events).
    ///
    /// <p>The log events.</p>
    pub fn log_events(mut self, input: crate::types::InputLogEvent) -> Self {
        self.inner = self.inner.log_events(input);
        self
    }
    /// <p>The log events.</p>
    pub fn set_log_events(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::InputLogEvent>>) -> Self {
        self.inner = self.inner.set_log_events(input);
        self
    }
    /// <p>The log events.</p>
    pub fn get_log_events(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::InputLogEvent>> {
        self.inner.get_log_events()
    }
    /// <p>The sequence token obtained from the response of the previous <code>PutLogEvents</code> call.</p><important>
    /// <p>The <code>sequenceToken</code> parameter is now ignored in <code>PutLogEvents</code> actions. <code>PutLogEvents</code> actions are now accepted and never return <code>InvalidSequenceTokenException</code> or <code>DataAlreadyAcceptedException</code> even if the sequence token is not valid.</p>
    /// </important>
    pub fn sequence_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.sequence_token(input.into());
        self
    }
    /// <p>The sequence token obtained from the response of the previous <code>PutLogEvents</code> call.</p><important>
    /// <p>The <code>sequenceToken</code> parameter is now ignored in <code>PutLogEvents</code> actions. <code>PutLogEvents</code> actions are now accepted and never return <code>InvalidSequenceTokenException</code> or <code>DataAlreadyAcceptedException</code> even if the sequence token is not valid.</p>
    /// </important>
    pub fn set_sequence_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_sequence_token(input);
        self
    }
    /// <p>The sequence token obtained from the response of the previous <code>PutLogEvents</code> call.</p><important>
    /// <p>The <code>sequenceToken</code> parameter is now ignored in <code>PutLogEvents</code> actions. <code>PutLogEvents</code> actions are now accepted and never return <code>InvalidSequenceTokenException</code> or <code>DataAlreadyAcceptedException</code> even if the sequence token is not valid.</p>
    /// </important>
    pub fn get_sequence_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_sequence_token()
    }
    /// <p>The entity associated with the log events.</p>
    pub fn entity(mut self, input: crate::types::Entity) -> Self {
        self.inner = self.inner.entity(input);
        self
    }
    /// <p>The entity associated with the log events.</p>
    pub fn set_entity(mut self, input: ::std::option::Option<crate::types::Entity>) -> Self {
        self.inner = self.inner.set_entity(input);
        self
    }
    /// <p>The entity associated with the log events.</p>
    pub fn get_entity(&self) -> &::std::option::Option<crate::types::Entity> {
        self.inner.get_entity()
    }
}
