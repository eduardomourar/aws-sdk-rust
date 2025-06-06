// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListRoutingRules`](crate::operation::list_routing_rules::builders::ListRoutingRulesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_routing_rules::builders::ListRoutingRulesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`domain_name(impl Into<String>)`](crate::operation::list_routing_rules::builders::ListRoutingRulesFluentBuilder::domain_name) / [`set_domain_name(Option<String>)`](crate::operation::list_routing_rules::builders::ListRoutingRulesFluentBuilder::set_domain_name):<br>required: **true**<br><p>The domain name.</p><br>
    ///   - [`domain_name_id(impl Into<String>)`](crate::operation::list_routing_rules::builders::ListRoutingRulesFluentBuilder::domain_name_id) / [`set_domain_name_id(Option<String>)`](crate::operation::list_routing_rules::builders::ListRoutingRulesFluentBuilder::set_domain_name_id):<br>required: **false**<br><p>The domain name ID.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_routing_rules::builders::ListRoutingRulesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_routing_rules::builders::ListRoutingRulesFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of elements to be returned for this resource.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_routing_rules::builders::ListRoutingRulesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_routing_rules::builders::ListRoutingRulesFluentBuilder::set_next_token):<br>required: **false**<br><p>The next page of elements from this collection. Not valid for the last element of the collection.</p><br>
    /// - On success, responds with [`ListRoutingRulesOutput`](crate::operation::list_routing_rules::ListRoutingRulesOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::list_routing_rules::ListRoutingRulesOutput::next_token): <p>The next page of elements from this collection. Not valid for the last element of the collection.</p>
    ///   - [`routing_rules(Option<Vec::<RoutingRule>>)`](crate::operation::list_routing_rules::ListRoutingRulesOutput::routing_rules): <p>The routing rules.</p> <p></p>
    /// - On failure, responds with [`SdkError<ListRoutingRulesError>`](crate::operation::list_routing_rules::ListRoutingRulesError)
    pub fn list_routing_rules(&self) -> crate::operation::list_routing_rules::builders::ListRoutingRulesFluentBuilder {
        crate::operation::list_routing_rules::builders::ListRoutingRulesFluentBuilder::new(self.handle.clone())
    }
}
