// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::batch_delete_bill_scenario_usage_modification::_batch_delete_bill_scenario_usage_modification_output::BatchDeleteBillScenarioUsageModificationOutputBuilder;

pub use crate::operation::batch_delete_bill_scenario_usage_modification::_batch_delete_bill_scenario_usage_modification_input::BatchDeleteBillScenarioUsageModificationInputBuilder;

impl crate::operation::batch_delete_bill_scenario_usage_modification::builders::BatchDeleteBillScenarioUsageModificationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::batch_delete_bill_scenario_usage_modification::BatchDeleteBillScenarioUsageModificationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::batch_delete_bill_scenario_usage_modification::BatchDeleteBillScenarioUsageModificationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.batch_delete_bill_scenario_usage_modification();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `BatchDeleteBillScenarioUsageModification`.
///
/// <p>Delete usage that you have created in a Bill Scenario. You can only delete usage that you had added and cannot model deletion (or removal) of a existing usage. If you want model removal of an existing usage, see <a href="https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_AWSBCMPricingCalculator_BatchUpdateBillScenarioUsageModification.html"> BatchUpdateBillScenarioUsageModification</a>.</p><note>
/// <p>The <code>BatchDeleteBillScenarioUsageModification</code> operation doesn't have its own IAM permission. To authorize this operation for Amazon Web Services principals, include the permission <code>bcm-pricing-calculator:DeleteBillScenarioUsageModification</code> in your policies.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct BatchDeleteBillScenarioUsageModificationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::batch_delete_bill_scenario_usage_modification::builders::BatchDeleteBillScenarioUsageModificationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::batch_delete_bill_scenario_usage_modification::BatchDeleteBillScenarioUsageModificationOutput,
        crate::operation::batch_delete_bill_scenario_usage_modification::BatchDeleteBillScenarioUsageModificationError,
    > for BatchDeleteBillScenarioUsageModificationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::batch_delete_bill_scenario_usage_modification::BatchDeleteBillScenarioUsageModificationOutput,
            crate::operation::batch_delete_bill_scenario_usage_modification::BatchDeleteBillScenarioUsageModificationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl BatchDeleteBillScenarioUsageModificationFluentBuilder {
    /// Creates a new `BatchDeleteBillScenarioUsageModificationFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the BatchDeleteBillScenarioUsageModification as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::batch_delete_bill_scenario_usage_modification::builders::BatchDeleteBillScenarioUsageModificationInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::batch_delete_bill_scenario_usage_modification::BatchDeleteBillScenarioUsageModificationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::batch_delete_bill_scenario_usage_modification::BatchDeleteBillScenarioUsageModificationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::batch_delete_bill_scenario_usage_modification::BatchDeleteBillScenarioUsageModification::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::batch_delete_bill_scenario_usage_modification::BatchDeleteBillScenarioUsageModification::orchestrate(
            &runtime_plugins,
            input,
        )
        .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::batch_delete_bill_scenario_usage_modification::BatchDeleteBillScenarioUsageModificationOutput,
        crate::operation::batch_delete_bill_scenario_usage_modification::BatchDeleteBillScenarioUsageModificationError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
        self.set_config_override(::std::option::Option::Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The ID of the Bill Scenario for which you want to delete the modeled usage.</p>
    pub fn bill_scenario_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bill_scenario_id(input.into());
        self
    }
    /// <p>The ID of the Bill Scenario for which you want to delete the modeled usage.</p>
    pub fn set_bill_scenario_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bill_scenario_id(input);
        self
    }
    /// <p>The ID of the Bill Scenario for which you want to delete the modeled usage.</p>
    pub fn get_bill_scenario_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bill_scenario_id()
    }
    ///
    /// Appends an item to `ids`.
    ///
    /// To override the contents of this collection use [`set_ids`](Self::set_ids).
    ///
    /// <p>List of usage that you want to delete from the Bill Scenario.</p>
    pub fn ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.ids(input.into());
        self
    }
    /// <p>List of usage that you want to delete from the Bill Scenario.</p>
    pub fn set_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_ids(input);
        self
    }
    /// <p>List of usage that you want to delete from the Bill Scenario.</p>
    pub fn get_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_ids()
    }
}
