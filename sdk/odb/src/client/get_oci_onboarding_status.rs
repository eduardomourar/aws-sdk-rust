// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetOciOnboardingStatus`](crate::operation::get_oci_onboarding_status::builders::GetOciOnboardingStatusFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::get_oci_onboarding_status::builders::GetOciOnboardingStatusFluentBuilder::send) it.
    /// - On success, responds with [`GetOciOnboardingStatusOutput`](crate::operation::get_oci_onboarding_status::GetOciOnboardingStatusOutput) with field(s):
    ///   - [`status(Option<OciOnboardingStatus>)`](crate::operation::get_oci_onboarding_status::GetOciOnboardingStatusOutput::status): <p></p>
    ///   - [`existing_tenancy_activation_link(Option<String>)`](crate::operation::get_oci_onboarding_status::GetOciOnboardingStatusOutput::existing_tenancy_activation_link): <p>The existing OCI tenancy activation link for your Amazon Web Services account.</p>
    ///   - [`new_tenancy_activation_link(Option<String>)`](crate::operation::get_oci_onboarding_status::GetOciOnboardingStatusOutput::new_tenancy_activation_link): <p>A new OCI tenancy activation link for your Amazon Web Services account.</p>
    /// - On failure, responds with [`SdkError<GetOciOnboardingStatusError>`](crate::operation::get_oci_onboarding_status::GetOciOnboardingStatusError)
    pub fn get_oci_onboarding_status(&self) -> crate::operation::get_oci_onboarding_status::builders::GetOciOnboardingStatusFluentBuilder {
        crate::operation::get_oci_onboarding_status::builders::GetOciOnboardingStatusFluentBuilder::new(self.handle.clone())
    }
}
