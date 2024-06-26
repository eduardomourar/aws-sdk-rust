// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteOrganization`](crate::operation::delete_organization::builders::DeleteOrganizationFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::delete_organization::builders::DeleteOrganizationFluentBuilder::send) it.
    /// - On success, responds with [`DeleteOrganizationOutput`](crate::operation::delete_organization::DeleteOrganizationOutput)
    /// - On failure, responds with [`SdkError<DeleteOrganizationError>`](crate::operation::delete_organization::DeleteOrganizationError)
    pub fn delete_organization(&self) -> crate::operation::delete_organization::builders::DeleteOrganizationFluentBuilder {
        crate::operation::delete_organization::builders::DeleteOrganizationFluentBuilder::new(self.handle.clone())
    }
}
