// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`BatchCreateBillScenarioUsageModification`](crate::operation::batch_create_bill_scenario_usage_modification::builders::BatchCreateBillScenarioUsageModificationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bill_scenario_id(impl Into<String>)`](crate::operation::batch_create_bill_scenario_usage_modification::builders::BatchCreateBillScenarioUsageModificationFluentBuilder::bill_scenario_id) / [`set_bill_scenario_id(Option<String>)`](crate::operation::batch_create_bill_scenario_usage_modification::builders::BatchCreateBillScenarioUsageModificationFluentBuilder::set_bill_scenario_id):<br>required: **true**<br><p>The ID of the Bill Scenario for which you want to create the modeled usage.</p><br>
    ///   - [`usage_modifications(BatchCreateBillScenarioUsageModificationEntry)`](crate::operation::batch_create_bill_scenario_usage_modification::builders::BatchCreateBillScenarioUsageModificationFluentBuilder::usage_modifications) / [`set_usage_modifications(Option<Vec::<BatchCreateBillScenarioUsageModificationEntry>>)`](crate::operation::batch_create_bill_scenario_usage_modification::builders::BatchCreateBillScenarioUsageModificationFluentBuilder::set_usage_modifications):<br>required: **true**<br><p>List of usage that you want to model in the Bill Scenario.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::batch_create_bill_scenario_usage_modification::builders::BatchCreateBillScenarioUsageModificationFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::batch_create_bill_scenario_usage_modification::builders::BatchCreateBillScenarioUsageModificationFluentBuilder::set_client_token):<br>required: **false**<br><p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p><br>
    /// - On success, responds with [`BatchCreateBillScenarioUsageModificationOutput`](crate::operation::batch_create_bill_scenario_usage_modification::BatchCreateBillScenarioUsageModificationOutput) with field(s):
    ///   - [`items(Option<Vec::<BatchCreateBillScenarioUsageModificationItem>>)`](crate::operation::batch_create_bill_scenario_usage_modification::BatchCreateBillScenarioUsageModificationOutput::items): <p>Returns the list of successful usage line items that were created for the Bill Scenario.</p>
    ///   - [`errors(Option<Vec::<BatchCreateBillScenarioUsageModificationError>>)`](crate::operation::batch_create_bill_scenario_usage_modification::BatchCreateBillScenarioUsageModificationOutput::errors): <p>Returns the list of errors reason and the usage item keys that cannot be created in the Bill Scenario.</p>
    /// - On failure, responds with [`SdkError<BatchCreateBillScenarioUsageModificationError>`](crate::operation::batch_create_bill_scenario_usage_modification::BatchCreateBillScenarioUsageModificationError)
    pub fn batch_create_bill_scenario_usage_modification(
        &self,
    ) -> crate::operation::batch_create_bill_scenario_usage_modification::builders::BatchCreateBillScenarioUsageModificationFluentBuilder {
        crate::operation::batch_create_bill_scenario_usage_modification::builders::BatchCreateBillScenarioUsageModificationFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
