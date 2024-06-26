// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateTestConfiguration`](crate::operation::update_test_configuration::builders::UpdateTestConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`test_configuration_id(impl Into<String>)`](crate::operation::update_test_configuration::builders::UpdateTestConfigurationFluentBuilder::test_configuration_id) / [`set_test_configuration_id(Option<String>)`](crate::operation::update_test_configuration::builders::UpdateTestConfigurationFluentBuilder::set_test_configuration_id):<br>required: **true**<br><p>The test configuration ID of the test configuration.</p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::update_test_configuration::builders::UpdateTestConfigurationFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_test_configuration::builders::UpdateTestConfigurationFluentBuilder::set_description):<br>required: **false**<br><p>The description of the test configuration.</p><br>
    ///   - [`resources(Resource)`](crate::operation::update_test_configuration::builders::UpdateTestConfigurationFluentBuilder::resources) / [`set_resources(Option<Vec::<Resource>>)`](crate::operation::update_test_configuration::builders::UpdateTestConfigurationFluentBuilder::set_resources):<br>required: **false**<br><p>The resources of the test configuration.</p><br>
    ///   - [`properties(impl Into<String>, impl Into<String>)`](crate::operation::update_test_configuration::builders::UpdateTestConfigurationFluentBuilder::properties) / [`set_properties(Option<HashMap::<String, String>>)`](crate::operation::update_test_configuration::builders::UpdateTestConfigurationFluentBuilder::set_properties):<br>required: **false**<br><p>The properties of the test configuration.</p><br>
    ///   - [`service_settings(ServiceSettings)`](crate::operation::update_test_configuration::builders::UpdateTestConfigurationFluentBuilder::service_settings) / [`set_service_settings(Option<ServiceSettings>)`](crate::operation::update_test_configuration::builders::UpdateTestConfigurationFluentBuilder::set_service_settings):<br>required: **false**<br><p>The service settings of the test configuration.</p><br>
    /// - On success, responds with [`UpdateTestConfigurationOutput`](crate::operation::update_test_configuration::UpdateTestConfigurationOutput) with field(s):
    ///   - [`test_configuration_id(String)`](crate::operation::update_test_configuration::UpdateTestConfigurationOutput::test_configuration_id): <p>The configuration ID of the test configuration.</p>
    ///   - [`test_configuration_version(i32)`](crate::operation::update_test_configuration::UpdateTestConfigurationOutput::test_configuration_version): <p>The configuration version of the test configuration.</p>
    /// - On failure, responds with [`SdkError<UpdateTestConfigurationError>`](crate::operation::update_test_configuration::UpdateTestConfigurationError)
    pub fn update_test_configuration(&self) -> crate::operation::update_test_configuration::builders::UpdateTestConfigurationFluentBuilder {
        crate::operation::update_test_configuration::builders::UpdateTestConfigurationFluentBuilder::new(self.handle.clone())
    }
}
