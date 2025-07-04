// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeregisterAccountAssociation`](crate::operation::deregister_account_association::builders::DeregisterAccountAssociationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`managed_thing_id(impl Into<String>)`](crate::operation::deregister_account_association::builders::DeregisterAccountAssociationFluentBuilder::managed_thing_id) / [`set_managed_thing_id(Option<String>)`](crate::operation::deregister_account_association::builders::DeregisterAccountAssociationFluentBuilder::set_managed_thing_id):<br>required: **true**<br><p>The identifier of the managed thing to be deregistered from the account association.</p><br>
    ///   - [`account_association_id(impl Into<String>)`](crate::operation::deregister_account_association::builders::DeregisterAccountAssociationFluentBuilder::account_association_id) / [`set_account_association_id(Option<String>)`](crate::operation::deregister_account_association::builders::DeregisterAccountAssociationFluentBuilder::set_account_association_id):<br>required: **true**<br><p>The unique identifier of the account association to be deregistered.</p><br>
    /// - On success, responds with [`DeregisterAccountAssociationOutput`](crate::operation::deregister_account_association::DeregisterAccountAssociationOutput)
    /// - On failure, responds with [`SdkError<DeregisterAccountAssociationError>`](crate::operation::deregister_account_association::DeregisterAccountAssociationError)
    pub fn deregister_account_association(
        &self,
    ) -> crate::operation::deregister_account_association::builders::DeregisterAccountAssociationFluentBuilder {
        crate::operation::deregister_account_association::builders::DeregisterAccountAssociationFluentBuilder::new(self.handle.clone())
    }
}
