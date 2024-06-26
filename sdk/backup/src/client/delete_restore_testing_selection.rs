// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteRestoreTestingSelection`](crate::operation::delete_restore_testing_selection::builders::DeleteRestoreTestingSelectionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`restore_testing_plan_name(impl Into<String>)`](crate::operation::delete_restore_testing_selection::builders::DeleteRestoreTestingSelectionFluentBuilder::restore_testing_plan_name) / [`set_restore_testing_plan_name(Option<String>)`](crate::operation::delete_restore_testing_selection::builders::DeleteRestoreTestingSelectionFluentBuilder::set_restore_testing_plan_name):<br>required: **true**<br><p>Required unique name of the restore testing plan that contains the restore testing selection you wish to delete.</p><br>
    ///   - [`restore_testing_selection_name(impl Into<String>)`](crate::operation::delete_restore_testing_selection::builders::DeleteRestoreTestingSelectionFluentBuilder::restore_testing_selection_name) / [`set_restore_testing_selection_name(Option<String>)`](crate::operation::delete_restore_testing_selection::builders::DeleteRestoreTestingSelectionFluentBuilder::set_restore_testing_selection_name):<br>required: **true**<br><p>Required unique name of the restore testing selection you wish to delete.</p><br>
    /// - On success, responds with [`DeleteRestoreTestingSelectionOutput`](crate::operation::delete_restore_testing_selection::DeleteRestoreTestingSelectionOutput)
    /// - On failure, responds with [`SdkError<DeleteRestoreTestingSelectionError>`](crate::operation::delete_restore_testing_selection::DeleteRestoreTestingSelectionError)
    pub fn delete_restore_testing_selection(
        &self,
    ) -> crate::operation::delete_restore_testing_selection::builders::DeleteRestoreTestingSelectionFluentBuilder {
        crate::operation::delete_restore_testing_selection::builders::DeleteRestoreTestingSelectionFluentBuilder::new(self.handle.clone())
    }
}
