// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StopCodeInterpreterSession`](crate::operation::stop_code_interpreter_session::builders::StopCodeInterpreterSessionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`code_interpreter_identifier(impl Into<String>)`](crate::operation::stop_code_interpreter_session::builders::StopCodeInterpreterSessionFluentBuilder::code_interpreter_identifier) / [`set_code_interpreter_identifier(Option<String>)`](crate::operation::stop_code_interpreter_session::builders::StopCodeInterpreterSessionFluentBuilder::set_code_interpreter_identifier):<br>required: **true**<br><p>The unique identifier of the code interpreter associated with the session.</p><br>
    ///   - [`session_id(impl Into<String>)`](crate::operation::stop_code_interpreter_session::builders::StopCodeInterpreterSessionFluentBuilder::session_id) / [`set_session_id(Option<String>)`](crate::operation::stop_code_interpreter_session::builders::StopCodeInterpreterSessionFluentBuilder::set_session_id):<br>required: **true**<br><p>The unique identifier of the code interpreter session to stop.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::stop_code_interpreter_session::builders::StopCodeInterpreterSessionFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::stop_code_interpreter_session::builders::StopCodeInterpreterSessionFluentBuilder::set_client_token):<br>required: **false**<br><p>A unique, case-sensitive identifier to ensure that the API request completes no more than one time. If this token matches a previous request, Amazon Bedrock ignores the request, but does not return an error.</p><br>
    /// - On success, responds with [`StopCodeInterpreterSessionOutput`](crate::operation::stop_code_interpreter_session::StopCodeInterpreterSessionOutput) with field(s):
    ///   - [`code_interpreter_identifier(String)`](crate::operation::stop_code_interpreter_session::StopCodeInterpreterSessionOutput::code_interpreter_identifier): <p>The identifier of the code interpreter.</p>
    ///   - [`session_id(String)`](crate::operation::stop_code_interpreter_session::StopCodeInterpreterSessionOutput::session_id): <p>The identifier of the code interpreter session.</p>
    ///   - [`last_updated_at(DateTime)`](crate::operation::stop_code_interpreter_session::StopCodeInterpreterSessionOutput::last_updated_at): <p>The timestamp when the code interpreter session was last updated.</p>
    /// - On failure, responds with [`SdkError<StopCodeInterpreterSessionError>`](crate::operation::stop_code_interpreter_session::StopCodeInterpreterSessionError)
    pub fn stop_code_interpreter_session(
        &self,
    ) -> crate::operation::stop_code_interpreter_session::builders::StopCodeInterpreterSessionFluentBuilder {
        crate::operation::stop_code_interpreter_session::builders::StopCodeInterpreterSessionFluentBuilder::new(self.handle.clone())
    }
}
