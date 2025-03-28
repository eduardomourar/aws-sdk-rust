// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateTableOptimizer`](crate::operation::update_table_optimizer::builders::UpdateTableOptimizerFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`catalog_id(impl Into<String>)`](crate::operation::update_table_optimizer::builders::UpdateTableOptimizerFluentBuilder::catalog_id) / [`set_catalog_id(Option<String>)`](crate::operation::update_table_optimizer::builders::UpdateTableOptimizerFluentBuilder::set_catalog_id):<br>required: **true**<br><p>The Catalog ID of the table.</p><br>
    ///   - [`database_name(impl Into<String>)`](crate::operation::update_table_optimizer::builders::UpdateTableOptimizerFluentBuilder::database_name) / [`set_database_name(Option<String>)`](crate::operation::update_table_optimizer::builders::UpdateTableOptimizerFluentBuilder::set_database_name):<br>required: **true**<br><p>The name of the database in the catalog in which the table resides.</p><br>
    ///   - [`table_name(impl Into<String>)`](crate::operation::update_table_optimizer::builders::UpdateTableOptimizerFluentBuilder::table_name) / [`set_table_name(Option<String>)`](crate::operation::update_table_optimizer::builders::UpdateTableOptimizerFluentBuilder::set_table_name):<br>required: **true**<br><p>The name of the table.</p><br>
    ///   - [`r#type(TableOptimizerType)`](crate::operation::update_table_optimizer::builders::UpdateTableOptimizerFluentBuilder::type) / [`set_type(Option<TableOptimizerType>)`](crate::operation::update_table_optimizer::builders::UpdateTableOptimizerFluentBuilder::set_type):<br>required: **true**<br><p>The type of table optimizer.</p><br>
    ///   - [`table_optimizer_configuration(TableOptimizerConfiguration)`](crate::operation::update_table_optimizer::builders::UpdateTableOptimizerFluentBuilder::table_optimizer_configuration) / [`set_table_optimizer_configuration(Option<TableOptimizerConfiguration>)`](crate::operation::update_table_optimizer::builders::UpdateTableOptimizerFluentBuilder::set_table_optimizer_configuration):<br>required: **true**<br><p>A <code>TableOptimizerConfiguration</code> object representing the configuration of a table optimizer.</p><br>
    /// - On success, responds with [`UpdateTableOptimizerOutput`](crate::operation::update_table_optimizer::UpdateTableOptimizerOutput)
    /// - On failure, responds with [`SdkError<UpdateTableOptimizerError>`](crate::operation::update_table_optimizer::UpdateTableOptimizerError)
    pub fn update_table_optimizer(&self) -> crate::operation::update_table_optimizer::builders::UpdateTableOptimizerFluentBuilder {
        crate::operation::update_table_optimizer::builders::UpdateTableOptimizerFluentBuilder::new(self.handle.clone())
    }
}
