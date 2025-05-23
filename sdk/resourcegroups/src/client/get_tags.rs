// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetTags`](crate::operation::get_tags::builders::GetTagsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`arn(impl Into<String>)`](crate::operation::get_tags::builders::GetTagsFluentBuilder::arn) / [`set_arn(Option<String>)`](crate::operation::get_tags::builders::GetTagsFluentBuilder::set_arn):<br>required: **true**<br><p>The Amazon resource name (ARN) of the resource group whose tags you want to retrieve.</p><br>
    /// - On success, responds with [`GetTagsOutput`](crate::operation::get_tags::GetTagsOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::operation::get_tags::GetTagsOutput::arn): <p>TheAmazon resource name (ARN) of the tagged resource group.</p>
    ///   - [`tags(Option<HashMap::<String, String>>)`](crate::operation::get_tags::GetTagsOutput::tags): <p>The tags associated with the specified resource group.</p>
    /// - On failure, responds with [`SdkError<GetTagsError>`](crate::operation::get_tags::GetTagsError)
    pub fn get_tags(&self) -> crate::operation::get_tags::builders::GetTagsFluentBuilder {
        crate::operation::get_tags::builders::GetTagsFluentBuilder::new(self.handle.clone())
    }
}
