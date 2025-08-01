// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateTenant`](crate::operation::create_tenant::builders::CreateTenantFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`tenant_name(impl Into<String>)`](crate::operation::create_tenant::builders::CreateTenantFluentBuilder::tenant_name) / [`set_tenant_name(Option<String>)`](crate::operation::create_tenant::builders::CreateTenantFluentBuilder::set_tenant_name):<br>required: **true**<br><p>The name of the tenant to create. The name can contain up to 64 alphanumeric characters, including letters, numbers, hyphens (-) and underscores (_) only.</p><br>
    ///   - [`tags(Tag)`](crate::operation::create_tenant::builders::CreateTenantFluentBuilder::tags) / [`set_tags(Option<Vec::<Tag>>)`](crate::operation::create_tenant::builders::CreateTenantFluentBuilder::set_tags):<br>required: **false**<br><p>An array of objects that define the tags (keys and values) to associate with the tenant</p><br>
    /// - On success, responds with [`CreateTenantOutput`](crate::operation::create_tenant::CreateTenantOutput) with field(s):
    ///   - [`tenant_name(Option<String>)`](crate::operation::create_tenant::CreateTenantOutput::tenant_name): <p>The name of the tenant.</p>
    ///   - [`tenant_id(Option<String>)`](crate::operation::create_tenant::CreateTenantOutput::tenant_id): <p>A unique identifier for the tenant.</p>
    ///   - [`tenant_arn(Option<String>)`](crate::operation::create_tenant::CreateTenantOutput::tenant_arn): <p>The Amazon Resource Name (ARN) of the tenant.</p>
    ///   - [`created_timestamp(Option<DateTime>)`](crate::operation::create_tenant::CreateTenantOutput::created_timestamp): <p>The date and time when the tenant was created.</p>
    ///   - [`tags(Option<Vec::<Tag>>)`](crate::operation::create_tenant::CreateTenantOutput::tags): <p>An array of objects that define the tags (keys and values) associated with the tenant.</p>
    ///   - [`sending_status(Option<SendingStatus>)`](crate::operation::create_tenant::CreateTenantOutput::sending_status): <p>The status of email sending capability for the tenant.</p>
    /// - On failure, responds with [`SdkError<CreateTenantError>`](crate::operation::create_tenant::CreateTenantError)
    pub fn create_tenant(&self) -> crate::operation::create_tenant::builders::CreateTenantFluentBuilder {
        crate::operation::create_tenant::builders::CreateTenantFluentBuilder::new(self.handle.clone())
    }
}
