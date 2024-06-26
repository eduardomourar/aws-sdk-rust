// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetDomainMaintenanceStatus`](crate::operation::get_domain_maintenance_status::builders::GetDomainMaintenanceStatusFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`domain_name(impl Into<String>)`](crate::operation::get_domain_maintenance_status::builders::GetDomainMaintenanceStatusFluentBuilder::domain_name) / [`set_domain_name(Option<String>)`](crate::operation::get_domain_maintenance_status::builders::GetDomainMaintenanceStatusFluentBuilder::set_domain_name):<br>required: **true**<br><p>The name of the domain.</p><br>
    ///   - [`maintenance_id(impl Into<String>)`](crate::operation::get_domain_maintenance_status::builders::GetDomainMaintenanceStatusFluentBuilder::maintenance_id) / [`set_maintenance_id(Option<String>)`](crate::operation::get_domain_maintenance_status::builders::GetDomainMaintenanceStatusFluentBuilder::set_maintenance_id):<br>required: **true**<br><p>The request ID of the maintenance action.</p><br>
    /// - On success, responds with [`GetDomainMaintenanceStatusOutput`](crate::operation::get_domain_maintenance_status::GetDomainMaintenanceStatusOutput) with field(s):
    ///   - [`status(Option<MaintenanceStatus>)`](crate::operation::get_domain_maintenance_status::GetDomainMaintenanceStatusOutput::status): <p>The status of the maintenance action.</p>
    ///   - [`status_message(Option<String>)`](crate::operation::get_domain_maintenance_status::GetDomainMaintenanceStatusOutput::status_message): <p>The status message of the maintenance action.</p>
    ///   - [`node_id(Option<String>)`](crate::operation::get_domain_maintenance_status::GetDomainMaintenanceStatusOutput::node_id): <p>The node ID of the maintenance action.</p>
    ///   - [`action(Option<MaintenanceType>)`](crate::operation::get_domain_maintenance_status::GetDomainMaintenanceStatusOutput::action): <p>The action name.</p>
    ///   - [`created_at(Option<DateTime>)`](crate::operation::get_domain_maintenance_status::GetDomainMaintenanceStatusOutput::created_at): <p>The time at which the action was created.</p>
    ///   - [`updated_at(Option<DateTime>)`](crate::operation::get_domain_maintenance_status::GetDomainMaintenanceStatusOutput::updated_at): <p>The time at which the action was updated.</p>
    /// - On failure, responds with [`SdkError<GetDomainMaintenanceStatusError>`](crate::operation::get_domain_maintenance_status::GetDomainMaintenanceStatusError)
    pub fn get_domain_maintenance_status(
        &self,
    ) -> crate::operation::get_domain_maintenance_status::builders::GetDomainMaintenanceStatusFluentBuilder {
        crate::operation::get_domain_maintenance_status::builders::GetDomainMaintenanceStatusFluentBuilder::new(self.handle.clone())
    }
}
