// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeCompute`](crate::operation::describe_compute::builders::DescribeComputeFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`fleet_id(impl Into<String>)`](crate::operation::describe_compute::builders::DescribeComputeFluentBuilder::fleet_id) / [`set_fleet_id(Option<String>)`](crate::operation::describe_compute::builders::DescribeComputeFluentBuilder::set_fleet_id):<br>required: **true**<br><p>A unique identifier for the fleet that the compute belongs to. You can use either the fleet ID or ARN value.</p><br>
    ///   - [`compute_name(impl Into<String>)`](crate::operation::describe_compute::builders::DescribeComputeFluentBuilder::compute_name) / [`set_compute_name(Option<String>)`](crate::operation::describe_compute::builders::DescribeComputeFluentBuilder::set_compute_name):<br>required: **true**<br><p>The unique identifier of the compute resource to retrieve properties for. For a managed container fleet or Anywhere fleet, use a compute name. For an EC2 fleet, use an instance ID. To retrieve a fleet's compute identifiers, call <a href="https://docs.aws.amazon.com/gamelift/latest/apireference/API_ListCompute.html">ListCompute</a>.</p><br>
    /// - On success, responds with [`DescribeComputeOutput`](crate::operation::describe_compute::DescribeComputeOutput) with field(s):
    ///   - [`compute(Option<Compute>)`](crate::operation::describe_compute::DescribeComputeOutput::compute): <p>The set of properties for the requested compute resource.</p>
    /// - On failure, responds with [`SdkError<DescribeComputeError>`](crate::operation::describe_compute::DescribeComputeError)
    pub fn describe_compute(&self) -> crate::operation::describe_compute::builders::DescribeComputeFluentBuilder {
        crate::operation::describe_compute::builders::DescribeComputeFluentBuilder::new(self.handle.clone())
    }
}
