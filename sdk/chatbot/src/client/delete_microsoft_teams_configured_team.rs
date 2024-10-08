// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteMicrosoftTeamsConfiguredTeam`](crate::operation::delete_microsoft_teams_configured_team::builders::DeleteMicrosoftTeamsConfiguredTeamFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`team_id(impl Into<String>)`](crate::operation::delete_microsoft_teams_configured_team::builders::DeleteMicrosoftTeamsConfiguredTeamFluentBuilder::team_id) / [`set_team_id(Option<String>)`](crate::operation::delete_microsoft_teams_configured_team::builders::DeleteMicrosoftTeamsConfiguredTeamFluentBuilder::set_team_id):<br>required: **true**<br><p>The ID of the Microsoft Teams team authorized with AWS Chatbot.</p> <p>To get the team ID, you must perform the initial authorization flow with Microsoft Teams in the AWS Chatbot console. Then you can copy and paste the team ID from the console. For more information, see <a href="https://docs.aws.amazon.com/chatbot/latest/adminguide/teams-setup.html#teams-client-setup">Step 1: Configure a Microsoft Teams client</a> in the <i> AWS Chatbot Administrator Guide</i>.</p><br>
    /// - On success, responds with [`DeleteMicrosoftTeamsConfiguredTeamOutput`](crate::operation::delete_microsoft_teams_configured_team::DeleteMicrosoftTeamsConfiguredTeamOutput)
    /// - On failure, responds with [`SdkError<DeleteMicrosoftTeamsConfiguredTeamError>`](crate::operation::delete_microsoft_teams_configured_team::DeleteMicrosoftTeamsConfiguredTeamError)
    pub fn delete_microsoft_teams_configured_team(
        &self,
    ) -> crate::operation::delete_microsoft_teams_configured_team::builders::DeleteMicrosoftTeamsConfiguredTeamFluentBuilder {
        crate::operation::delete_microsoft_teams_configured_team::builders::DeleteMicrosoftTeamsConfiguredTeamFluentBuilder::new(self.handle.clone())
    }
}
