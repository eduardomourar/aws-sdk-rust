// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_telemetry_rules_for_organization::_list_telemetry_rules_for_organization_output::ListTelemetryRulesForOrganizationOutputBuilder;

pub use crate::operation::list_telemetry_rules_for_organization::_list_telemetry_rules_for_organization_input::ListTelemetryRulesForOrganizationInputBuilder;

impl crate::operation::list_telemetry_rules_for_organization::builders::ListTelemetryRulesForOrganizationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_telemetry_rules_for_organization::ListTelemetryRulesForOrganizationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_telemetry_rules_for_organization::ListTelemetryRulesForOrganizationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_telemetry_rules_for_organization();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListTelemetryRulesForOrganization`.
///
/// <p>Lists all telemetry rules in your organization. This operation can only be called by the organization's management account or a delegated administrator account.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListTelemetryRulesForOrganizationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_telemetry_rules_for_organization::builders::ListTelemetryRulesForOrganizationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_telemetry_rules_for_organization::ListTelemetryRulesForOrganizationOutput,
        crate::operation::list_telemetry_rules_for_organization::ListTelemetryRulesForOrganizationError,
    > for ListTelemetryRulesForOrganizationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_telemetry_rules_for_organization::ListTelemetryRulesForOrganizationOutput,
            crate::operation::list_telemetry_rules_for_organization::ListTelemetryRulesForOrganizationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListTelemetryRulesForOrganizationFluentBuilder {
    /// Creates a new `ListTelemetryRulesForOrganizationFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListTelemetryRulesForOrganization as a reference.
    pub fn as_input(&self) -> &crate::operation::list_telemetry_rules_for_organization::builders::ListTelemetryRulesForOrganizationInputBuilder {
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
        crate::operation::list_telemetry_rules_for_organization::ListTelemetryRulesForOrganizationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_telemetry_rules_for_organization::ListTelemetryRulesForOrganizationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_telemetry_rules_for_organization::ListTelemetryRulesForOrganization::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_telemetry_rules_for_organization::ListTelemetryRulesForOrganization::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_telemetry_rules_for_organization::ListTelemetryRulesForOrganizationOutput,
        crate::operation::list_telemetry_rules_for_organization::ListTelemetryRulesForOrganizationError,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_telemetry_rules_for_organization::paginator::ListTelemetryRulesForOrganizationPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_telemetry_rules_for_organization::paginator::ListTelemetryRulesForOrganizationPaginator {
        crate::operation::list_telemetry_rules_for_organization::paginator::ListTelemetryRulesForOrganizationPaginator::new(self.handle, self.inner)
    }
    /// <p>A string to filter organization telemetry rules whose names begin with the specified prefix.</p>
    pub fn rule_name_prefix(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.rule_name_prefix(input.into());
        self
    }
    /// <p>A string to filter organization telemetry rules whose names begin with the specified prefix.</p>
    pub fn set_rule_name_prefix(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_rule_name_prefix(input);
        self
    }
    /// <p>A string to filter organization telemetry rules whose names begin with the specified prefix.</p>
    pub fn get_rule_name_prefix(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_rule_name_prefix()
    }
    ///
    /// Appends an item to `SourceAccountIds`.
    ///
    /// To override the contents of this collection use [`set_source_account_ids`](Self::set_source_account_ids).
    ///
    /// <p>The list of account IDs to filter organization telemetry rules by their source accounts.</p>
    pub fn source_account_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.source_account_ids(input.into());
        self
    }
    /// <p>The list of account IDs to filter organization telemetry rules by their source accounts.</p>
    pub fn set_source_account_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_source_account_ids(input);
        self
    }
    /// <p>The list of account IDs to filter organization telemetry rules by their source accounts.</p>
    pub fn get_source_account_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_source_account_ids()
    }
    ///
    /// Appends an item to `SourceOrganizationUnitIds`.
    ///
    /// To override the contents of this collection use [`set_source_organization_unit_ids`](Self::set_source_organization_unit_ids).
    ///
    /// <p>The list of organizational unit IDs to filter organization telemetry rules by their source organizational units.</p>
    pub fn source_organization_unit_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.source_organization_unit_ids(input.into());
        self
    }
    /// <p>The list of organizational unit IDs to filter organization telemetry rules by their source organizational units.</p>
    pub fn set_source_organization_unit_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_source_organization_unit_ids(input);
        self
    }
    /// <p>The list of organizational unit IDs to filter organization telemetry rules by their source organizational units.</p>
    pub fn get_source_organization_unit_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_source_organization_unit_ids()
    }
    /// <p>The maximum number of organization telemetry rules to return in a single call.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of organization telemetry rules to return in a single call.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of organization telemetry rules to return in a single call.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>The token for the next set of results. A previous call generates this token.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token for the next set of results. A previous call generates this token.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token for the next set of results. A previous call generates this token.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
