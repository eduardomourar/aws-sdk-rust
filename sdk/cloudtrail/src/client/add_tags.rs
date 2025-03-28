// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AddTags`](crate::operation::add_tags::builders::AddTagsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_id(impl Into<String>)`](crate::operation::add_tags::builders::AddTagsFluentBuilder::resource_id) / [`set_resource_id(Option<String>)`](crate::operation::add_tags::builders::AddTagsFluentBuilder::set_resource_id):<br>required: **true**<br><p>Specifies the ARN of the trail, event data store, dashboard, or channel to which one or more tags will be added.</p> <p>The format of a trail ARN is: <code>arn:aws:cloudtrail:us-east-2:123456789012:trail/MyTrail</code></p> <p>The format of an event data store ARN is: <code>arn:aws:cloudtrail:us-east-2:123456789012:eventdatastore/EXAMPLE-f852-4e8f-8bd1-bcf6cEXAMPLE</code></p> <p>The format of a dashboard ARN is: <code>arn:aws:cloudtrail:us-east-1:123456789012:dashboard/exampleDash</code></p> <p>The format of a channel ARN is: <code>arn:aws:cloudtrail:us-east-2:123456789012:channel/01234567890</code></p><br>
    ///   - [`tags_list(Tag)`](crate::operation::add_tags::builders::AddTagsFluentBuilder::tags_list) / [`set_tags_list(Option<Vec::<Tag>>)`](crate::operation::add_tags::builders::AddTagsFluentBuilder::set_tags_list):<br>required: **true**<br><p>Contains a list of tags, up to a limit of 50</p><br>
    /// - On success, responds with [`AddTagsOutput`](crate::operation::add_tags::AddTagsOutput)
    /// - On failure, responds with [`SdkError<AddTagsError>`](crate::operation::add_tags::AddTagsError)
    pub fn add_tags(&self) -> crate::operation::add_tags::builders::AddTagsFluentBuilder {
        crate::operation::add_tags::builders::AddTagsFluentBuilder::new(self.handle.clone())
    }
}
