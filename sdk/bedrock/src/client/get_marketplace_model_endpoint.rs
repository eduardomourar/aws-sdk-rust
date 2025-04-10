// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetMarketplaceModelEndpoint`](crate::operation::get_marketplace_model_endpoint::builders::GetMarketplaceModelEndpointFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`endpoint_arn(impl Into<String>)`](crate::operation::get_marketplace_model_endpoint::builders::GetMarketplaceModelEndpointFluentBuilder::endpoint_arn) / [`set_endpoint_arn(Option<String>)`](crate::operation::get_marketplace_model_endpoint::builders::GetMarketplaceModelEndpointFluentBuilder::set_endpoint_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the endpoint you want to get information about.</p><br>
    /// - On success, responds with [`GetMarketplaceModelEndpointOutput`](crate::operation::get_marketplace_model_endpoint::GetMarketplaceModelEndpointOutput) with field(s):
    ///   - [`marketplace_model_endpoint(Option<MarketplaceModelEndpoint>)`](crate::operation::get_marketplace_model_endpoint::GetMarketplaceModelEndpointOutput::marketplace_model_endpoint): <p>Details about the requested endpoint.</p>
    /// - On failure, responds with [`SdkError<GetMarketplaceModelEndpointError>`](crate::operation::get_marketplace_model_endpoint::GetMarketplaceModelEndpointError)
    pub fn get_marketplace_model_endpoint(
        &self,
    ) -> crate::operation::get_marketplace_model_endpoint::builders::GetMarketplaceModelEndpointFluentBuilder {
        crate::operation::get_marketplace_model_endpoint::builders::GetMarketplaceModelEndpointFluentBuilder::new(self.handle.clone())
    }
}
