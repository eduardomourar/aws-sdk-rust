// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListRooms`](crate::operation::list_rooms::builders::ListRoomsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_rooms::builders::ListRoomsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::operation::list_rooms::builders::ListRoomsFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::list_rooms::builders::ListRoomsFluentBuilder::set_name):<br>required: **false**<br><p>Filters the list to match the specified room name.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_rooms::builders::ListRoomsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_rooms::builders::ListRoomsFluentBuilder::set_next_token):<br>required: **false**<br><p>The first room to retrieve. This is used for pagination; see the <code>nextToken</code> response field.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_rooms::builders::ListRoomsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_rooms::builders::ListRoomsFluentBuilder::set_max_results):<br>required: **false**<br><p>Maximum number of rooms to return. Default: 50.</p><br>
    ///   - [`message_review_handler_uri(impl Into<String>)`](crate::operation::list_rooms::builders::ListRoomsFluentBuilder::message_review_handler_uri) / [`set_message_review_handler_uri(Option<String>)`](crate::operation::list_rooms::builders::ListRoomsFluentBuilder::set_message_review_handler_uri):<br>required: **false**<br><p>Filters the list to match the specified message review handler URI.</p><br>
    ///   - [`logging_configuration_identifier(impl Into<String>)`](crate::operation::list_rooms::builders::ListRoomsFluentBuilder::logging_configuration_identifier) / [`set_logging_configuration_identifier(Option<String>)`](crate::operation::list_rooms::builders::ListRoomsFluentBuilder::set_logging_configuration_identifier):<br>required: **false**<br><p>Logging-configuration identifier.</p><br>
    /// - On success, responds with [`ListRoomsOutput`](crate::operation::list_rooms::ListRoomsOutput) with field(s):
    ///   - [`rooms(Vec::<RoomSummary>)`](crate::operation::list_rooms::ListRoomsOutput::rooms): <p>List of the matching rooms (summary information only).</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_rooms::ListRoomsOutput::next_token): <p>If there are more rooms than <code>maxResults</code>, use <code>nextToken</code> in the request to get the next set.</p>
    /// - On failure, responds with [`SdkError<ListRoomsError>`](crate::operation::list_rooms::ListRoomsError)
    pub fn list_rooms(&self) -> crate::operation::list_rooms::builders::ListRoomsFluentBuilder {
        crate::operation::list_rooms::builders::ListRoomsFluentBuilder::new(self.handle.clone())
    }
}
