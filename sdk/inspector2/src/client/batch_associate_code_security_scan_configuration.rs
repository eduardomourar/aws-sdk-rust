// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`BatchAssociateCodeSecurityScanConfiguration`](crate::operation::batch_associate_code_security_scan_configuration::builders::BatchAssociateCodeSecurityScanConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`associate_configuration_requests(AssociateConfigurationRequest)`](crate::operation::batch_associate_code_security_scan_configuration::builders::BatchAssociateCodeSecurityScanConfigurationFluentBuilder::associate_configuration_requests) / [`set_associate_configuration_requests(Option<Vec::<AssociateConfigurationRequest>>)`](crate::operation::batch_associate_code_security_scan_configuration::builders::BatchAssociateCodeSecurityScanConfigurationFluentBuilder::set_associate_configuration_requests):<br>required: **true**<br><p>A list of code repositories to associate with the specified scan configuration.</p><br>
    /// - On success, responds with [`BatchAssociateCodeSecurityScanConfigurationOutput`](crate::operation::batch_associate_code_security_scan_configuration::BatchAssociateCodeSecurityScanConfigurationOutput) with field(s):
    ///   - [`failed_associations(Option<Vec::<FailedAssociationResult>>)`](crate::operation::batch_associate_code_security_scan_configuration::BatchAssociateCodeSecurityScanConfigurationOutput::failed_associations): <p>Details of any code repositories that failed to be associated with the scan configuration.</p>
    ///   - [`successful_associations(Option<Vec::<SuccessfulAssociationResult>>)`](crate::operation::batch_associate_code_security_scan_configuration::BatchAssociateCodeSecurityScanConfigurationOutput::successful_associations): <p>Details of code repositories that were successfully associated with the scan configuration.</p>
    /// - On failure, responds with [`SdkError<BatchAssociateCodeSecurityScanConfigurationError>`](crate::operation::batch_associate_code_security_scan_configuration::BatchAssociateCodeSecurityScanConfigurationError)
    pub fn batch_associate_code_security_scan_configuration(
        &self,
    ) -> crate::operation::batch_associate_code_security_scan_configuration::builders::BatchAssociateCodeSecurityScanConfigurationFluentBuilder {
        crate::operation::batch_associate_code_security_scan_configuration::builders::BatchAssociateCodeSecurityScanConfigurationFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
