// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DisassociateMemberFromJob`](crate::operation::disassociate_member_from_job::builders::DisassociateMemberFromJobFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`farm_id(impl Into<String>)`](crate::operation::disassociate_member_from_job::builders::DisassociateMemberFromJobFluentBuilder::farm_id) / [`set_farm_id(Option<String>)`](crate::operation::disassociate_member_from_job::builders::DisassociateMemberFromJobFluentBuilder::set_farm_id):<br>required: **true**<br><p>The farm ID for the job to disassociate from the member.</p><br>
    ///   - [`queue_id(impl Into<String>)`](crate::operation::disassociate_member_from_job::builders::DisassociateMemberFromJobFluentBuilder::queue_id) / [`set_queue_id(Option<String>)`](crate::operation::disassociate_member_from_job::builders::DisassociateMemberFromJobFluentBuilder::set_queue_id):<br>required: **true**<br><p>The queue ID connected to a job for which you're disassociating a member.</p><br>
    ///   - [`job_id(impl Into<String>)`](crate::operation::disassociate_member_from_job::builders::DisassociateMemberFromJobFluentBuilder::job_id) / [`set_job_id(Option<String>)`](crate::operation::disassociate_member_from_job::builders::DisassociateMemberFromJobFluentBuilder::set_job_id):<br>required: **true**<br><p>The job ID to disassociate from a member in a job.</p><br>
    ///   - [`principal_id(impl Into<String>)`](crate::operation::disassociate_member_from_job::builders::DisassociateMemberFromJobFluentBuilder::principal_id) / [`set_principal_id(Option<String>)`](crate::operation::disassociate_member_from_job::builders::DisassociateMemberFromJobFluentBuilder::set_principal_id):<br>required: **true**<br><p>A member's principal ID to disassociate from a job.</p><br>
    /// - On success, responds with [`DisassociateMemberFromJobOutput`](crate::operation::disassociate_member_from_job::DisassociateMemberFromJobOutput)
    /// - On failure, responds with [`SdkError<DisassociateMemberFromJobError>`](crate::operation::disassociate_member_from_job::DisassociateMemberFromJobError)
    pub fn disassociate_member_from_job(&self) -> crate::operation::disassociate_member_from_job::builders::DisassociateMemberFromJobFluentBuilder {
        crate::operation::disassociate_member_from_job::builders::DisassociateMemberFromJobFluentBuilder::new(self.handle.clone())
    }
}
