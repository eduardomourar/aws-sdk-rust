// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteCampaignCommunicationTime`](crate::operation::delete_campaign_communication_time::builders::DeleteCampaignCommunicationTimeFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::operation::delete_campaign_communication_time::builders::DeleteCampaignCommunicationTimeFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::delete_campaign_communication_time::builders::DeleteCampaignCommunicationTimeFluentBuilder::set_id):<br>required: **true**<br>Identifier representing a Campaign<br>
    ///   - [`config(CommunicationTimeConfigType)`](crate::operation::delete_campaign_communication_time::builders::DeleteCampaignCommunicationTimeFluentBuilder::config) / [`set_config(Option<CommunicationTimeConfigType>)`](crate::operation::delete_campaign_communication_time::builders::DeleteCampaignCommunicationTimeFluentBuilder::set_config):<br>required: **true**<br>The type of campaign communication time config<br>
    /// - On success, responds with [`DeleteCampaignCommunicationTimeOutput`](crate::operation::delete_campaign_communication_time::DeleteCampaignCommunicationTimeOutput)
    /// - On failure, responds with [`SdkError<DeleteCampaignCommunicationTimeError>`](crate::operation::delete_campaign_communication_time::DeleteCampaignCommunicationTimeError)
    pub fn delete_campaign_communication_time(
        &self,
    ) -> crate::operation::delete_campaign_communication_time::builders::DeleteCampaignCommunicationTimeFluentBuilder {
        crate::operation::delete_campaign_communication_time::builders::DeleteCampaignCommunicationTimeFluentBuilder::new(self.handle.clone())
    }
}
