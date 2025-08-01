// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListOdbPeeringConnections`](crate::operation::list_odb_peering_connections::builders::ListOdbPeeringConnectionsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_odb_peering_connections::builders::ListOdbPeeringConnectionsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::operation::list_odb_peering_connections::builders::ListOdbPeeringConnectionsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_odb_peering_connections::builders::ListOdbPeeringConnectionsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of ODB peering connections to return in the response.</p> <p>Default: <code>20</code></p> <p>Constraints:</p> <ul>  <li>   <p>Must be between 1 and 100.</p></li> </ul><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_odb_peering_connections::builders::ListOdbPeeringConnectionsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_odb_peering_connections::builders::ListOdbPeeringConnectionsFluentBuilder::set_next_token):<br>required: **false**<br><p>The pagination token for the next page of ODB peering connections.</p><br>
    ///   - [`odb_network_id(impl Into<String>)`](crate::operation::list_odb_peering_connections::builders::ListOdbPeeringConnectionsFluentBuilder::odb_network_id) / [`set_odb_network_id(Option<String>)`](crate::operation::list_odb_peering_connections::builders::ListOdbPeeringConnectionsFluentBuilder::set_odb_network_id):<br>required: **false**<br><p>The identifier of the ODB network to list peering connections for.</p> <p>If not specified, lists all ODB peering connections in the account.</p><br>
    /// - On success, responds with [`ListOdbPeeringConnectionsOutput`](crate::operation::list_odb_peering_connections::ListOdbPeeringConnectionsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::list_odb_peering_connections::ListOdbPeeringConnectionsOutput::next_token): <p>The pagination token for the next page of ODB peering connections.</p>
    ///   - [`odb_peering_connections(Vec::<OdbPeeringConnectionSummary>)`](crate::operation::list_odb_peering_connections::ListOdbPeeringConnectionsOutput::odb_peering_connections): <p>The list of ODB peering connections.</p>
    /// - On failure, responds with [`SdkError<ListOdbPeeringConnectionsError>`](crate::operation::list_odb_peering_connections::ListOdbPeeringConnectionsError)
    pub fn list_odb_peering_connections(&self) -> crate::operation::list_odb_peering_connections::builders::ListOdbPeeringConnectionsFluentBuilder {
        crate::operation::list_odb_peering_connections::builders::ListOdbPeeringConnectionsFluentBuilder::new(self.handle.clone())
    }
}
