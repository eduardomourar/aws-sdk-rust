// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteDataIntegrationFlow`](crate::operation::delete_data_integration_flow::builders::DeleteDataIntegrationFlowFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`instance_id(impl Into<String>)`](crate::operation::delete_data_integration_flow::builders::DeleteDataIntegrationFlowFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::delete_data_integration_flow::builders::DeleteDataIntegrationFlowFluentBuilder::set_instance_id):<br>required: **true**<br><p>The Amazon Web Services Supply Chain instance identifier.</p><br>
    ///   - [`name(impl Into<String>)`](crate::operation::delete_data_integration_flow::builders::DeleteDataIntegrationFlowFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::delete_data_integration_flow::builders::DeleteDataIntegrationFlowFluentBuilder::set_name):<br>required: **true**<br><p>The name of the DataIntegrationFlow to be deleted.</p><br>
    /// - On success, responds with [`DeleteDataIntegrationFlowOutput`](crate::operation::delete_data_integration_flow::DeleteDataIntegrationFlowOutput) with field(s):
    ///   - [`instance_id(String)`](crate::operation::delete_data_integration_flow::DeleteDataIntegrationFlowOutput::instance_id): <p>The Amazon Web Services Supply Chain instance identifier.</p>
    ///   - [`name(String)`](crate::operation::delete_data_integration_flow::DeleteDataIntegrationFlowOutput::name): <p>The name of the DataIntegrationFlow deleted.</p>
    /// - On failure, responds with [`SdkError<DeleteDataIntegrationFlowError>`](crate::operation::delete_data_integration_flow::DeleteDataIntegrationFlowError)
    pub fn delete_data_integration_flow(&self) -> crate::operation::delete_data_integration_flow::builders::DeleteDataIntegrationFlowFluentBuilder {
        crate::operation::delete_data_integration_flow::builders::DeleteDataIntegrationFlowFluentBuilder::new(self.handle.clone())
    }
}
