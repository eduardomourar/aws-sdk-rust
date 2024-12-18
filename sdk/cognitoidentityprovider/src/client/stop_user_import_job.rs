// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StopUserImportJob`](crate::operation::stop_user_import_job::builders::StopUserImportJobFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`user_pool_id(impl Into<String>)`](crate::operation::stop_user_import_job::builders::StopUserImportJobFluentBuilder::user_pool_id) / [`set_user_pool_id(Option<String>)`](crate::operation::stop_user_import_job::builders::StopUserImportJobFluentBuilder::set_user_pool_id):<br>required: **true**<br><p>The ID of the user pool that the users are being imported into.</p><br>
    ///   - [`job_id(impl Into<String>)`](crate::operation::stop_user_import_job::builders::StopUserImportJobFluentBuilder::job_id) / [`set_job_id(Option<String>)`](crate::operation::stop_user_import_job::builders::StopUserImportJobFluentBuilder::set_job_id):<br>required: **true**<br><p>The job ID for the user import job.</p><br>
    /// - On success, responds with [`StopUserImportJobOutput`](crate::operation::stop_user_import_job::StopUserImportJobOutput) with field(s):
    ///   - [`user_import_job(Option<UserImportJobType>)`](crate::operation::stop_user_import_job::StopUserImportJobOutput::user_import_job): <p>The job object that represents the user import job.</p>
    /// - On failure, responds with [`SdkError<StopUserImportJobError>`](crate::operation::stop_user_import_job::StopUserImportJobError)
    pub fn stop_user_import_job(&self) -> crate::operation::stop_user_import_job::builders::StopUserImportJobFluentBuilder {
        crate::operation::stop_user_import_job::builders::StopUserImportJobFluentBuilder::new(self.handle.clone())
    }
}
