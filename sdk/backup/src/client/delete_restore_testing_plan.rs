// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteRestoreTestingPlan`](crate::operation::delete_restore_testing_plan::builders::DeleteRestoreTestingPlanFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`restore_testing_plan_name(impl Into<String>)`](crate::operation::delete_restore_testing_plan::builders::DeleteRestoreTestingPlanFluentBuilder::restore_testing_plan_name) / [`set_restore_testing_plan_name(Option<String>)`](crate::operation::delete_restore_testing_plan::builders::DeleteRestoreTestingPlanFluentBuilder::set_restore_testing_plan_name):<br>required: **true**<br><p>Required unique name of the restore testing plan you wish to delete.</p><br>
    /// - On success, responds with [`DeleteRestoreTestingPlanOutput`](crate::operation::delete_restore_testing_plan::DeleteRestoreTestingPlanOutput)
    /// - On failure, responds with [`SdkError<DeleteRestoreTestingPlanError>`](crate::operation::delete_restore_testing_plan::DeleteRestoreTestingPlanError)
    pub fn delete_restore_testing_plan(&self) -> crate::operation::delete_restore_testing_plan::builders::DeleteRestoreTestingPlanFluentBuilder {
        crate::operation::delete_restore_testing_plan::builders::DeleteRestoreTestingPlanFluentBuilder::new(self.handle.clone())
    }
}
