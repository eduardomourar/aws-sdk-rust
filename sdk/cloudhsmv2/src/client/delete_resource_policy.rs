// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteResourcePolicy`](crate::operation::delete_resource_policy::builders::DeleteResourcePolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl Into<String>)`](crate::operation::delete_resource_policy::builders::DeleteResourcePolicyFluentBuilder::resource_arn) / [`set_resource_arn(Option<String>)`](crate::operation::delete_resource_policy::builders::DeleteResourcePolicyFluentBuilder::set_resource_arn):<br>required: **false**<br><p>Amazon Resource Name (ARN) of the resource from which the policy will be removed.</p><br>
    /// - On success, responds with [`DeleteResourcePolicyOutput`](crate::operation::delete_resource_policy::DeleteResourcePolicyOutput) with field(s):
    ///   - [`resource_arn(Option<String>)`](crate::operation::delete_resource_policy::DeleteResourcePolicyOutput::resource_arn): <p>Amazon Resource Name (ARN) of the resource from which the policy was deleted.</p>
    ///   - [`policy(Option<String>)`](crate::operation::delete_resource_policy::DeleteResourcePolicyOutput::policy): <p>The policy previously attached to the resource.</p>
    /// - On failure, responds with [`SdkError<DeleteResourcePolicyError>`](crate::operation::delete_resource_policy::DeleteResourcePolicyError)
    pub fn delete_resource_policy(&self) -> crate::operation::delete_resource_policy::builders::DeleteResourcePolicyFluentBuilder {
        crate::operation::delete_resource_policy::builders::DeleteResourcePolicyFluentBuilder::new(self.handle.clone())
    }
}
