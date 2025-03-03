// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateMemberDetectors`](crate::operation::update_member_detectors::builders::UpdateMemberDetectorsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`detector_id(impl Into<String>)`](crate::operation::update_member_detectors::builders::UpdateMemberDetectorsFluentBuilder::detector_id) / [`set_detector_id(Option<String>)`](crate::operation::update_member_detectors::builders::UpdateMemberDetectorsFluentBuilder::set_detector_id):<br>required: **true**<br><p>The detector ID of the administrator account.</p> <p>To find the <code>detectorId</code> in the current Region, see the Settings page in the GuardDuty console, or run the <a href="https://docs.aws.amazon.com/guardduty/latest/APIReference/API_ListDetectors.html">ListDetectors</a> API.</p><br>
    ///   - [`account_ids(impl Into<String>)`](crate::operation::update_member_detectors::builders::UpdateMemberDetectorsFluentBuilder::account_ids) / [`set_account_ids(Option<Vec::<String>>)`](crate::operation::update_member_detectors::builders::UpdateMemberDetectorsFluentBuilder::set_account_ids):<br>required: **true**<br><p>A list of member account IDs to be updated.</p><br>
    ///   - [`data_sources(DataSourceConfigurations)`](crate::operation::update_member_detectors::builders::UpdateMemberDetectorsFluentBuilder::data_sources) / [`set_data_sources(Option<DataSourceConfigurations>)`](crate::operation::update_member_detectors::builders::UpdateMemberDetectorsFluentBuilder::set_data_sources):<br>required: **false**<br><p>Describes which data sources will be updated.</p><br>
    ///   - [`features(MemberFeaturesConfiguration)`](crate::operation::update_member_detectors::builders::UpdateMemberDetectorsFluentBuilder::features) / [`set_features(Option<Vec::<MemberFeaturesConfiguration>>)`](crate::operation::update_member_detectors::builders::UpdateMemberDetectorsFluentBuilder::set_features):<br>required: **false**<br><p>A list of features that will be updated for the specified member accounts.</p><br>
    /// - On success, responds with [`UpdateMemberDetectorsOutput`](crate::operation::update_member_detectors::UpdateMemberDetectorsOutput) with field(s):
    ///   - [`unprocessed_accounts(Option<Vec::<UnprocessedAccount>>)`](crate::operation::update_member_detectors::UpdateMemberDetectorsOutput::unprocessed_accounts): <p>A list of member account IDs that were unable to be processed along with an explanation for why they were not processed.</p>
    /// - On failure, responds with [`SdkError<UpdateMemberDetectorsError>`](crate::operation::update_member_detectors::UpdateMemberDetectorsError)
    pub fn update_member_detectors(&self) -> crate::operation::update_member_detectors::builders::UpdateMemberDetectorsFluentBuilder {
        crate::operation::update_member_detectors::builders::UpdateMemberDetectorsFluentBuilder::new(self.handle.clone())
    }
}
