// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeRules`](crate::operation::describe_rules::builders::DescribeRulesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_rules::builders::DescribeRulesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`listener_arn(impl Into<String>)`](crate::operation::describe_rules::builders::DescribeRulesFluentBuilder::listener_arn) / [`set_listener_arn(Option<String>)`](crate::operation::describe_rules::builders::DescribeRulesFluentBuilder::set_listener_arn):<br>required: **false**<br><p>The Amazon Resource Name (ARN) of the listener.</p><br>
    ///   - [`rule_arns(impl Into<String>)`](crate::operation::describe_rules::builders::DescribeRulesFluentBuilder::rule_arns) / [`set_rule_arns(Option<Vec::<String>>)`](crate::operation::describe_rules::builders::DescribeRulesFluentBuilder::set_rule_arns):<br>required: **false**<br><p>The Amazon Resource Names (ARN) of the rules.</p><br>
    ///   - [`marker(impl Into<String>)`](crate::operation::describe_rules::builders::DescribeRulesFluentBuilder::marker) / [`set_marker(Option<String>)`](crate::operation::describe_rules::builders::DescribeRulesFluentBuilder::set_marker):<br>required: **false**<br><p>The marker for the next set of results. (You received this marker from a previous call.)</p><br>
    ///   - [`page_size(i32)`](crate::operation::describe_rules::builders::DescribeRulesFluentBuilder::page_size) / [`set_page_size(Option<i32>)`](crate::operation::describe_rules::builders::DescribeRulesFluentBuilder::set_page_size):<br>required: **false**<br><p>The maximum number of results to return with this call.</p><br>
    /// - On success, responds with [`DescribeRulesOutput`](crate::operation::describe_rules::DescribeRulesOutput) with field(s):
    ///   - [`rules(Option<Vec::<Rule>>)`](crate::operation::describe_rules::DescribeRulesOutput::rules): <p>Information about the rules.</p>
    ///   - [`next_marker(Option<String>)`](crate::operation::describe_rules::DescribeRulesOutput::next_marker): <p>If there are additional results, this is the marker for the next set of results. Otherwise, this is null.</p>
    /// - On failure, responds with [`SdkError<DescribeRulesError>`](crate::operation::describe_rules::DescribeRulesError)
    pub fn describe_rules(&self) -> crate::operation::describe_rules::builders::DescribeRulesFluentBuilder {
        crate::operation::describe_rules::builders::DescribeRulesFluentBuilder::new(self.handle.clone())
    }
}
