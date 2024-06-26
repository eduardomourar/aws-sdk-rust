// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteResourcePermission`](crate::operation::delete_resource_permission::builders::DeleteResourcePermissionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`action_type(PermissionActionType)`](crate::operation::delete_resource_permission::builders::DeleteResourcePermissionFluentBuilder::action_type) / [`set_action_type(Option<PermissionActionType>)`](crate::operation::delete_resource_permission::builders::DeleteResourcePermissionFluentBuilder::set_action_type):<br>required: **false**<br><p>Delete or restore the permissions on the target database.</p><br>
    ///   - [`source_resource_arn(impl Into<String>)`](crate::operation::delete_resource_permission::builders::DeleteResourcePermissionFluentBuilder::source_resource_arn) / [`set_source_resource_arn(Option<String>)`](crate::operation::delete_resource_permission::builders::DeleteResourcePermissionFluentBuilder::set_source_resource_arn):<br>required: **false**<br><p>The Amazon Resource Name (ARN) of the source resource.</p><br>
    ///   - [`resource_arn(impl Into<String>)`](crate::operation::delete_resource_permission::builders::DeleteResourcePermissionFluentBuilder::resource_arn) / [`set_resource_arn(Option<String>)`](crate::operation::delete_resource_permission::builders::DeleteResourcePermissionFluentBuilder::set_resource_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the resource.</p><br>
    /// - On success, responds with [`DeleteResourcePermissionOutput`](crate::operation::delete_resource_permission::DeleteResourcePermissionOutput) with field(s):
    ///   - [`policy(Option<String>)`](crate::operation::delete_resource_permission::DeleteResourcePermissionOutput::policy): <p>The policy that removes permissions on the target database.</p>
    /// - On failure, responds with [`SdkError<DeleteResourcePermissionError>`](crate::operation::delete_resource_permission::DeleteResourcePermissionError)
    pub fn delete_resource_permission(&self) -> crate::operation::delete_resource_permission::builders::DeleteResourcePermissionFluentBuilder {
        crate::operation::delete_resource_permission::builders::DeleteResourcePermissionFluentBuilder::new(self.handle.clone())
    }
}
