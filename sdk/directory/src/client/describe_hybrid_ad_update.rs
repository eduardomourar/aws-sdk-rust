// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeHybridADUpdate`](crate::operation::describe_hybrid_ad_update::builders::DescribeHybridADUpdateFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`directory_id(impl Into<String>)`](crate::operation::describe_hybrid_ad_update::builders::DescribeHybridADUpdateFluentBuilder::directory_id) / [`set_directory_id(Option<String>)`](crate::operation::describe_hybrid_ad_update::builders::DescribeHybridADUpdateFluentBuilder::set_directory_id):<br>required: **true**<br><p>The identifier of the hybrid directory for which to retrieve update information.</p><br>
    ///   - [`update_type(HybridUpdateType)`](crate::operation::describe_hybrid_ad_update::builders::DescribeHybridADUpdateFluentBuilder::update_type) / [`set_update_type(Option<HybridUpdateType>)`](crate::operation::describe_hybrid_ad_update::builders::DescribeHybridADUpdateFluentBuilder::set_update_type):<br>required: **false**<br><p>The type of update activities to retrieve. Valid values include <code>SelfManagedInstances</code> and <code>HybridAdministratorAccount</code>.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_hybrid_ad_update::builders::DescribeHybridADUpdateFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_hybrid_ad_update::builders::DescribeHybridADUpdateFluentBuilder::set_next_token):<br>required: **false**<br><p>The pagination token from a previous request to <code>DescribeHybridADUpdate</code>. Pass null if this is the first request.</p><br>
    /// - On success, responds with [`DescribeHybridAdUpdateOutput`](crate::operation::describe_hybrid_ad_update::DescribeHybridAdUpdateOutput) with field(s):
    ///   - [`update_activities(Option<HybridUpdateActivities>)`](crate::operation::describe_hybrid_ad_update::DescribeHybridAdUpdateOutput::update_activities): <p>Information about update activities for the hybrid directory, organized by update type.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_hybrid_ad_update::DescribeHybridAdUpdateOutput::next_token): <p>If not null, more results are available. Pass this value for the <code>NextToken</code> parameter in a subsequent request to retrieve the next set of items.</p>
    /// - On failure, responds with [`SdkError<DescribeHybridADUpdateError>`](crate::operation::describe_hybrid_ad_update::DescribeHybridADUpdateError)
    pub fn describe_hybrid_ad_update(&self) -> crate::operation::describe_hybrid_ad_update::builders::DescribeHybridADUpdateFluentBuilder {
        crate::operation::describe_hybrid_ad_update::builders::DescribeHybridADUpdateFluentBuilder::new(self.handle.clone())
    }
}
