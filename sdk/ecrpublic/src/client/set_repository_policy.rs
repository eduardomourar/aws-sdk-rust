// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`SetRepositoryPolicy`](crate::operation::set_repository_policy::builders::SetRepositoryPolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`registry_id(impl Into<String>)`](crate::operation::set_repository_policy::builders::SetRepositoryPolicyFluentBuilder::registry_id) / [`set_registry_id(Option<String>)`](crate::operation::set_repository_policy::builders::SetRepositoryPolicyFluentBuilder::set_registry_id):<br>required: **false**<br><p>The Amazon Web Services account ID that's associated with the registry that contains the repository. If you do not specify a registry, the default public registry is assumed.</p><br>
    ///   - [`repository_name(impl Into<String>)`](crate::operation::set_repository_policy::builders::SetRepositoryPolicyFluentBuilder::repository_name) / [`set_repository_name(Option<String>)`](crate::operation::set_repository_policy::builders::SetRepositoryPolicyFluentBuilder::set_repository_name):<br>required: **true**<br><p>The name of the repository to receive the policy.</p><br>
    ///   - [`policy_text(impl Into<String>)`](crate::operation::set_repository_policy::builders::SetRepositoryPolicyFluentBuilder::policy_text) / [`set_policy_text(Option<String>)`](crate::operation::set_repository_policy::builders::SetRepositoryPolicyFluentBuilder::set_policy_text):<br>required: **true**<br><p>The JSON repository policy text to apply to the repository. For more information, see <a href="https://docs.aws.amazon.com/AmazonECR/latest/userguide/repository-policy-examples.html">Amazon ECR Repository Policies</a> in the <i>Amazon Elastic Container Registry User Guide</i>.</p><br>
    ///   - [`force(bool)`](crate::operation::set_repository_policy::builders::SetRepositoryPolicyFluentBuilder::force) / [`set_force(Option<bool>)`](crate::operation::set_repository_policy::builders::SetRepositoryPolicyFluentBuilder::set_force):<br>required: **false**<br><p>If the policy that you want to set on a repository policy would prevent you from setting another policy in the future, you must force the <code>SetRepositoryPolicy</code> operation. This prevents accidental repository lockouts.</p><br>
    /// - On success, responds with [`SetRepositoryPolicyOutput`](crate::operation::set_repository_policy::SetRepositoryPolicyOutput) with field(s):
    ///   - [`registry_id(Option<String>)`](crate::operation::set_repository_policy::SetRepositoryPolicyOutput::registry_id): <p>The registry ID that's associated with the request.</p>
    ///   - [`repository_name(Option<String>)`](crate::operation::set_repository_policy::SetRepositoryPolicyOutput::repository_name): <p>The repository name that's associated with the request.</p>
    ///   - [`policy_text(Option<String>)`](crate::operation::set_repository_policy::SetRepositoryPolicyOutput::policy_text): <p>The JSON repository policy text that's applied to the repository.</p>
    /// - On failure, responds with [`SdkError<SetRepositoryPolicyError>`](crate::operation::set_repository_policy::SetRepositoryPolicyError)
    pub fn set_repository_policy(&self) -> crate::operation::set_repository_policy::builders::SetRepositoryPolicyFluentBuilder {
        crate::operation::set_repository_policy::builders::SetRepositoryPolicyFluentBuilder::new(self.handle.clone())
    }
}
