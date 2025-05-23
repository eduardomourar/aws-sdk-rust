// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`RegisterStreamConsumer`](crate::operation::register_stream_consumer::builders::RegisterStreamConsumerFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`stream_arn(impl Into<String>)`](crate::operation::register_stream_consumer::builders::RegisterStreamConsumerFluentBuilder::stream_arn) / [`set_stream_arn(Option<String>)`](crate::operation::register_stream_consumer::builders::RegisterStreamConsumerFluentBuilder::set_stream_arn):<br>required: **true**<br><p>The ARN of the Kinesis data stream that you want to register the consumer with. For more info, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-kinesis-streams">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a>.</p><br>
    ///   - [`consumer_name(impl Into<String>)`](crate::operation::register_stream_consumer::builders::RegisterStreamConsumerFluentBuilder::consumer_name) / [`set_consumer_name(Option<String>)`](crate::operation::register_stream_consumer::builders::RegisterStreamConsumerFluentBuilder::set_consumer_name):<br>required: **true**<br><p>For a given Kinesis data stream, each consumer must have a unique name. However, consumer names don't have to be unique across data streams.</p><br>
    ///   - [`tags(impl Into<String>, impl Into<String>)`](crate::operation::register_stream_consumer::builders::RegisterStreamConsumerFluentBuilder::tags) / [`set_tags(Option<HashMap::<String, String>>)`](crate::operation::register_stream_consumer::builders::RegisterStreamConsumerFluentBuilder::set_tags):<br>required: **false**<br><p>A set of up to 50 key-value pairs. A tag consists of a required key and an optional value.</p><br>
    /// - On success, responds with [`RegisterStreamConsumerOutput`](crate::operation::register_stream_consumer::RegisterStreamConsumerOutput) with field(s):
    ///   - [`consumer(Option<Consumer>)`](crate::operation::register_stream_consumer::RegisterStreamConsumerOutput::consumer): <p>An object that represents the details of the consumer you registered. When you register a consumer, it gets an ARN that is generated by Kinesis Data Streams.</p>
    /// - On failure, responds with [`SdkError<RegisterStreamConsumerError>`](crate::operation::register_stream_consumer::RegisterStreamConsumerError)
    pub fn register_stream_consumer(&self) -> crate::operation::register_stream_consumer::builders::RegisterStreamConsumerFluentBuilder {
        crate::operation::register_stream_consumer::builders::RegisterStreamConsumerFluentBuilder::new(self.handle.clone())
    }
}
