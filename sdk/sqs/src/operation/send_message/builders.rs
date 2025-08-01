// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::send_message::_send_message_output::SendMessageOutputBuilder;

pub use crate::operation::send_message::_send_message_input::SendMessageInputBuilder;

impl crate::operation::send_message::builders::SendMessageInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::send_message::SendMessageOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::send_message::SendMessageError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.send_message();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `SendMessage`.
///
/// <p>Delivers a message to the specified queue.</p><important>
/// <p>A message can include only XML, JSON, and unformatted text. The following Unicode characters are allowed. For more information, see the <a href="http://www.w3.org/TR/REC-xml/#charsets">W3C specification for characters</a>.</p>
/// <p><code>#x9</code> | <code>#xA</code> | <code>#xD</code> | <code>#x20</code> to <code>#xD7FF</code> | <code>#xE000</code> to <code>#xFFFD</code> | <code>#x10000</code> to <code>#x10FFFF</code></p>
/// <p>Amazon SQS does not throw an exception or completely reject the message if it contains invalid characters. Instead, it replaces those invalid characters with U+FFFD before storing the message in the queue, as long as the message body contains at least one valid character.</p>
/// </important>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct SendMessageFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::send_message::builders::SendMessageInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::send_message::SendMessageOutput,
        crate::operation::send_message::SendMessageError,
    > for SendMessageFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::send_message::SendMessageOutput,
            crate::operation::send_message::SendMessageError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl SendMessageFluentBuilder {
    /// Creates a new `SendMessageFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the SendMessage as a reference.
    pub fn as_input(&self) -> &crate::operation::send_message::builders::SendMessageInputBuilder {
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
        crate::operation::send_message::SendMessageOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::send_message::SendMessageError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::send_message::SendMessage::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::send_message::SendMessage::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::send_message::SendMessageOutput,
        crate::operation::send_message::SendMessageError,
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
    /// <p>The URL of the Amazon SQS queue to which a message is sent.</p>
    /// <p>Queue URLs and names are case-sensitive.</p>
    pub fn queue_url(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.queue_url(input.into());
        self
    }
    /// <p>The URL of the Amazon SQS queue to which a message is sent.</p>
    /// <p>Queue URLs and names are case-sensitive.</p>
    pub fn set_queue_url(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_queue_url(input);
        self
    }
    /// <p>The URL of the Amazon SQS queue to which a message is sent.</p>
    /// <p>Queue URLs and names are case-sensitive.</p>
    pub fn get_queue_url(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_queue_url()
    }
    /// <p>The message to send. The minimum size is one character. The maximum size is 256 KiB.</p><important>
    /// <p>A message can include only XML, JSON, and unformatted text. The following Unicode characters are allowed. For more information, see the <a href="http://www.w3.org/TR/REC-xml/#charsets">W3C specification for characters</a>.</p>
    /// <p><code>#x9</code> | <code>#xA</code> | <code>#xD</code> | <code>#x20</code> to <code>#xD7FF</code> | <code>#xE000</code> to <code>#xFFFD</code> | <code>#x10000</code> to <code>#x10FFFF</code></p>
    /// <p>Amazon SQS does not throw an exception or completely reject the message if it contains invalid characters. Instead, it replaces those invalid characters with U+FFFD before storing the message in the queue, as long as the message body contains at least one valid character.</p>
    /// </important>
    pub fn message_body(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.message_body(input.into());
        self
    }
    /// <p>The message to send. The minimum size is one character. The maximum size is 256 KiB.</p><important>
    /// <p>A message can include only XML, JSON, and unformatted text. The following Unicode characters are allowed. For more information, see the <a href="http://www.w3.org/TR/REC-xml/#charsets">W3C specification for characters</a>.</p>
    /// <p><code>#x9</code> | <code>#xA</code> | <code>#xD</code> | <code>#x20</code> to <code>#xD7FF</code> | <code>#xE000</code> to <code>#xFFFD</code> | <code>#x10000</code> to <code>#x10FFFF</code></p>
    /// <p>Amazon SQS does not throw an exception or completely reject the message if it contains invalid characters. Instead, it replaces those invalid characters with U+FFFD before storing the message in the queue, as long as the message body contains at least one valid character.</p>
    /// </important>
    pub fn set_message_body(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_message_body(input);
        self
    }
    /// <p>The message to send. The minimum size is one character. The maximum size is 256 KiB.</p><important>
    /// <p>A message can include only XML, JSON, and unformatted text. The following Unicode characters are allowed. For more information, see the <a href="http://www.w3.org/TR/REC-xml/#charsets">W3C specification for characters</a>.</p>
    /// <p><code>#x9</code> | <code>#xA</code> | <code>#xD</code> | <code>#x20</code> to <code>#xD7FF</code> | <code>#xE000</code> to <code>#xFFFD</code> | <code>#x10000</code> to <code>#x10FFFF</code></p>
    /// <p>Amazon SQS does not throw an exception or completely reject the message if it contains invalid characters. Instead, it replaces those invalid characters with U+FFFD before storing the message in the queue, as long as the message body contains at least one valid character.</p>
    /// </important>
    pub fn get_message_body(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_message_body()
    }
    /// <p>The length of time, in seconds, for which to delay a specific message. Valid values: 0 to 900. Maximum: 15 minutes. Messages with a positive <code>DelaySeconds</code> value become available for processing after the delay period is finished. If you don't specify a value, the default value for the queue applies.</p><note>
    /// <p>When you set <code>FifoQueue</code>, you can't set <code>DelaySeconds</code> per message. You can set this parameter only on a queue level.</p>
    /// </note>
    pub fn delay_seconds(mut self, input: i32) -> Self {
        self.inner = self.inner.delay_seconds(input);
        self
    }
    /// <p>The length of time, in seconds, for which to delay a specific message. Valid values: 0 to 900. Maximum: 15 minutes. Messages with a positive <code>DelaySeconds</code> value become available for processing after the delay period is finished. If you don't specify a value, the default value for the queue applies.</p><note>
    /// <p>When you set <code>FifoQueue</code>, you can't set <code>DelaySeconds</code> per message. You can set this parameter only on a queue level.</p>
    /// </note>
    pub fn set_delay_seconds(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_delay_seconds(input);
        self
    }
    /// <p>The length of time, in seconds, for which to delay a specific message. Valid values: 0 to 900. Maximum: 15 minutes. Messages with a positive <code>DelaySeconds</code> value become available for processing after the delay period is finished. If you don't specify a value, the default value for the queue applies.</p><note>
    /// <p>When you set <code>FifoQueue</code>, you can't set <code>DelaySeconds</code> per message. You can set this parameter only on a queue level.</p>
    /// </note>
    pub fn get_delay_seconds(&self) -> &::std::option::Option<i32> {
        self.inner.get_delay_seconds()
    }
    ///
    /// Adds a key-value pair to `MessageAttributes`.
    ///
    /// To override the contents of this collection use [`set_message_attributes`](Self::set_message_attributes).
    ///
    /// <p>Each message attribute consists of a <code>Name</code>, <code>Type</code>, and <code>Value</code>. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-message-metadata.html#sqs-message-attributes">Amazon SQS message attributes</a> in the <i>Amazon SQS Developer Guide</i>.</p>
    pub fn message_attributes(mut self, k: impl ::std::convert::Into<::std::string::String>, v: crate::types::MessageAttributeValue) -> Self {
        self.inner = self.inner.message_attributes(k.into(), v);
        self
    }
    /// <p>Each message attribute consists of a <code>Name</code>, <code>Type</code>, and <code>Value</code>. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-message-metadata.html#sqs-message-attributes">Amazon SQS message attributes</a> in the <i>Amazon SQS Developer Guide</i>.</p>
    pub fn set_message_attributes(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::MessageAttributeValue>>,
    ) -> Self {
        self.inner = self.inner.set_message_attributes(input);
        self
    }
    /// <p>Each message attribute consists of a <code>Name</code>, <code>Type</code>, and <code>Value</code>. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-message-metadata.html#sqs-message-attributes">Amazon SQS message attributes</a> in the <i>Amazon SQS Developer Guide</i>.</p>
    pub fn get_message_attributes(
        &self,
    ) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::MessageAttributeValue>> {
        self.inner.get_message_attributes()
    }
    ///
    /// Adds a key-value pair to `MessageSystemAttributes`.
    ///
    /// To override the contents of this collection use [`set_message_system_attributes`](Self::set_message_system_attributes).
    ///
    /// <p>The message system attribute to send. Each message system attribute consists of a <code>Name</code>, <code>Type</code>, and <code>Value</code>.</p><important>
    /// <ul>
    /// <li>
    /// <p>Currently, the only supported message system attribute is <code>AWSTraceHeader</code>. Its type must be <code>String</code> and its value must be a correctly formatted X-Ray trace header string.</p></li>
    /// <li>
    /// <p>The size of a message system attribute doesn't count towards the total size of a message.</p></li>
    /// </ul>
    /// </important>
    pub fn message_system_attributes(
        mut self,
        k: crate::types::MessageSystemAttributeNameForSends,
        v: crate::types::MessageSystemAttributeValue,
    ) -> Self {
        self.inner = self.inner.message_system_attributes(k, v);
        self
    }
    /// <p>The message system attribute to send. Each message system attribute consists of a <code>Name</code>, <code>Type</code>, and <code>Value</code>.</p><important>
    /// <ul>
    /// <li>
    /// <p>Currently, the only supported message system attribute is <code>AWSTraceHeader</code>. Its type must be <code>String</code> and its value must be a correctly formatted X-Ray trace header string.</p></li>
    /// <li>
    /// <p>The size of a message system attribute doesn't count towards the total size of a message.</p></li>
    /// </ul>
    /// </important>
    pub fn set_message_system_attributes(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<crate::types::MessageSystemAttributeNameForSends, crate::types::MessageSystemAttributeValue>,
        >,
    ) -> Self {
        self.inner = self.inner.set_message_system_attributes(input);
        self
    }
    /// <p>The message system attribute to send. Each message system attribute consists of a <code>Name</code>, <code>Type</code>, and <code>Value</code>.</p><important>
    /// <ul>
    /// <li>
    /// <p>Currently, the only supported message system attribute is <code>AWSTraceHeader</code>. Its type must be <code>String</code> and its value must be a correctly formatted X-Ray trace header string.</p></li>
    /// <li>
    /// <p>The size of a message system attribute doesn't count towards the total size of a message.</p></li>
    /// </ul>
    /// </important>
    pub fn get_message_system_attributes(
        &self,
    ) -> &::std::option::Option<
        ::std::collections::HashMap<crate::types::MessageSystemAttributeNameForSends, crate::types::MessageSystemAttributeValue>,
    > {
        self.inner.get_message_system_attributes()
    }
    /// <p>This parameter applies only to FIFO (first-in-first-out) queues.</p>
    /// <p>The token used for deduplication of sent messages. If a message with a particular <code>MessageDeduplicationId</code> is sent successfully, any messages sent with the same <code>MessageDeduplicationId</code> are accepted successfully but aren't delivered during the 5-minute deduplication interval. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues-exactly-once-processing.html"> Exactly-once processing</a> in the <i>Amazon SQS Developer Guide</i>.</p>
    /// <ul>
    /// <li>
    /// <p>Every message must have a unique <code>MessageDeduplicationId</code>,</p>
    /// <ul>
    /// <li>
    /// <p>You may provide a <code>MessageDeduplicationId</code> explicitly.</p></li>
    /// <li>
    /// <p>If you aren't able to provide a <code>MessageDeduplicationId</code> and you enable <code>ContentBasedDeduplication</code> for your queue, Amazon SQS uses a SHA-256 hash to generate the <code>MessageDeduplicationId</code> using the body of the message (but not the attributes of the message).</p></li>
    /// <li>
    /// <p>If you don't provide a <code>MessageDeduplicationId</code> and the queue doesn't have <code>ContentBasedDeduplication</code> set, the action fails with an error.</p></li>
    /// <li>
    /// <p>If the queue has <code>ContentBasedDeduplication</code> set, your <code>MessageDeduplicationId</code> overrides the generated one.</p></li>
    /// </ul></li>
    /// <li>
    /// <p>When <code>ContentBasedDeduplication</code> is in effect, messages with identical content sent within the deduplication interval are treated as duplicates and only one copy of the message is delivered.</p></li>
    /// <li>
    /// <p>If you send one message with <code>ContentBasedDeduplication</code> enabled and then another message with a <code>MessageDeduplicationId</code> that is the same as the one generated for the first <code>MessageDeduplicationId</code>, the two messages are treated as duplicates and only one copy of the message is delivered.</p></li>
    /// </ul><note>
    /// <p>The <code>MessageDeduplicationId</code> is available to the consumer of the message (this can be useful for troubleshooting delivery issues).</p>
    /// <p>If a message is sent successfully but the acknowledgement is lost and the message is resent with the same <code>MessageDeduplicationId</code> after the deduplication interval, Amazon SQS can't detect duplicate messages.</p>
    /// <p>Amazon SQS continues to keep track of the message deduplication ID even after the message is received and deleted.</p>
    /// </note>
    /// <p>The maximum length of <code>MessageDeduplicationId</code> is 128 characters. <code>MessageDeduplicationId</code> can contain alphanumeric characters (<code>a-z</code>, <code>A-Z</code>, <code>0-9</code>) and punctuation (<code>!"#$%&amp;'()*+,-./:;&lt;=&gt;?@\[\\]^_`{|}~</code>).</p>
    /// <p>For best practices of using <code>MessageDeduplicationId</code>, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/using-messagededuplicationid-property.html">Using the MessageDeduplicationId Property</a> in the <i>Amazon SQS Developer Guide</i>.</p>
    pub fn message_deduplication_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.message_deduplication_id(input.into());
        self
    }
    /// <p>This parameter applies only to FIFO (first-in-first-out) queues.</p>
    /// <p>The token used for deduplication of sent messages. If a message with a particular <code>MessageDeduplicationId</code> is sent successfully, any messages sent with the same <code>MessageDeduplicationId</code> are accepted successfully but aren't delivered during the 5-minute deduplication interval. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues-exactly-once-processing.html"> Exactly-once processing</a> in the <i>Amazon SQS Developer Guide</i>.</p>
    /// <ul>
    /// <li>
    /// <p>Every message must have a unique <code>MessageDeduplicationId</code>,</p>
    /// <ul>
    /// <li>
    /// <p>You may provide a <code>MessageDeduplicationId</code> explicitly.</p></li>
    /// <li>
    /// <p>If you aren't able to provide a <code>MessageDeduplicationId</code> and you enable <code>ContentBasedDeduplication</code> for your queue, Amazon SQS uses a SHA-256 hash to generate the <code>MessageDeduplicationId</code> using the body of the message (but not the attributes of the message).</p></li>
    /// <li>
    /// <p>If you don't provide a <code>MessageDeduplicationId</code> and the queue doesn't have <code>ContentBasedDeduplication</code> set, the action fails with an error.</p></li>
    /// <li>
    /// <p>If the queue has <code>ContentBasedDeduplication</code> set, your <code>MessageDeduplicationId</code> overrides the generated one.</p></li>
    /// </ul></li>
    /// <li>
    /// <p>When <code>ContentBasedDeduplication</code> is in effect, messages with identical content sent within the deduplication interval are treated as duplicates and only one copy of the message is delivered.</p></li>
    /// <li>
    /// <p>If you send one message with <code>ContentBasedDeduplication</code> enabled and then another message with a <code>MessageDeduplicationId</code> that is the same as the one generated for the first <code>MessageDeduplicationId</code>, the two messages are treated as duplicates and only one copy of the message is delivered.</p></li>
    /// </ul><note>
    /// <p>The <code>MessageDeduplicationId</code> is available to the consumer of the message (this can be useful for troubleshooting delivery issues).</p>
    /// <p>If a message is sent successfully but the acknowledgement is lost and the message is resent with the same <code>MessageDeduplicationId</code> after the deduplication interval, Amazon SQS can't detect duplicate messages.</p>
    /// <p>Amazon SQS continues to keep track of the message deduplication ID even after the message is received and deleted.</p>
    /// </note>
    /// <p>The maximum length of <code>MessageDeduplicationId</code> is 128 characters. <code>MessageDeduplicationId</code> can contain alphanumeric characters (<code>a-z</code>, <code>A-Z</code>, <code>0-9</code>) and punctuation (<code>!"#$%&amp;'()*+,-./:;&lt;=&gt;?@\[\\]^_`{|}~</code>).</p>
    /// <p>For best practices of using <code>MessageDeduplicationId</code>, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/using-messagededuplicationid-property.html">Using the MessageDeduplicationId Property</a> in the <i>Amazon SQS Developer Guide</i>.</p>
    pub fn set_message_deduplication_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_message_deduplication_id(input);
        self
    }
    /// <p>This parameter applies only to FIFO (first-in-first-out) queues.</p>
    /// <p>The token used for deduplication of sent messages. If a message with a particular <code>MessageDeduplicationId</code> is sent successfully, any messages sent with the same <code>MessageDeduplicationId</code> are accepted successfully but aren't delivered during the 5-minute deduplication interval. For more information, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/FIFO-queues-exactly-once-processing.html"> Exactly-once processing</a> in the <i>Amazon SQS Developer Guide</i>.</p>
    /// <ul>
    /// <li>
    /// <p>Every message must have a unique <code>MessageDeduplicationId</code>,</p>
    /// <ul>
    /// <li>
    /// <p>You may provide a <code>MessageDeduplicationId</code> explicitly.</p></li>
    /// <li>
    /// <p>If you aren't able to provide a <code>MessageDeduplicationId</code> and you enable <code>ContentBasedDeduplication</code> for your queue, Amazon SQS uses a SHA-256 hash to generate the <code>MessageDeduplicationId</code> using the body of the message (but not the attributes of the message).</p></li>
    /// <li>
    /// <p>If you don't provide a <code>MessageDeduplicationId</code> and the queue doesn't have <code>ContentBasedDeduplication</code> set, the action fails with an error.</p></li>
    /// <li>
    /// <p>If the queue has <code>ContentBasedDeduplication</code> set, your <code>MessageDeduplicationId</code> overrides the generated one.</p></li>
    /// </ul></li>
    /// <li>
    /// <p>When <code>ContentBasedDeduplication</code> is in effect, messages with identical content sent within the deduplication interval are treated as duplicates and only one copy of the message is delivered.</p></li>
    /// <li>
    /// <p>If you send one message with <code>ContentBasedDeduplication</code> enabled and then another message with a <code>MessageDeduplicationId</code> that is the same as the one generated for the first <code>MessageDeduplicationId</code>, the two messages are treated as duplicates and only one copy of the message is delivered.</p></li>
    /// </ul><note>
    /// <p>The <code>MessageDeduplicationId</code> is available to the consumer of the message (this can be useful for troubleshooting delivery issues).</p>
    /// <p>If a message is sent successfully but the acknowledgement is lost and the message is resent with the same <code>MessageDeduplicationId</code> after the deduplication interval, Amazon SQS can't detect duplicate messages.</p>
    /// <p>Amazon SQS continues to keep track of the message deduplication ID even after the message is received and deleted.</p>
    /// </note>
    /// <p>The maximum length of <code>MessageDeduplicationId</code> is 128 characters. <code>MessageDeduplicationId</code> can contain alphanumeric characters (<code>a-z</code>, <code>A-Z</code>, <code>0-9</code>) and punctuation (<code>!"#$%&amp;'()*+,-./:;&lt;=&gt;?@\[\\]^_`{|}~</code>).</p>
    /// <p>For best practices of using <code>MessageDeduplicationId</code>, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/using-messagededuplicationid-property.html">Using the MessageDeduplicationId Property</a> in the <i>Amazon SQS Developer Guide</i>.</p>
    pub fn get_message_deduplication_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_message_deduplication_id()
    }
    /// <p><code>MessageGroupId</code> is an attribute used in Amazon SQS FIFO (First-In-First-Out) and standard queues. In FIFO queues, <code>MessageGroupId</code> organizes messages into distinct groups. Messages within the same message group are always processed one at a time, in strict order, ensuring that no two messages from the same group are processed simultaneously. In standard queues, using <code>MessageGroupId</code> enables fair queues. It is used to identify the tenant a message belongs to, helping maintain consistent message dwell time across all tenants during noisy neighbor events. Unlike FIFO queues, messages with the same <code>MessageGroupId</code> can be processed in parallel, maintaining the high throughput of standard queues.</p>
    /// <ul>
    /// <li>
    /// <p><b>FIFO queues:</b> <code>MessageGroupId</code> acts as the tag that specifies that a message belongs to a specific message group. Messages that belong to the same message group are processed in a FIFO manner (however, messages in different message groups might be processed out of order). To interleave multiple ordered streams within a single queue, use <code>MessageGroupId</code> values (for example, session data for multiple users). In this scenario, multiple consumers can process the queue, but the session data of each user is processed in a FIFO fashion.</p>
    /// <p>If you do not provide a <code>MessageGroupId</code> when sending a message to a FIFO queue, the action fails.</p>
    /// <p><code>ReceiveMessage</code> might return messages with multiple <code>MessageGroupId</code> values. For each <code>MessageGroupId</code>, the messages are sorted by time sent.</p></li>
    /// <li>
    /// <p><b>Standard queues:</b>Use <code>MessageGroupId</code> in standard queues to enable fair queues. The <code>MessageGroupId</code> identifies the tenant a message belongs to. A tenant can be any entity that shares a queue with others, such as your customer, a client application, or a request type. When one tenant sends a disproportionately large volume of messages or has messages that require longer processing time, fair queues ensure other tenants' messages maintain low dwell time. This preserves quality of service for all tenants while maintaining the scalability and throughput of standard queues. We recommend that you include a <code>MessageGroupId</code> in all messages when using fair queues.</p></li>
    /// </ul>
    /// <p>The length of <code>MessageGroupId</code> is 128 characters. Valid values: alphanumeric characters and punctuation <code>(!"#$%&amp;'()*+,-./:;&lt;=&gt;?@\[\\]^_`{|}~)</code>.</p>
    /// <p>For best practices of using <code>MessageGroupId</code>, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/using-messagegroupid-property.html">Using the MessageGroupId Property</a> in the <i>Amazon SQS Developer Guide</i>.</p>
    pub fn message_group_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.message_group_id(input.into());
        self
    }
    /// <p><code>MessageGroupId</code> is an attribute used in Amazon SQS FIFO (First-In-First-Out) and standard queues. In FIFO queues, <code>MessageGroupId</code> organizes messages into distinct groups. Messages within the same message group are always processed one at a time, in strict order, ensuring that no two messages from the same group are processed simultaneously. In standard queues, using <code>MessageGroupId</code> enables fair queues. It is used to identify the tenant a message belongs to, helping maintain consistent message dwell time across all tenants during noisy neighbor events. Unlike FIFO queues, messages with the same <code>MessageGroupId</code> can be processed in parallel, maintaining the high throughput of standard queues.</p>
    /// <ul>
    /// <li>
    /// <p><b>FIFO queues:</b> <code>MessageGroupId</code> acts as the tag that specifies that a message belongs to a specific message group. Messages that belong to the same message group are processed in a FIFO manner (however, messages in different message groups might be processed out of order). To interleave multiple ordered streams within a single queue, use <code>MessageGroupId</code> values (for example, session data for multiple users). In this scenario, multiple consumers can process the queue, but the session data of each user is processed in a FIFO fashion.</p>
    /// <p>If you do not provide a <code>MessageGroupId</code> when sending a message to a FIFO queue, the action fails.</p>
    /// <p><code>ReceiveMessage</code> might return messages with multiple <code>MessageGroupId</code> values. For each <code>MessageGroupId</code>, the messages are sorted by time sent.</p></li>
    /// <li>
    /// <p><b>Standard queues:</b>Use <code>MessageGroupId</code> in standard queues to enable fair queues. The <code>MessageGroupId</code> identifies the tenant a message belongs to. A tenant can be any entity that shares a queue with others, such as your customer, a client application, or a request type. When one tenant sends a disproportionately large volume of messages or has messages that require longer processing time, fair queues ensure other tenants' messages maintain low dwell time. This preserves quality of service for all tenants while maintaining the scalability and throughput of standard queues. We recommend that you include a <code>MessageGroupId</code> in all messages when using fair queues.</p></li>
    /// </ul>
    /// <p>The length of <code>MessageGroupId</code> is 128 characters. Valid values: alphanumeric characters and punctuation <code>(!"#$%&amp;'()*+,-./:;&lt;=&gt;?@\[\\]^_`{|}~)</code>.</p>
    /// <p>For best practices of using <code>MessageGroupId</code>, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/using-messagegroupid-property.html">Using the MessageGroupId Property</a> in the <i>Amazon SQS Developer Guide</i>.</p>
    pub fn set_message_group_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_message_group_id(input);
        self
    }
    /// <p><code>MessageGroupId</code> is an attribute used in Amazon SQS FIFO (First-In-First-Out) and standard queues. In FIFO queues, <code>MessageGroupId</code> organizes messages into distinct groups. Messages within the same message group are always processed one at a time, in strict order, ensuring that no two messages from the same group are processed simultaneously. In standard queues, using <code>MessageGroupId</code> enables fair queues. It is used to identify the tenant a message belongs to, helping maintain consistent message dwell time across all tenants during noisy neighbor events. Unlike FIFO queues, messages with the same <code>MessageGroupId</code> can be processed in parallel, maintaining the high throughput of standard queues.</p>
    /// <ul>
    /// <li>
    /// <p><b>FIFO queues:</b> <code>MessageGroupId</code> acts as the tag that specifies that a message belongs to a specific message group. Messages that belong to the same message group are processed in a FIFO manner (however, messages in different message groups might be processed out of order). To interleave multiple ordered streams within a single queue, use <code>MessageGroupId</code> values (for example, session data for multiple users). In this scenario, multiple consumers can process the queue, but the session data of each user is processed in a FIFO fashion.</p>
    /// <p>If you do not provide a <code>MessageGroupId</code> when sending a message to a FIFO queue, the action fails.</p>
    /// <p><code>ReceiveMessage</code> might return messages with multiple <code>MessageGroupId</code> values. For each <code>MessageGroupId</code>, the messages are sorted by time sent.</p></li>
    /// <li>
    /// <p><b>Standard queues:</b>Use <code>MessageGroupId</code> in standard queues to enable fair queues. The <code>MessageGroupId</code> identifies the tenant a message belongs to. A tenant can be any entity that shares a queue with others, such as your customer, a client application, or a request type. When one tenant sends a disproportionately large volume of messages or has messages that require longer processing time, fair queues ensure other tenants' messages maintain low dwell time. This preserves quality of service for all tenants while maintaining the scalability and throughput of standard queues. We recommend that you include a <code>MessageGroupId</code> in all messages when using fair queues.</p></li>
    /// </ul>
    /// <p>The length of <code>MessageGroupId</code> is 128 characters. Valid values: alphanumeric characters and punctuation <code>(!"#$%&amp;'()*+,-./:;&lt;=&gt;?@\[\\]^_`{|}~)</code>.</p>
    /// <p>For best practices of using <code>MessageGroupId</code>, see <a href="https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/using-messagegroupid-property.html">Using the MessageGroupId Property</a> in the <i>Amazon SQS Developer Guide</i>.</p>
    pub fn get_message_group_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_message_group_id()
    }
}
