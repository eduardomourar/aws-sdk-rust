// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetTenant`](crate::operation::get_tenant::builders::GetTenantFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`tenant_name(impl Into<String>)`](crate::operation::get_tenant::builders::GetTenantFluentBuilder::tenant_name) / [`set_tenant_name(Option<String>)`](crate::operation::get_tenant::builders::GetTenantFluentBuilder::set_tenant_name):<br>required: **true**<br><p>The name of the tenant to retrieve information about.</p><br>
    /// - On success, responds with [`GetTenantOutput`](crate::operation::get_tenant::GetTenantOutput) with field(s):
    ///   - [`tenant(Option<Tenant>)`](crate::operation::get_tenant::GetTenantOutput::tenant): <p>A structure that contains details about the tenant.</p>
    /// - On failure, responds with [`SdkError<GetTenantError>`](crate::operation::get_tenant::GetTenantError)
    pub fn get_tenant(&self) -> crate::operation::get_tenant::builders::GetTenantFluentBuilder {
        crate::operation::get_tenant::builders::GetTenantFluentBuilder::new(self.handle.clone())
    }
}
