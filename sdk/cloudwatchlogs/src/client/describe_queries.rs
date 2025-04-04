// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeQueries`](crate::operation::describe_queries::builders::DescribeQueriesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`log_group_name(impl Into<String>)`](crate::operation::describe_queries::builders::DescribeQueriesFluentBuilder::log_group_name) / [`set_log_group_name(Option<String>)`](crate::operation::describe_queries::builders::DescribeQueriesFluentBuilder::set_log_group_name):<br>required: **false**<br><p>Limits the returned queries to only those for the specified log group.</p><br>
    ///   - [`status(QueryStatus)`](crate::operation::describe_queries::builders::DescribeQueriesFluentBuilder::status) / [`set_status(Option<QueryStatus>)`](crate::operation::describe_queries::builders::DescribeQueriesFluentBuilder::set_status):<br>required: **false**<br><p>Limits the returned queries to only those that have the specified status. Valid values are <code>Cancelled</code>, <code>Complete</code>, <code>Failed</code>, <code>Running</code>, and <code>Scheduled</code>.</p><br>
    ///   - [`max_results(i32)`](crate::operation::describe_queries::builders::DescribeQueriesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_queries::builders::DescribeQueriesFluentBuilder::set_max_results):<br>required: **false**<br><p>Limits the number of returned queries to the specified number.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_queries::builders::DescribeQueriesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_queries::builders::DescribeQueriesFluentBuilder::set_next_token):<br>required: **false**<br><p>The token for the next set of items to return. The token expires after 24 hours.</p><br>
    ///   - [`query_language(QueryLanguage)`](crate::operation::describe_queries::builders::DescribeQueriesFluentBuilder::query_language) / [`set_query_language(Option<QueryLanguage>)`](crate::operation::describe_queries::builders::DescribeQueriesFluentBuilder::set_query_language):<br>required: **false**<br><p>Limits the returned queries to only the queries that use the specified query language.</p><br>
    /// - On success, responds with [`DescribeQueriesOutput`](crate::operation::describe_queries::DescribeQueriesOutput) with field(s):
    ///   - [`queries(Option<Vec::<QueryInfo>>)`](crate::operation::describe_queries::DescribeQueriesOutput::queries): <p>The list of queries that match the request.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_queries::DescribeQueriesOutput::next_token): <p>The token for the next set of items to return. The token expires after 24 hours.</p>
    /// - On failure, responds with [`SdkError<DescribeQueriesError>`](crate::operation::describe_queries::DescribeQueriesError)
    pub fn describe_queries(&self) -> crate::operation::describe_queries::builders::DescribeQueriesFluentBuilder {
        crate::operation::describe_queries::builders::DescribeQueriesFluentBuilder::new(self.handle.clone())
    }
}
