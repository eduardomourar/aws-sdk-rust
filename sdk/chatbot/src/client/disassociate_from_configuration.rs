// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DisassociateFromConfiguration`](crate::operation::disassociate_from_configuration::builders::DisassociateFromConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource(impl Into<String>)`](crate::operation::disassociate_from_configuration::builders::DisassociateFromConfigurationFluentBuilder::resource) / [`set_resource(Option<String>)`](crate::operation::disassociate_from_configuration::builders::DisassociateFromConfigurationFluentBuilder::set_resource):<br>required: **true**<br><p>The resource (for example, a custom action) Amazon Resource Name (ARN) to unlink.</p><br>
    ///   - [`chat_configuration(impl Into<String>)`](crate::operation::disassociate_from_configuration::builders::DisassociateFromConfigurationFluentBuilder::chat_configuration) / [`set_chat_configuration(Option<String>)`](crate::operation::disassociate_from_configuration::builders::DisassociateFromConfigurationFluentBuilder::set_chat_configuration):<br>required: **true**<br><p>The channel configuration the resource is being disassociated from.</p><br>
    /// - On success, responds with [`DisassociateFromConfigurationOutput`](crate::operation::disassociate_from_configuration::DisassociateFromConfigurationOutput)
    /// - On failure, responds with [`SdkError<DisassociateFromConfigurationError>`](crate::operation::disassociate_from_configuration::DisassociateFromConfigurationError)
    pub fn disassociate_from_configuration(
        &self,
    ) -> crate::operation::disassociate_from_configuration::builders::DisassociateFromConfigurationFluentBuilder {
        crate::operation::disassociate_from_configuration::builders::DisassociateFromConfigurationFluentBuilder::new(self.handle.clone())
    }
}
