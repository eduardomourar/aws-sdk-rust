// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateContext`](crate::operation::create_context::builders::CreateContextFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`context_name(impl Into<String>)`](crate::operation::create_context::builders::CreateContextFluentBuilder::context_name) / [`set_context_name(Option<String>)`](crate::operation::create_context::builders::CreateContextFluentBuilder::set_context_name):<br>required: **true**<br><p>The name of the context. Must be unique to your account in an Amazon Web Services Region.</p><br>
    ///   - [`source(ContextSource)`](crate::operation::create_context::builders::CreateContextFluentBuilder::source) / [`set_source(Option<ContextSource>)`](crate::operation::create_context::builders::CreateContextFluentBuilder::set_source):<br>required: **true**<br><p>The source type, ID, and URI.</p><br>
    ///   - [`context_type(impl Into<String>)`](crate::operation::create_context::builders::CreateContextFluentBuilder::context_type) / [`set_context_type(Option<String>)`](crate::operation::create_context::builders::CreateContextFluentBuilder::set_context_type):<br>required: **true**<br><p>The context type.</p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::create_context::builders::CreateContextFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_context::builders::CreateContextFluentBuilder::set_description):<br>required: **false**<br><p>The description of the context.</p><br>
    ///   - [`properties(impl Into<String>, impl Into<String>)`](crate::operation::create_context::builders::CreateContextFluentBuilder::properties) / [`set_properties(Option<HashMap::<String, String>>)`](crate::operation::create_context::builders::CreateContextFluentBuilder::set_properties):<br>required: **false**<br><p>A list of properties to add to the context.</p><br>
    ///   - [`tags(Tag)`](crate::operation::create_context::builders::CreateContextFluentBuilder::tags) / [`set_tags(Option<Vec::<Tag>>)`](crate::operation::create_context::builders::CreateContextFluentBuilder::set_tags):<br>required: **false**<br><p>A list of tags to apply to the context.</p><br>
    /// - On success, responds with [`CreateContextOutput`](crate::operation::create_context::CreateContextOutput) with field(s):
    ///   - [`context_arn(Option<String>)`](crate::operation::create_context::CreateContextOutput::context_arn): <p>The Amazon Resource Name (ARN) of the context.</p>
    /// - On failure, responds with [`SdkError<CreateContextError>`](crate::operation::create_context::CreateContextError)
    pub fn create_context(&self) -> crate::operation::create_context::builders::CreateContextFluentBuilder {
        crate::operation::create_context::builders::CreateContextFluentBuilder::new(self.handle.clone())
    }
}
