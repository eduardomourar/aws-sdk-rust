// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListCodeInterpreters`](crate::operation::list_code_interpreters::builders::ListCodeInterpretersFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_code_interpreters::builders::ListCodeInterpretersFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::operation::list_code_interpreters::builders::ListCodeInterpretersFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_code_interpreters::builders::ListCodeInterpretersFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return in the response.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_code_interpreters::builders::ListCodeInterpretersFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_code_interpreters::builders::ListCodeInterpretersFluentBuilder::set_next_token):<br>required: **false**<br><p>A token to retrieve the next page of results.</p><br>
    ///   - [`r#type(ResourceType)`](crate::operation::list_code_interpreters::builders::ListCodeInterpretersFluentBuilder::type) / [`set_type(Option<ResourceType>)`](crate::operation::list_code_interpreters::builders::ListCodeInterpretersFluentBuilder::set_type):<br>required: **false**<br><p>The type of code interpreters to list.</p><br>
    /// - On success, responds with [`ListCodeInterpretersOutput`](crate::operation::list_code_interpreters::ListCodeInterpretersOutput) with field(s):
    ///   - [`code_interpreter_summaries(Vec::<CodeInterpreterSummary>)`](crate::operation::list_code_interpreters::ListCodeInterpretersOutput::code_interpreter_summaries): <p>The list of code interpreter summaries.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_code_interpreters::ListCodeInterpretersOutput::next_token): <p>A token to retrieve the next page of results.</p>
    /// - On failure, responds with [`SdkError<ListCodeInterpretersError>`](crate::operation::list_code_interpreters::ListCodeInterpretersError)
    pub fn list_code_interpreters(&self) -> crate::operation::list_code_interpreters::builders::ListCodeInterpretersFluentBuilder {
        crate::operation::list_code_interpreters::builders::ListCodeInterpretersFluentBuilder::new(self.handle.clone())
    }
}
