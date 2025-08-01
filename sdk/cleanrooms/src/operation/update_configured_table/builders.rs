// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_configured_table::_update_configured_table_output::UpdateConfiguredTableOutputBuilder;

pub use crate::operation::update_configured_table::_update_configured_table_input::UpdateConfiguredTableInputBuilder;

impl crate::operation::update_configured_table::builders::UpdateConfiguredTableInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_configured_table::UpdateConfiguredTableOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_configured_table::UpdateConfiguredTableError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_configured_table();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateConfiguredTable`.
///
/// <p>Updates a configured table.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateConfiguredTableFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_configured_table::builders::UpdateConfiguredTableInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_configured_table::UpdateConfiguredTableOutput,
        crate::operation::update_configured_table::UpdateConfiguredTableError,
    > for UpdateConfiguredTableFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_configured_table::UpdateConfiguredTableOutput,
            crate::operation::update_configured_table::UpdateConfiguredTableError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateConfiguredTableFluentBuilder {
    /// Creates a new `UpdateConfiguredTableFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateConfiguredTable as a reference.
    pub fn as_input(&self) -> &crate::operation::update_configured_table::builders::UpdateConfiguredTableInputBuilder {
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
        crate::operation::update_configured_table::UpdateConfiguredTableOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_configured_table::UpdateConfiguredTableError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_configured_table::UpdateConfiguredTable::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_configured_table::UpdateConfiguredTable::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_configured_table::UpdateConfiguredTableOutput,
        crate::operation::update_configured_table::UpdateConfiguredTableError,
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
    /// <p>The identifier for the configured table to update. Currently accepts the configured table ID.</p>
    pub fn configured_table_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.configured_table_identifier(input.into());
        self
    }
    /// <p>The identifier for the configured table to update. Currently accepts the configured table ID.</p>
    pub fn set_configured_table_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_configured_table_identifier(input);
        self
    }
    /// <p>The identifier for the configured table to update. Currently accepts the configured table ID.</p>
    pub fn get_configured_table_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_configured_table_identifier()
    }
    /// <p>A new name for the configured table.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>A new name for the configured table.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>A new name for the configured table.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>A new description for the configured table.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A new description for the configured table.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>A new description for the configured table.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>A pointer to the dataset that underlies this table.</p>
    pub fn table_reference(mut self, input: crate::types::TableReference) -> Self {
        self.inner = self.inner.table_reference(input);
        self
    }
    /// <p>A pointer to the dataset that underlies this table.</p>
    pub fn set_table_reference(mut self, input: ::std::option::Option<crate::types::TableReference>) -> Self {
        self.inner = self.inner.set_table_reference(input);
        self
    }
    /// <p>A pointer to the dataset that underlies this table.</p>
    pub fn get_table_reference(&self) -> &::std::option::Option<crate::types::TableReference> {
        self.inner.get_table_reference()
    }
    ///
    /// Appends an item to `allowedColumns`.
    ///
    /// To override the contents of this collection use [`set_allowed_columns`](Self::set_allowed_columns).
    ///
    /// <p>The columns of the underlying table that can be used by collaborations or analysis rules.</p>
    pub fn allowed_columns(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.allowed_columns(input.into());
        self
    }
    /// <p>The columns of the underlying table that can be used by collaborations or analysis rules.</p>
    pub fn set_allowed_columns(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_allowed_columns(input);
        self
    }
    /// <p>The columns of the underlying table that can be used by collaborations or analysis rules.</p>
    pub fn get_allowed_columns(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_allowed_columns()
    }
    /// <p>The analysis method for the configured table.</p>
    /// <p><code>DIRECT_QUERY</code> allows SQL queries to be run directly on this table.</p>
    /// <p><code>DIRECT_JOB</code> allows PySpark jobs to be run directly on this table.</p>
    /// <p><code>MULTIPLE</code> allows both SQL queries and PySpark jobs to be run directly on this table.</p>
    pub fn analysis_method(mut self, input: crate::types::AnalysisMethod) -> Self {
        self.inner = self.inner.analysis_method(input);
        self
    }
    /// <p>The analysis method for the configured table.</p>
    /// <p><code>DIRECT_QUERY</code> allows SQL queries to be run directly on this table.</p>
    /// <p><code>DIRECT_JOB</code> allows PySpark jobs to be run directly on this table.</p>
    /// <p><code>MULTIPLE</code> allows both SQL queries and PySpark jobs to be run directly on this table.</p>
    pub fn set_analysis_method(mut self, input: ::std::option::Option<crate::types::AnalysisMethod>) -> Self {
        self.inner = self.inner.set_analysis_method(input);
        self
    }
    /// <p>The analysis method for the configured table.</p>
    /// <p><code>DIRECT_QUERY</code> allows SQL queries to be run directly on this table.</p>
    /// <p><code>DIRECT_JOB</code> allows PySpark jobs to be run directly on this table.</p>
    /// <p><code>MULTIPLE</code> allows both SQL queries and PySpark jobs to be run directly on this table.</p>
    pub fn get_analysis_method(&self) -> &::std::option::Option<crate::types::AnalysisMethod> {
        self.inner.get_analysis_method()
    }
    ///
    /// Appends an item to `selectedAnalysisMethods`.
    ///
    /// To override the contents of this collection use [`set_selected_analysis_methods`](Self::set_selected_analysis_methods).
    ///
    /// <p>The selected analysis methods for the table configuration update.</p>
    pub fn selected_analysis_methods(mut self, input: crate::types::SelectedAnalysisMethod) -> Self {
        self.inner = self.inner.selected_analysis_methods(input);
        self
    }
    /// <p>The selected analysis methods for the table configuration update.</p>
    pub fn set_selected_analysis_methods(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::SelectedAnalysisMethod>>) -> Self {
        self.inner = self.inner.set_selected_analysis_methods(input);
        self
    }
    /// <p>The selected analysis methods for the table configuration update.</p>
    pub fn get_selected_analysis_methods(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::SelectedAnalysisMethod>> {
        self.inner.get_selected_analysis_methods()
    }
}
