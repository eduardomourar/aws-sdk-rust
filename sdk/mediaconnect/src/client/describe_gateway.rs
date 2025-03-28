// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeGateway`](crate::operation::describe_gateway::builders::DescribeGatewayFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`gateway_arn(impl Into<String>)`](crate::operation::describe_gateway::builders::DescribeGatewayFluentBuilder::gateway_arn) / [`set_gateway_arn(Option<String>)`](crate::operation::describe_gateway::builders::DescribeGatewayFluentBuilder::set_gateway_arn):<br>required: **true**<br><p>The ARN of the gateway that you want to describe.</p><br>
    /// - On success, responds with [`DescribeGatewayOutput`](crate::operation::describe_gateway::DescribeGatewayOutput) with field(s):
    ///   - [`gateway(Option<Gateway>)`](crate::operation::describe_gateway::DescribeGatewayOutput::gateway): <p>The gateway that you wanted to describe.</p>
    /// - On failure, responds with [`SdkError<DescribeGatewayError>`](crate::operation::describe_gateway::DescribeGatewayError)
    pub fn describe_gateway(&self) -> crate::operation::describe_gateway::builders::DescribeGatewayFluentBuilder {
        crate::operation::describe_gateway::builders::DescribeGatewayFluentBuilder::new(self.handle.clone())
    }
}
