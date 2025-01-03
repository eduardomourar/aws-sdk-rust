// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteMessageTemplate`](crate::operation::delete_message_template::builders::DeleteMessageTemplateFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`knowledge_base_id(impl Into<String>)`](crate::operation::delete_message_template::builders::DeleteMessageTemplateFluentBuilder::knowledge_base_id) / [`set_knowledge_base_id(Option<String>)`](crate::operation::delete_message_template::builders::DeleteMessageTemplateFluentBuilder::set_knowledge_base_id):<br>required: **true**<br><p>The identifier of the knowledge base. Can be either the ID or the ARN. URLs cannot contain the ARN.</p><br>
    ///   - [`message_template_id(impl Into<String>)`](crate::operation::delete_message_template::builders::DeleteMessageTemplateFluentBuilder::message_template_id) / [`set_message_template_id(Option<String>)`](crate::operation::delete_message_template::builders::DeleteMessageTemplateFluentBuilder::set_message_template_id):<br>required: **true**<br><p>The identifier of the message template. Can be either the ID or the ARN.</p><br>
    /// - On success, responds with [`DeleteMessageTemplateOutput`](crate::operation::delete_message_template::DeleteMessageTemplateOutput)
    /// - On failure, responds with [`SdkError<DeleteMessageTemplateError>`](crate::operation::delete_message_template::DeleteMessageTemplateError)
    pub fn delete_message_template(&self) -> crate::operation::delete_message_template::builders::DeleteMessageTemplateFluentBuilder {
        crate::operation::delete_message_template::builders::DeleteMessageTemplateFluentBuilder::new(self.handle.clone())
    }
}
