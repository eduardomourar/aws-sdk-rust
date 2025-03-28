// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutTableMaintenanceConfiguration`](crate::operation::put_table_maintenance_configuration::builders::PutTableMaintenanceConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`table_bucket_arn(impl Into<String>)`](crate::operation::put_table_maintenance_configuration::builders::PutTableMaintenanceConfigurationFluentBuilder::table_bucket_arn) / [`set_table_bucket_arn(Option<String>)`](crate::operation::put_table_maintenance_configuration::builders::PutTableMaintenanceConfigurationFluentBuilder::set_table_bucket_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the table associated with the maintenance configuration.</p><br>
    ///   - [`namespace(impl Into<String>)`](crate::operation::put_table_maintenance_configuration::builders::PutTableMaintenanceConfigurationFluentBuilder::namespace) / [`set_namespace(Option<String>)`](crate::operation::put_table_maintenance_configuration::builders::PutTableMaintenanceConfigurationFluentBuilder::set_namespace):<br>required: **true**<br><p>The namespace of the table.</p><br>
    ///   - [`name(impl Into<String>)`](crate::operation::put_table_maintenance_configuration::builders::PutTableMaintenanceConfigurationFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::put_table_maintenance_configuration::builders::PutTableMaintenanceConfigurationFluentBuilder::set_name):<br>required: **true**<br><p>The name of the maintenance configuration.</p><br>
    ///   - [`r#type(TableMaintenanceType)`](crate::operation::put_table_maintenance_configuration::builders::PutTableMaintenanceConfigurationFluentBuilder::type) / [`set_type(Option<TableMaintenanceType>)`](crate::operation::put_table_maintenance_configuration::builders::PutTableMaintenanceConfigurationFluentBuilder::set_type):<br>required: **true**<br><p>The type of the maintenance configuration.</p><br>
    ///   - [`value(TableMaintenanceConfigurationValue)`](crate::operation::put_table_maintenance_configuration::builders::PutTableMaintenanceConfigurationFluentBuilder::value) / [`set_value(Option<TableMaintenanceConfigurationValue>)`](crate::operation::put_table_maintenance_configuration::builders::PutTableMaintenanceConfigurationFluentBuilder::set_value):<br>required: **true**<br><p>Defines the values of the maintenance configuration for the table.</p><br>
    /// - On success, responds with [`PutTableMaintenanceConfigurationOutput`](crate::operation::put_table_maintenance_configuration::PutTableMaintenanceConfigurationOutput)
    /// - On failure, responds with [`SdkError<PutTableMaintenanceConfigurationError>`](crate::operation::put_table_maintenance_configuration::PutTableMaintenanceConfigurationError)
    pub fn put_table_maintenance_configuration(
        &self,
    ) -> crate::operation::put_table_maintenance_configuration::builders::PutTableMaintenanceConfigurationFluentBuilder {
        crate::operation::put_table_maintenance_configuration::builders::PutTableMaintenanceConfigurationFluentBuilder::new(self.handle.clone())
    }
}
