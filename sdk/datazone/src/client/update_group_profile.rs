// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateGroupProfile`](crate::operation::update_group_profile::builders::UpdateGroupProfileFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`domain_identifier(impl Into<String>)`](crate::operation::update_group_profile::builders::UpdateGroupProfileFluentBuilder::domain_identifier) / [`set_domain_identifier(Option<String>)`](crate::operation::update_group_profile::builders::UpdateGroupProfileFluentBuilder::set_domain_identifier):<br>required: **true**<br><p>The identifier of the Amazon DataZone domain in which a group profile is updated.</p><br>
    ///   - [`group_identifier(impl Into<String>)`](crate::operation::update_group_profile::builders::UpdateGroupProfileFluentBuilder::group_identifier) / [`set_group_identifier(Option<String>)`](crate::operation::update_group_profile::builders::UpdateGroupProfileFluentBuilder::set_group_identifier):<br>required: **true**<br><p>The identifier of the group profile that is updated.</p><br>
    ///   - [`status(GroupProfileStatus)`](crate::operation::update_group_profile::builders::UpdateGroupProfileFluentBuilder::status) / [`set_status(Option<GroupProfileStatus>)`](crate::operation::update_group_profile::builders::UpdateGroupProfileFluentBuilder::set_status):<br>required: **true**<br><p>The status of the group profile that is updated.</p><br>
    /// - On success, responds with [`UpdateGroupProfileOutput`](crate::operation::update_group_profile::UpdateGroupProfileOutput) with field(s):
    ///   - [`domain_id(Option<String>)`](crate::operation::update_group_profile::UpdateGroupProfileOutput::domain_id): <p>The identifier of the Amazon DataZone domain in which a group profile is updated.</p>
    ///   - [`id(Option<String>)`](crate::operation::update_group_profile::UpdateGroupProfileOutput::id): <p>The identifier of the group profile that is updated.</p>
    ///   - [`status(Option<GroupProfileStatus>)`](crate::operation::update_group_profile::UpdateGroupProfileOutput::status): <p>The status of the group profile that is updated.</p>
    ///   - [`group_name(Option<String>)`](crate::operation::update_group_profile::UpdateGroupProfileOutput::group_name): <p>The name of the group profile that is updated.</p>
    /// - On failure, responds with [`SdkError<UpdateGroupProfileError>`](crate::operation::update_group_profile::UpdateGroupProfileError)
    pub fn update_group_profile(&self) -> crate::operation::update_group_profile::builders::UpdateGroupProfileFluentBuilder {
        crate::operation::update_group_profile::builders::UpdateGroupProfileFluentBuilder::new(self.handle.clone())
    }
}
