// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`NotifyResourceDeploymentStatusChange`](crate::operation::notify_resource_deployment_status_change::builders::NotifyResourceDeploymentStatusChangeFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_arn(impl Into<String>)`](crate::operation::notify_resource_deployment_status_change::builders::NotifyResourceDeploymentStatusChangeFluentBuilder::resource_arn) / [`set_resource_arn(Option<String>)`](crate::operation::notify_resource_deployment_status_change::builders::NotifyResourceDeploymentStatusChangeFluentBuilder::set_resource_arn):<br>required: **true**<br><p>The provisioned resource Amazon Resource Name (ARN).</p><br>
    ///   - [`status(ResourceDeploymentStatus)`](crate::operation::notify_resource_deployment_status_change::builders::NotifyResourceDeploymentStatusChangeFluentBuilder::status) / [`set_status(Option<ResourceDeploymentStatus>)`](crate::operation::notify_resource_deployment_status_change::builders::NotifyResourceDeploymentStatusChangeFluentBuilder::set_status):<br>required: **false**<br><p>The status of your provisioned resource.</p><br>
    ///   - [`outputs(Output)`](crate::operation::notify_resource_deployment_status_change::builders::NotifyResourceDeploymentStatusChangeFluentBuilder::outputs) / [`set_outputs(Option<Vec::<Output>>)`](crate::operation::notify_resource_deployment_status_change::builders::NotifyResourceDeploymentStatusChangeFluentBuilder::set_outputs):<br>required: **false**<br><p>The provisioned resource state change detail data that's returned by Proton.</p><br>
    ///   - [`deployment_id(impl Into<String>)`](crate::operation::notify_resource_deployment_status_change::builders::NotifyResourceDeploymentStatusChangeFluentBuilder::deployment_id) / [`set_deployment_id(Option<String>)`](crate::operation::notify_resource_deployment_status_change::builders::NotifyResourceDeploymentStatusChangeFluentBuilder::set_deployment_id):<br>required: **false**<br><p>The deployment ID for your provisioned resource.</p><br>
    ///   - [`status_message(impl Into<String>)`](crate::operation::notify_resource_deployment_status_change::builders::NotifyResourceDeploymentStatusChangeFluentBuilder::status_message) / [`set_status_message(Option<String>)`](crate::operation::notify_resource_deployment_status_change::builders::NotifyResourceDeploymentStatusChangeFluentBuilder::set_status_message):<br>required: **false**<br><p>The deployment status message for your provisioned resource.</p><br>
    /// - On success, responds with [`NotifyResourceDeploymentStatusChangeOutput`](crate::operation::notify_resource_deployment_status_change::NotifyResourceDeploymentStatusChangeOutput)
    /// - On failure, responds with [`SdkError<NotifyResourceDeploymentStatusChangeError>`](crate::operation::notify_resource_deployment_status_change::NotifyResourceDeploymentStatusChangeError)
    pub fn notify_resource_deployment_status_change(
        &self,
    ) -> crate::operation::notify_resource_deployment_status_change::builders::NotifyResourceDeploymentStatusChangeFluentBuilder {
        crate::operation::notify_resource_deployment_status_change::builders::NotifyResourceDeploymentStatusChangeFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
