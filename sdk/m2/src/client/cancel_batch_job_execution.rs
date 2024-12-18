// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CancelBatchJobExecution`](crate::operation::cancel_batch_job_execution::builders::CancelBatchJobExecutionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`application_id(impl Into<String>)`](crate::operation::cancel_batch_job_execution::builders::CancelBatchJobExecutionFluentBuilder::application_id) / [`set_application_id(Option<String>)`](crate::operation::cancel_batch_job_execution::builders::CancelBatchJobExecutionFluentBuilder::set_application_id):<br>required: **true**<br><p>The unique identifier of the application.</p><br>
    ///   - [`execution_id(impl Into<String>)`](crate::operation::cancel_batch_job_execution::builders::CancelBatchJobExecutionFluentBuilder::execution_id) / [`set_execution_id(Option<String>)`](crate::operation::cancel_batch_job_execution::builders::CancelBatchJobExecutionFluentBuilder::set_execution_id):<br>required: **true**<br><p>The unique identifier of the batch job execution.</p><br>
    ///   - [`auth_secrets_manager_arn(impl Into<String>)`](crate::operation::cancel_batch_job_execution::builders::CancelBatchJobExecutionFluentBuilder::auth_secrets_manager_arn) / [`set_auth_secrets_manager_arn(Option<String>)`](crate::operation::cancel_batch_job_execution::builders::CancelBatchJobExecutionFluentBuilder::set_auth_secrets_manager_arn):<br>required: **false**<br><p>The Amazon Web Services Secrets Manager containing user's credentials for authentication and authorization for Cancel Batch Job Execution operation.</p><br>
    /// - On success, responds with [`CancelBatchJobExecutionOutput`](crate::operation::cancel_batch_job_execution::CancelBatchJobExecutionOutput)
    /// - On failure, responds with [`SdkError<CancelBatchJobExecutionError>`](crate::operation::cancel_batch_job_execution::CancelBatchJobExecutionError)
    pub fn cancel_batch_job_execution(&self) -> crate::operation::cancel_batch_job_execution::builders::CancelBatchJobExecutionFluentBuilder {
        crate::operation::cancel_batch_job_execution::builders::CancelBatchJobExecutionFluentBuilder::new(self.handle.clone())
    }
}
