// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateRuleset`](crate::operation::update_ruleset::builders::UpdateRulesetFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::operation::update_ruleset::builders::UpdateRulesetFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_ruleset::builders::UpdateRulesetFluentBuilder::set_name):<br>required: **true**<br><p>The name of the ruleset to be updated.</p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::update_ruleset::builders::UpdateRulesetFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_ruleset::builders::UpdateRulesetFluentBuilder::set_description):<br>required: **false**<br><p>The description of the ruleset.</p><br>
    ///   - [`rules(Rule)`](crate::operation::update_ruleset::builders::UpdateRulesetFluentBuilder::rules) / [`set_rules(Option<Vec::<Rule>>)`](crate::operation::update_ruleset::builders::UpdateRulesetFluentBuilder::set_rules):<br>required: **true**<br><p>A list of rules that are defined with the ruleset. A rule includes one or more checks to be validated on a DataBrew dataset.</p><br>
    /// - On success, responds with [`UpdateRulesetOutput`](crate::operation::update_ruleset::UpdateRulesetOutput) with field(s):
    ///   - [`name(String)`](crate::operation::update_ruleset::UpdateRulesetOutput::name): <p>The name of the updated ruleset.</p>
    /// - On failure, responds with [`SdkError<UpdateRulesetError>`](crate::operation::update_ruleset::UpdateRulesetError)
    pub fn update_ruleset(&self) -> crate::operation::update_ruleset::builders::UpdateRulesetFluentBuilder {
        crate::operation::update_ruleset::builders::UpdateRulesetFluentBuilder::new(self.handle.clone())
    }
}
