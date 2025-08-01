// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteSessionLogger`](crate::operation::delete_session_logger::builders::DeleteSessionLoggerFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`session_logger_arn(impl Into<String>)`](crate::operation::delete_session_logger::builders::DeleteSessionLoggerFluentBuilder::session_logger_arn) / [`set_session_logger_arn(Option<String>)`](crate::operation::delete_session_logger::builders::DeleteSessionLoggerFluentBuilder::set_session_logger_arn):<br>required: **true**<br><p>The ARN of the session logger.</p><br>
    /// - On success, responds with [`DeleteSessionLoggerOutput`](crate::operation::delete_session_logger::DeleteSessionLoggerOutput)
    /// - On failure, responds with [`SdkError<DeleteSessionLoggerError>`](crate::operation::delete_session_logger::DeleteSessionLoggerError)
    pub fn delete_session_logger(&self) -> crate::operation::delete_session_logger::builders::DeleteSessionLoggerFluentBuilder {
        crate::operation::delete_session_logger::builders::DeleteSessionLoggerFluentBuilder::new(self.handle.clone())
    }
}
