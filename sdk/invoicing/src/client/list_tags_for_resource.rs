// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListTagsForResource`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl Into<String>)`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::resource_arn) / [`set_resource_arn(Option<String>)`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::set_resource_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of tags to list.</p><br>
    /// - On success, responds with [`ListTagsForResourceOutput`](crate::operation::list_tags_for_resource::ListTagsForResourceOutput) with field(s):
    ///   - [`resource_tags(Option<Vec::<ResourceTag>>)`](crate::operation::list_tags_for_resource::ListTagsForResourceOutput::resource_tags): <p>Adds a tag to a resource.</p>
    /// - On failure, responds with [`SdkError<ListTagsForResourceError>`](crate::operation::list_tags_for_resource::ListTagsForResourceError)
    pub fn list_tags_for_resource(&self) -> crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder {
        crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::new(self.handle.clone())
    }
}
