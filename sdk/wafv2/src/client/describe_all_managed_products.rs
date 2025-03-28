// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeAllManagedProducts`](crate::operation::describe_all_managed_products::builders::DescribeAllManagedProductsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`scope(Scope)`](crate::operation::describe_all_managed_products::builders::DescribeAllManagedProductsFluentBuilder::scope) / [`set_scope(Option<Scope>)`](crate::operation::describe_all_managed_products::builders::DescribeAllManagedProductsFluentBuilder::set_scope):<br>required: **true**<br><p>Specifies whether this is for a global resource type, such as a Amazon CloudFront distribution. For an Amplify application, use <code>CLOUDFRONT</code>.</p> <p>To work with CloudFront, you must also specify the Region US East (N. Virginia) as follows:</p> <ul>  <li>   <p>CLI - Specify the Region when you use the CloudFront scope: <code>--scope=CLOUDFRONT --region=us-east-1</code>.</p></li>  <li>   <p>API and SDKs - For all calls, use the Region endpoint us-east-1.</p></li> </ul><br>
    /// - On success, responds with [`DescribeAllManagedProductsOutput`](crate::operation::describe_all_managed_products::DescribeAllManagedProductsOutput) with field(s):
    ///   - [`managed_products(Option<Vec::<ManagedProductDescriptor>>)`](crate::operation::describe_all_managed_products::DescribeAllManagedProductsOutput::managed_products): <p>High-level information for the Amazon Web Services Managed Rules rule groups and Amazon Web Services Marketplace managed rule groups.</p>
    /// - On failure, responds with [`SdkError<DescribeAllManagedProductsError>`](crate::operation::describe_all_managed_products::DescribeAllManagedProductsError)
    pub fn describe_all_managed_products(
        &self,
    ) -> crate::operation::describe_all_managed_products::builders::DescribeAllManagedProductsFluentBuilder {
        crate::operation::describe_all_managed_products::builders::DescribeAllManagedProductsFluentBuilder::new(self.handle.clone())
    }
}
