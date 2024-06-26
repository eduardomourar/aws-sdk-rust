// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetExecution`](crate::operation::get_execution::builders::GetExecutionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`export_arn(impl Into<String>)`](crate::operation::get_execution::builders::GetExecutionFluentBuilder::export_arn) / [`set_export_arn(Option<String>)`](crate::operation::get_execution::builders::GetExecutionFluentBuilder::set_export_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the Export object that generated this specific execution.</p><br>
    ///   - [`execution_id(impl Into<String>)`](crate::operation::get_execution::builders::GetExecutionFluentBuilder::execution_id) / [`set_execution_id(Option<String>)`](crate::operation::get_execution::builders::GetExecutionFluentBuilder::set_execution_id):<br>required: **true**<br><p>The ID for this specific execution.</p><br>
    /// - On success, responds with [`GetExecutionOutput`](crate::operation::get_execution::GetExecutionOutput) with field(s):
    ///   - [`execution_id(Option<String>)`](crate::operation::get_execution::GetExecutionOutput::execution_id): <p>The ID for this specific execution.</p>
    ///   - [`export(Option<Export>)`](crate::operation::get_execution::GetExecutionOutput::export): <p>The export data for this specific execution. This export data is a snapshot from when the execution was generated. The data could be different from the current export data if the export was updated since the execution was generated.</p>
    ///   - [`execution_status(Option<ExecutionStatus>)`](crate::operation::get_execution::GetExecutionOutput::execution_status): <p>The status of this specific execution.</p>
    /// - On failure, responds with [`SdkError<GetExecutionError>`](crate::operation::get_execution::GetExecutionError)
    pub fn get_execution(&self) -> crate::operation::get_execution::builders::GetExecutionFluentBuilder {
        crate::operation::get_execution::builders::GetExecutionFluentBuilder::new(self.handle.clone())
    }
}
