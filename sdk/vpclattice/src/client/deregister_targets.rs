// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeregisterTargets`](crate::operation::deregister_targets::builders::DeregisterTargetsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`target_group_identifier(impl Into<String>)`](crate::operation::deregister_targets::builders::DeregisterTargetsFluentBuilder::target_group_identifier) / [`set_target_group_identifier(Option<String>)`](crate::operation::deregister_targets::builders::DeregisterTargetsFluentBuilder::set_target_group_identifier):<br>required: **true**<br><p>The ID or ARN of the target group.</p><br>
    ///   - [`targets(Target)`](crate::operation::deregister_targets::builders::DeregisterTargetsFluentBuilder::targets) / [`set_targets(Option<Vec::<Target>>)`](crate::operation::deregister_targets::builders::DeregisterTargetsFluentBuilder::set_targets):<br>required: **true**<br><p>The targets to deregister.</p><br>
    /// - On success, responds with [`DeregisterTargetsOutput`](crate::operation::deregister_targets::DeregisterTargetsOutput) with field(s):
    ///   - [`successful(Option<Vec::<Target>>)`](crate::operation::deregister_targets::DeregisterTargetsOutput::successful): <p>The targets that were successfully deregistered.</p>
    ///   - [`unsuccessful(Option<Vec::<TargetFailure>>)`](crate::operation::deregister_targets::DeregisterTargetsOutput::unsuccessful): <p>The targets that the operation couldn't deregister.</p>
    /// - On failure, responds with [`SdkError<DeregisterTargetsError>`](crate::operation::deregister_targets::DeregisterTargetsError)
    pub fn deregister_targets(&self) -> crate::operation::deregister_targets::builders::DeregisterTargetsFluentBuilder {
        crate::operation::deregister_targets::builders::DeregisterTargetsFluentBuilder::new(self.handle.clone())
    }
}
