// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`SetTokenVaultCMK`](crate::operation::set_token_vault_cmk::builders::SetTokenVaultCMKFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`token_vault_id(impl Into<String>)`](crate::operation::set_token_vault_cmk::builders::SetTokenVaultCMKFluentBuilder::token_vault_id) / [`set_token_vault_id(Option<String>)`](crate::operation::set_token_vault_cmk::builders::SetTokenVaultCMKFluentBuilder::set_token_vault_id):<br>required: **false**<br><p>The unique identifier of the token vault to update.</p><br>
    ///   - [`kms_configuration(KmsConfiguration)`](crate::operation::set_token_vault_cmk::builders::SetTokenVaultCMKFluentBuilder::kms_configuration) / [`set_kms_configuration(Option<KmsConfiguration>)`](crate::operation::set_token_vault_cmk::builders::SetTokenVaultCMKFluentBuilder::set_kms_configuration):<br>required: **true**<br><p>The KMS configuration for the token vault, including the key type and KMS key ARN.</p><br>
    /// - On success, responds with [`SetTokenVaultCmkOutput`](crate::operation::set_token_vault_cmk::SetTokenVaultCmkOutput) with field(s):
    ///   - [`token_vault_id(String)`](crate::operation::set_token_vault_cmk::SetTokenVaultCmkOutput::token_vault_id): <p>The ID of the token vault.</p>
    ///   - [`kms_configuration(Option<KmsConfiguration>)`](crate::operation::set_token_vault_cmk::SetTokenVaultCmkOutput::kms_configuration): <p>The KMS configuration for the token vault.</p>
    ///   - [`last_modified_date(DateTime)`](crate::operation::set_token_vault_cmk::SetTokenVaultCmkOutput::last_modified_date): <p>The timestamp when the token vault was last modified.</p>
    /// - On failure, responds with [`SdkError<SetTokenVaultCMKError>`](crate::operation::set_token_vault_cmk::SetTokenVaultCMKError)
    pub fn set_token_vault_cmk(&self) -> crate::operation::set_token_vault_cmk::builders::SetTokenVaultCMKFluentBuilder {
        crate::operation::set_token_vault_cmk::builders::SetTokenVaultCMKFluentBuilder::new(self.handle.clone())
    }
}
