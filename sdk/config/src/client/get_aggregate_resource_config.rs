// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetAggregateResourceConfig`](crate::operation::get_aggregate_resource_config::builders::GetAggregateResourceConfigFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`configuration_aggregator_name(impl Into<String>)`](crate::operation::get_aggregate_resource_config::builders::GetAggregateResourceConfigFluentBuilder::configuration_aggregator_name) / [`set_configuration_aggregator_name(Option<String>)`](crate::operation::get_aggregate_resource_config::builders::GetAggregateResourceConfigFluentBuilder::set_configuration_aggregator_name):<br>required: **true**<br><p>The name of the configuration aggregator.</p><br>
    ///   - [`resource_identifier(AggregateResourceIdentifier)`](crate::operation::get_aggregate_resource_config::builders::GetAggregateResourceConfigFluentBuilder::resource_identifier) / [`set_resource_identifier(Option<AggregateResourceIdentifier>)`](crate::operation::get_aggregate_resource_config::builders::GetAggregateResourceConfigFluentBuilder::set_resource_identifier):<br>required: **true**<br><p>An object that identifies aggregate resource.</p><br>
    /// - On success, responds with [`GetAggregateResourceConfigOutput`](crate::operation::get_aggregate_resource_config::GetAggregateResourceConfigOutput) with field(s):
    ///   - [`configuration_item(Option<ConfigurationItem>)`](crate::operation::get_aggregate_resource_config::GetAggregateResourceConfigOutput::configuration_item): <p>Returns a <code>ConfigurationItem</code> object.</p>
    /// - On failure, responds with [`SdkError<GetAggregateResourceConfigError>`](crate::operation::get_aggregate_resource_config::GetAggregateResourceConfigError)
    pub fn get_aggregate_resource_config(
        &self,
    ) -> crate::operation::get_aggregate_resource_config::builders::GetAggregateResourceConfigFluentBuilder {
        crate::operation::get_aggregate_resource_config::builders::GetAggregateResourceConfigFluentBuilder::new(self.handle.clone())
    }
}
