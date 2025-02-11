// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutAccountSetting`](crate::operation::put_account_setting::builders::PutAccountSettingFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::operation::put_account_setting::builders::PutAccountSettingFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::put_account_setting::builders::PutAccountSettingFluentBuilder::set_name):<br>required: **true**<br><p>The name of the account setting, such as <code>BASIC_SCAN_TYPE_VERSION</code> or <code>REGISTRY_POLICY_SCOPE</code>.</p><br>
    ///   - [`value(impl Into<String>)`](crate::operation::put_account_setting::builders::PutAccountSettingFluentBuilder::value) / [`set_value(Option<String>)`](crate::operation::put_account_setting::builders::PutAccountSettingFluentBuilder::set_value):<br>required: **true**<br><p>Setting value that is specified. The following are valid values for the basic scan type being used: <code>AWS_NATIVE</code> or <code>CLAIR</code>. The following are valid values for the registry policy scope being used: <code>V1</code> or <code>V2</code>.</p><br>
    /// - On success, responds with [`PutAccountSettingOutput`](crate::operation::put_account_setting::PutAccountSettingOutput) with field(s):
    ///   - [`name(Option<String>)`](crate::operation::put_account_setting::PutAccountSettingOutput::name): <p>Retrieves the name of the account setting.</p>
    ///   - [`value(Option<String>)`](crate::operation::put_account_setting::PutAccountSettingOutput::value): <p>Retrieves the value of the specified account setting.</p>
    /// - On failure, responds with [`SdkError<PutAccountSettingError>`](crate::operation::put_account_setting::PutAccountSettingError)
    pub fn put_account_setting(&self) -> crate::operation::put_account_setting::builders::PutAccountSettingFluentBuilder {
        crate::operation::put_account_setting::builders::PutAccountSettingFluentBuilder::new(self.handle.clone())
    }
}
