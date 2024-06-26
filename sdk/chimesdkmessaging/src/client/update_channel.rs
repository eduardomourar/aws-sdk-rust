// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateChannel`](crate::operation::update_channel::builders::UpdateChannelFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`channel_arn(impl Into<String>)`](crate::operation::update_channel::builders::UpdateChannelFluentBuilder::channel_arn) / [`set_channel_arn(Option<String>)`](crate::operation::update_channel::builders::UpdateChannelFluentBuilder::set_channel_arn):<br>required: **true**<br><p>The ARN of the channel.</p><br>
    ///   - [`name(impl Into<String>)`](crate::operation::update_channel::builders::UpdateChannelFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_channel::builders::UpdateChannelFluentBuilder::set_name):<br>required: **false**<br><p>The name of the channel.</p><br>
    ///   - [`mode(ChannelMode)`](crate::operation::update_channel::builders::UpdateChannelFluentBuilder::mode) / [`set_mode(Option<ChannelMode>)`](crate::operation::update_channel::builders::UpdateChannelFluentBuilder::set_mode):<br>required: **false**<br><p>The mode of the update request.</p><br>
    ///   - [`metadata(impl Into<String>)`](crate::operation::update_channel::builders::UpdateChannelFluentBuilder::metadata) / [`set_metadata(Option<String>)`](crate::operation::update_channel::builders::UpdateChannelFluentBuilder::set_metadata):<br>required: **false**<br><p>The metadata for the update request.</p><br>
    ///   - [`chime_bearer(impl Into<String>)`](crate::operation::update_channel::builders::UpdateChannelFluentBuilder::chime_bearer) / [`set_chime_bearer(Option<String>)`](crate::operation::update_channel::builders::UpdateChannelFluentBuilder::set_chime_bearer):<br>required: **true**<br><p>The ARN of the <code>AppInstanceUser</code> or <code>AppInstanceBot</code> that makes the API call.</p><br>
    /// - On success, responds with [`UpdateChannelOutput`](crate::operation::update_channel::UpdateChannelOutput) with field(s):
    ///   - [`channel_arn(Option<String>)`](crate::operation::update_channel::UpdateChannelOutput::channel_arn): <p>The ARN of the channel.</p>
    /// - On failure, responds with [`SdkError<UpdateChannelError>`](crate::operation::update_channel::UpdateChannelError)
    pub fn update_channel(&self) -> crate::operation::update_channel::builders::UpdateChannelFluentBuilder {
        crate::operation::update_channel::builders::UpdateChannelFluentBuilder::new(self.handle.clone())
    }
}
