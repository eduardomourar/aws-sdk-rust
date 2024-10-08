// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`RebootInstances`](crate::operation::reboot_instances::builders::RebootInstancesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_ids(impl Into<String>)`](crate::operation::reboot_instances::builders::RebootInstancesFluentBuilder::instance_ids) / [`set_instance_ids(Option<Vec::<String>>)`](crate::operation::reboot_instances::builders::RebootInstancesFluentBuilder::set_instance_ids):<br>required: **true**<br><p>The instance IDs.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::reboot_instances::builders::RebootInstancesFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::reboot_instances::builders::RebootInstancesFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the operation, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`RebootInstancesOutput`](crate::operation::reboot_instances::RebootInstancesOutput)
    /// - On failure, responds with [`SdkError<RebootInstancesError>`](crate::operation::reboot_instances::RebootInstancesError)
    pub fn reboot_instances(&self) -> crate::operation::reboot_instances::builders::RebootInstancesFluentBuilder {
        crate::operation::reboot_instances::builders::RebootInstancesFluentBuilder::new(self.handle.clone())
    }
}
