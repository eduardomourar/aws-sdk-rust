// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateStream`](crate::operation::create_stream::builders::CreateStreamFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`stream_name(impl Into<String>)`](crate::operation::create_stream::builders::CreateStreamFluentBuilder::stream_name) / [`set_stream_name(Option<String>)`](crate::operation::create_stream::builders::CreateStreamFluentBuilder::set_stream_name):<br>required: **true**<br><p>A name to identify the stream. The stream name is scoped to the Amazon Web Services account used by the application that creates the stream. It is also scoped by Amazon Web Services Region. That is, two streams in two different Amazon Web Services accounts can have the same name. Two streams in the same Amazon Web Services account but in two different Regions can also have the same name.</p><br>
    ///   - [`shard_count(i32)`](crate::operation::create_stream::builders::CreateStreamFluentBuilder::shard_count) / [`set_shard_count(Option<i32>)`](crate::operation::create_stream::builders::CreateStreamFluentBuilder::set_shard_count):<br>required: **false**<br><p>The number of shards that the stream will use. The throughput of the stream is a function of the number of shards; more shards are required for greater provisioned throughput.</p><br>
    ///   - [`stream_mode_details(StreamModeDetails)`](crate::operation::create_stream::builders::CreateStreamFluentBuilder::stream_mode_details) / [`set_stream_mode_details(Option<StreamModeDetails>)`](crate::operation::create_stream::builders::CreateStreamFluentBuilder::set_stream_mode_details):<br>required: **false**<br><p>Indicates the capacity mode of the data stream. Currently, in Kinesis Data Streams, you can choose between an <b>on-demand</b> capacity mode and a <b>provisioned</b> capacity mode for your data streams.</p><br>
    ///   - [`tags(impl Into<String>, impl Into<String>)`](crate::operation::create_stream::builders::CreateStreamFluentBuilder::tags) / [`set_tags(Option<HashMap::<String, String>>)`](crate::operation::create_stream::builders::CreateStreamFluentBuilder::set_tags):<br>required: **false**<br><p>A set of up to 50 key-value pairs to use to create the tags. A tag consists of a required key and an optional value.</p><br>
    /// - On success, responds with [`CreateStreamOutput`](crate::operation::create_stream::CreateStreamOutput)
    /// - On failure, responds with [`SdkError<CreateStreamError>`](crate::operation::create_stream::CreateStreamError)
    pub fn create_stream(&self) -> crate::operation::create_stream::builders::CreateStreamFluentBuilder {
        crate::operation::create_stream::builders::CreateStreamFluentBuilder::new(self.handle.clone())
    }
}
