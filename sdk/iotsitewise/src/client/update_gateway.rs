// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateGateway`](crate::operation::update_gateway::builders::UpdateGatewayFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`gateway_id(impl Into<String>)`](crate::operation::update_gateway::builders::UpdateGatewayFluentBuilder::gateway_id) / [`set_gateway_id(Option<String>)`](crate::operation::update_gateway::builders::UpdateGatewayFluentBuilder::set_gateway_id):<br>required: **true**<br><p>The ID of the gateway to update.</p><br>
    ///   - [`gateway_name(impl Into<String>)`](crate::operation::update_gateway::builders::UpdateGatewayFluentBuilder::gateway_name) / [`set_gateway_name(Option<String>)`](crate::operation::update_gateway::builders::UpdateGatewayFluentBuilder::set_gateway_name):<br>required: **true**<br><p>A unique name for the gateway.</p><br>
    /// - On success, responds with [`UpdateGatewayOutput`](crate::operation::update_gateway::UpdateGatewayOutput)
    /// - On failure, responds with [`SdkError<UpdateGatewayError>`](crate::operation::update_gateway::UpdateGatewayError)
    pub fn update_gateway(&self) -> crate::operation::update_gateway::builders::UpdateGatewayFluentBuilder {
        crate::operation::update_gateway::builders::UpdateGatewayFluentBuilder::new(self.handle.clone())
    }
}
