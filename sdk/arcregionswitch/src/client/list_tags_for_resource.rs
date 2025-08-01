// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListTagsForResource`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`arn(impl Into<String>)`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::arn) / [`set_arn(Option<String>)`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::set_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the resource.</p><br>
    /// - On success, responds with [`ListTagsForResourceOutput`](crate::operation::list_tags_for_resource::ListTagsForResourceOutput) with field(s):
    ///   - [`resource_tags(Option<HashMap::<String, String>>)`](crate::operation::list_tags_for_resource::ListTagsForResourceOutput::resource_tags): <p>The tags for a resource.</p>
    /// - On failure, responds with [`SdkError<ListTagsForResourceError>`](crate::operation::list_tags_for_resource::ListTagsForResourceError)
    pub fn list_tags_for_resource(&self) -> crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder {
        crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::new(self.handle.clone())
    }
}
