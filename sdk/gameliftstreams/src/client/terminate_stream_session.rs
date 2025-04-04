// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`TerminateStreamSession`](crate::operation::terminate_stream_session::builders::TerminateStreamSessionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`identifier(impl Into<String>)`](crate::operation::terminate_stream_session::builders::TerminateStreamSessionFluentBuilder::identifier) / [`set_identifier(Option<String>)`](crate::operation::terminate_stream_session::builders::TerminateStreamSessionFluentBuilder::set_identifier):<br>required: **true**<br><p><a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference-arns.html">Amazon Resource Name (ARN)</a> or ID that uniquely identifies the stream group resource. Format example: ARN-<code>arn:aws:gameliftstreams:us-west-2:123456789012:streamgroup/sg-1AB2C3De4</code> or ID-<code>sg-1AB2C3De4</code>.</p> <p>The stream group that runs this stream session.</p><br>
    ///   - [`stream_session_identifier(impl Into<String>)`](crate::operation::terminate_stream_session::builders::TerminateStreamSessionFluentBuilder::stream_session_identifier) / [`set_stream_session_identifier(Option<String>)`](crate::operation::terminate_stream_session::builders::TerminateStreamSessionFluentBuilder::set_stream_session_identifier):<br>required: **true**<br><p><a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference-arns.html">Amazon Resource Name (ARN)</a> that uniquely identifies the stream session resource. Format example: <code>1AB2C3De4</code>.</p><br>
    /// - On success, responds with [`TerminateStreamSessionOutput`](crate::operation::terminate_stream_session::TerminateStreamSessionOutput)
    /// - On failure, responds with [`SdkError<TerminateStreamSessionError>`](crate::operation::terminate_stream_session::TerminateStreamSessionError)
    pub fn terminate_stream_session(&self) -> crate::operation::terminate_stream_session::builders::TerminateStreamSessionFluentBuilder {
        crate::operation::terminate_stream_session::builders::TerminateStreamSessionFluentBuilder::new(self.handle.clone())
    }
}
