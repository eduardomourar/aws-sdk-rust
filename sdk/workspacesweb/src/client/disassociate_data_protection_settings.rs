// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DisassociateDataProtectionSettings`](crate::operation::disassociate_data_protection_settings::builders::DisassociateDataProtectionSettingsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`portal_arn(impl Into<String>)`](crate::operation::disassociate_data_protection_settings::builders::DisassociateDataProtectionSettingsFluentBuilder::portal_arn) / [`set_portal_arn(Option<String>)`](crate::operation::disassociate_data_protection_settings::builders::DisassociateDataProtectionSettingsFluentBuilder::set_portal_arn):<br>required: **true**<br><p>The ARN of the web portal.</p><br>
    /// - On success, responds with [`DisassociateDataProtectionSettingsOutput`](crate::operation::disassociate_data_protection_settings::DisassociateDataProtectionSettingsOutput)
    /// - On failure, responds with [`SdkError<DisassociateDataProtectionSettingsError>`](crate::operation::disassociate_data_protection_settings::DisassociateDataProtectionSettingsError)
    pub fn disassociate_data_protection_settings(
        &self,
    ) -> crate::operation::disassociate_data_protection_settings::builders::DisassociateDataProtectionSettingsFluentBuilder {
        crate::operation::disassociate_data_protection_settings::builders::DisassociateDataProtectionSettingsFluentBuilder::new(self.handle.clone())
    }
}
