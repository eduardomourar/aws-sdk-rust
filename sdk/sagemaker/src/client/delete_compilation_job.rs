// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteCompilationJob`](crate::operation::delete_compilation_job::builders::DeleteCompilationJobFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`compilation_job_name(impl Into<String>)`](crate::operation::delete_compilation_job::builders::DeleteCompilationJobFluentBuilder::compilation_job_name) / [`set_compilation_job_name(Option<String>)`](crate::operation::delete_compilation_job::builders::DeleteCompilationJobFluentBuilder::set_compilation_job_name):<br>required: **true**<br><p>The name of the compilation job to delete.</p><br>
    /// - On success, responds with [`DeleteCompilationJobOutput`](crate::operation::delete_compilation_job::DeleteCompilationJobOutput)
    /// - On failure, responds with [`SdkError<DeleteCompilationJobError>`](crate::operation::delete_compilation_job::DeleteCompilationJobError)
    pub fn delete_compilation_job(&self) -> crate::operation::delete_compilation_job::builders::DeleteCompilationJobFluentBuilder {
        crate::operation::delete_compilation_job::builders::DeleteCompilationJobFluentBuilder::new(self.handle.clone())
    }
}
