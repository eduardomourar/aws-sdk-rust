// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateHttpNamespace`](crate::operation::update_http_namespace::builders::UpdateHttpNamespaceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`id(impl Into<String>)`](crate::operation::update_http_namespace::builders::UpdateHttpNamespaceFluentBuilder::id) / [`set_id(Option<String>)`](crate::operation::update_http_namespace::builders::UpdateHttpNamespaceFluentBuilder::set_id):<br>required: **true**<br><p>The ID of the namespace that you want to update.</p><br>
    ///   - [`updater_request_id(impl Into<String>)`](crate::operation::update_http_namespace::builders::UpdateHttpNamespaceFluentBuilder::updater_request_id) / [`set_updater_request_id(Option<String>)`](crate::operation::update_http_namespace::builders::UpdateHttpNamespaceFluentBuilder::set_updater_request_id):<br>required: **false**<br><p>A unique string that identifies the request and that allows failed <code>UpdateHttpNamespace</code> requests to be retried without the risk of running the operation twice. <code>UpdaterRequestId</code> can be any unique string (for example, a date/timestamp).</p><br>
    ///   - [`namespace(HttpNamespaceChange)`](crate::operation::update_http_namespace::builders::UpdateHttpNamespaceFluentBuilder::namespace) / [`set_namespace(Option<HttpNamespaceChange>)`](crate::operation::update_http_namespace::builders::UpdateHttpNamespaceFluentBuilder::set_namespace):<br>required: **true**<br><p>Updated properties for the the HTTP namespace.</p><br>
    /// - On success, responds with [`UpdateHttpNamespaceOutput`](crate::operation::update_http_namespace::UpdateHttpNamespaceOutput) with field(s):
    ///   - [`operation_id(Option<String>)`](crate::operation::update_http_namespace::UpdateHttpNamespaceOutput::operation_id): <p>A value that you can use to determine whether the request completed successfully. To get the status of the operation, see <a href="https://docs.aws.amazon.com/cloud-map/latest/api/API_GetOperation.html">GetOperation</a>.</p>
    /// - On failure, responds with [`SdkError<UpdateHttpNamespaceError>`](crate::operation::update_http_namespace::UpdateHttpNamespaceError)
    pub fn update_http_namespace(&self) -> crate::operation::update_http_namespace::builders::UpdateHttpNamespaceFluentBuilder {
        crate::operation::update_http_namespace::builders::UpdateHttpNamespaceFluentBuilder::new(self.handle.clone())
    }
}
