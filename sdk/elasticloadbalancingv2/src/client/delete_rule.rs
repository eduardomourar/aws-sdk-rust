// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteRule`](crate::operation::delete_rule::builders::DeleteRuleFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`rule_arn(impl Into<String>)`](crate::operation::delete_rule::builders::DeleteRuleFluentBuilder::rule_arn) / [`set_rule_arn(Option<String>)`](crate::operation::delete_rule::builders::DeleteRuleFluentBuilder::set_rule_arn):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the rule.</p><br>
    /// - On success, responds with [`DeleteRuleOutput`](crate::operation::delete_rule::DeleteRuleOutput)
    /// - On failure, responds with [`SdkError<DeleteRuleError>`](crate::operation::delete_rule::DeleteRuleError)
    pub fn delete_rule(&self) -> crate::operation::delete_rule::builders::DeleteRuleFluentBuilder {
        crate::operation::delete_rule::builders::DeleteRuleFluentBuilder::new(self.handle.clone())
    }
}
