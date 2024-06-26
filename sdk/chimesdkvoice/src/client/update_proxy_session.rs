// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateProxySession`](crate::operation::update_proxy_session::builders::UpdateProxySessionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`voice_connector_id(impl Into<String>)`](crate::operation::update_proxy_session::builders::UpdateProxySessionFluentBuilder::voice_connector_id) / [`set_voice_connector_id(Option<String>)`](crate::operation::update_proxy_session::builders::UpdateProxySessionFluentBuilder::set_voice_connector_id):<br>required: **true**<br><p>The Voice Connector ID.</p><br>
    ///   - [`proxy_session_id(impl Into<String>)`](crate::operation::update_proxy_session::builders::UpdateProxySessionFluentBuilder::proxy_session_id) / [`set_proxy_session_id(Option<String>)`](crate::operation::update_proxy_session::builders::UpdateProxySessionFluentBuilder::set_proxy_session_id):<br>required: **true**<br><p>The proxy session ID.</p><br>
    ///   - [`capabilities(Capability)`](crate::operation::update_proxy_session::builders::UpdateProxySessionFluentBuilder::capabilities) / [`set_capabilities(Option<Vec::<Capability>>)`](crate::operation::update_proxy_session::builders::UpdateProxySessionFluentBuilder::set_capabilities):<br>required: **true**<br><p>The proxy session capabilities.</p><br>
    ///   - [`expiry_minutes(i32)`](crate::operation::update_proxy_session::builders::UpdateProxySessionFluentBuilder::expiry_minutes) / [`set_expiry_minutes(Option<i32>)`](crate::operation::update_proxy_session::builders::UpdateProxySessionFluentBuilder::set_expiry_minutes):<br>required: **false**<br><p>The number of minutes allowed for the proxy session.</p><br>
    /// - On success, responds with [`UpdateProxySessionOutput`](crate::operation::update_proxy_session::UpdateProxySessionOutput) with field(s):
    ///   - [`proxy_session(Option<ProxySession>)`](crate::operation::update_proxy_session::UpdateProxySessionOutput::proxy_session): <p>The updated proxy session details.</p>
    /// - On failure, responds with [`SdkError<UpdateProxySessionError>`](crate::operation::update_proxy_session::UpdateProxySessionError)
    pub fn update_proxy_session(&self) -> crate::operation::update_proxy_session::builders::UpdateProxySessionFluentBuilder {
        crate::operation::update_proxy_session::builders::UpdateProxySessionFluentBuilder::new(self.handle.clone())
    }
}
