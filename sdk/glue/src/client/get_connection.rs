// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetConnection`](crate::operation::get_connection::builders::GetConnectionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`catalog_id(impl Into<String>)`](crate::operation::get_connection::builders::GetConnectionFluentBuilder::catalog_id) / [`set_catalog_id(Option<String>)`](crate::operation::get_connection::builders::GetConnectionFluentBuilder::set_catalog_id):<br>required: **false**<br><p>The ID of the Data Catalog in which the connection resides. If none is provided, the Amazon Web Services account ID is used by default.</p><br>
    ///   - [`name(impl Into<String>)`](crate::operation::get_connection::builders::GetConnectionFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::get_connection::builders::GetConnectionFluentBuilder::set_name):<br>required: **true**<br><p>The name of the connection definition to retrieve.</p><br>
    ///   - [`hide_password(bool)`](crate::operation::get_connection::builders::GetConnectionFluentBuilder::hide_password) / [`set_hide_password(Option<bool>)`](crate::operation::get_connection::builders::GetConnectionFluentBuilder::set_hide_password):<br>required: **false**<br><p>Allows you to retrieve the connection metadata without returning the password. For instance, the Glue console uses this flag to retrieve the connection, and does not display the password. Set this parameter when the caller might not have permission to use the KMS key to decrypt the password, but it does have permission to access the rest of the connection properties.</p><br>
    ///   - [`apply_override_for_compute_environment(ComputeEnvironment)`](crate::operation::get_connection::builders::GetConnectionFluentBuilder::apply_override_for_compute_environment) / [`set_apply_override_for_compute_environment(Option<ComputeEnvironment>)`](crate::operation::get_connection::builders::GetConnectionFluentBuilder::set_apply_override_for_compute_environment):<br>required: **false**<br><p>For connections that may be used in multiple services, specifies returning properties for the specified compute environment.</p><br>
    /// - On success, responds with [`GetConnectionOutput`](crate::operation::get_connection::GetConnectionOutput) with field(s):
    ///   - [`connection(Option<Connection>)`](crate::operation::get_connection::GetConnectionOutput::connection): <p>The requested connection definition.</p>
    /// - On failure, responds with [`SdkError<GetConnectionError>`](crate::operation::get_connection::GetConnectionError)
    pub fn get_connection(&self) -> crate::operation::get_connection::builders::GetConnectionFluentBuilder {
        crate::operation::get_connection::builders::GetConnectionFluentBuilder::new(self.handle.clone())
    }
}
