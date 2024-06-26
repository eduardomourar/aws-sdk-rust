// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ConvertRecoveryPointToSnapshot`](crate::operation::convert_recovery_point_to_snapshot::builders::ConvertRecoveryPointToSnapshotFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`recovery_point_id(impl Into<String>)`](crate::operation::convert_recovery_point_to_snapshot::builders::ConvertRecoveryPointToSnapshotFluentBuilder::recovery_point_id) / [`set_recovery_point_id(Option<String>)`](crate::operation::convert_recovery_point_to_snapshot::builders::ConvertRecoveryPointToSnapshotFluentBuilder::set_recovery_point_id):<br>required: **true**<br><p>The unique identifier of the recovery point.</p><br>
    ///   - [`snapshot_name(impl Into<String>)`](crate::operation::convert_recovery_point_to_snapshot::builders::ConvertRecoveryPointToSnapshotFluentBuilder::snapshot_name) / [`set_snapshot_name(Option<String>)`](crate::operation::convert_recovery_point_to_snapshot::builders::ConvertRecoveryPointToSnapshotFluentBuilder::set_snapshot_name):<br>required: **true**<br><p>The name of the snapshot.</p><br>
    ///   - [`retention_period(i32)`](crate::operation::convert_recovery_point_to_snapshot::builders::ConvertRecoveryPointToSnapshotFluentBuilder::retention_period) / [`set_retention_period(Option<i32>)`](crate::operation::convert_recovery_point_to_snapshot::builders::ConvertRecoveryPointToSnapshotFluentBuilder::set_retention_period):<br>required: **false**<br><p>How long to retain the snapshot.</p><br>
    ///   - [`tags(Tag)`](crate::operation::convert_recovery_point_to_snapshot::builders::ConvertRecoveryPointToSnapshotFluentBuilder::tags) / [`set_tags(Option<Vec::<Tag>>)`](crate::operation::convert_recovery_point_to_snapshot::builders::ConvertRecoveryPointToSnapshotFluentBuilder::set_tags):<br>required: **false**<br><p>An array of <a href="https://docs.aws.amazon.com/redshift-serverless/latest/APIReference/API_Tag.html">Tag objects</a> to associate with the created snapshot.</p><br>
    /// - On success, responds with [`ConvertRecoveryPointToSnapshotOutput`](crate::operation::convert_recovery_point_to_snapshot::ConvertRecoveryPointToSnapshotOutput) with field(s):
    ///   - [`snapshot(Option<Snapshot>)`](crate::operation::convert_recovery_point_to_snapshot::ConvertRecoveryPointToSnapshotOutput::snapshot): <p>The snapshot converted from the recovery point.</p>
    /// - On failure, responds with [`SdkError<ConvertRecoveryPointToSnapshotError>`](crate::operation::convert_recovery_point_to_snapshot::ConvertRecoveryPointToSnapshotError)
    pub fn convert_recovery_point_to_snapshot(
        &self,
    ) -> crate::operation::convert_recovery_point_to_snapshot::builders::ConvertRecoveryPointToSnapshotFluentBuilder {
        crate::operation::convert_recovery_point_to_snapshot::builders::ConvertRecoveryPointToSnapshotFluentBuilder::new(self.handle.clone())
    }
}
