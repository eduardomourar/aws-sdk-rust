// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetMessagingSessionEndpoint`](crate::operation::get_messaging_session_endpoint::builders::GetMessagingSessionEndpointFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::get_messaging_session_endpoint::builders::GetMessagingSessionEndpointFluentBuilder::send) it.
    /// - On success, responds with [`GetMessagingSessionEndpointOutput`](crate::operation::get_messaging_session_endpoint::GetMessagingSessionEndpointOutput) with field(s):
    ///   - [`endpoint(Option<MessagingSessionEndpoint>)`](crate::operation::get_messaging_session_endpoint::GetMessagingSessionEndpointOutput::endpoint): <p>The endpoint returned in the response.</p>
    /// - On failure, responds with [`SdkError<GetMessagingSessionEndpointError>`](crate::operation::get_messaging_session_endpoint::GetMessagingSessionEndpointError)
    #[deprecated(note = "Replaced by GetMessagingSessionEndpoint in the Amazon Chime SDK Messaging Namespace")]
    pub fn get_messaging_session_endpoint(
        &self,
    ) -> crate::operation::get_messaging_session_endpoint::builders::GetMessagingSessionEndpointFluentBuilder {
        crate::operation::get_messaging_session_endpoint::builders::GetMessagingSessionEndpointFluentBuilder::new(self.handle.clone())
    }
}
